//! crates/shbt-swarm-bio/src/lyapunov_reactor.rs
//!
//! Non-Equilibrium Biogeochemical Lyapunov Reactor & Boundary Controller
//! Zero-allocation,  finite-volume implementation for SHBT-ELYSIUM.



/// Number of primary tracked biogeochemical species.
pub const NUM_SPECIES: usize = 32;

/// Maximum finite-volume discretization cells in local compute tile.
pub const MAX_CELLS: usize = 64;

/// Maximum boundary interface facets per tile.
pub const MAX_BOUNDARY_FACETS: usize = 24;

/// Reference homeostatic target concentrations C_k^* (mol / m^3).
pub static TARGET_CONCENTRATIONS: [f64; NUM_SPECIES] = [
    9.38e0,   //  0: O2
    1.78e-2,  //  1: CO2
    3.48e1,   //  2: N2
    8.05e-1,  //  3: H2O(v)
    7.50e-5,  //  4: CH4
    1.20e-5,  //  5: NH3
    5.00e-6,  //  6: H2S
    4.15e-1,  //  7: Ar
    2.30e0,   //  8: HCO3-
    2.50e-1,  //  9: CO3(2-)
    1.50e-2,  // 10: H2CO3
    3.10e-2,  // 11: NO3-
    1.00e-4,  // 12: NO2-
    5.00e-3,  // 13: NH4+
    5.00e-5,  // 14: PO4(3-)
    1.20e-3,  // 15: HPO4(2-)
    7.40e-4,  // 16: H2PO4-
    2.80e1,   // 17: SO4(2-)
    1.00e-5,  // 18: HS-
    1.02e1,   // 19: K+
    1.03e1,   // 20: Ca(2+)
    5.30e1,   // 21: Mg(2+)
    4.68e2,   // 22: Na+
    5.45e2,   // 23: Cl-
    1.00e-4,  // 24: Fe(2+)
    1.00e-6,  // 25: Fe(3+)
    2.00e-5,  // 26: Mn(2+)
    5.00e1,   // 27: C_auto (Autotrophic biomass)
    5.00e0,   // 28: C_hetero (Consumer biomass)
    2.50e0,   // 29: C_micro (Decomposer biomass)
    1.50e-1,  // 30: DOM (Dissolved organic matter)
    8.00e-1,  // 31: POM (Particulate organic matter)
];

/// Molecular and effective diffusivities D_k (m^2 / s).
pub static DIFFUSION_COEFFICIENTS: [f64; NUM_SPECIES] = [
    2.10e-5, 1.60e-5, 2.00e-5, 2.40e-5, 2.20e-5, 2.30e-5, 1.70e-5, 1.90e-5, // 0-7: Gases
    1.18e-9, 9.55e-10, 1.25e-9, 1.90e-9, 1.91e-9, 1.96e-9, 6.12e-10, 7.59e-10, // 8-15: Dissolved
    8.79e-10, 1.06e-9, 1.73e-9, 1.96e-9, 7.92e-10, 7.06e-10, 1.33e-9, 2.03e-9, // 16-23: Electrolytes
    7.19e-10, 6.04e-10, 7.12e-10, 1.00e-11, 1.00e-12, 5.00e-12, 5.00e-10, 2.00e-11, // 24-31: Catalysts/Bio
];

/// Active boundary Robin conductance rates kappa_k (m / s).
pub static BOUNDARY_CONDUCTANCES: [f64; NUM_SPECIES] = [
    1.50e-2, 1.50e-2, 1.00e-2, 2.00e-2, 1.00e-2, 1.00e-2, 1.00e-2, 5.00e-3, // 0-7
    5.00e-3, 5.00e-3, 5.00e-3, 1.00e-2, 1.00e-2, 1.00e-2, 2.00e-2, 2.00e-2, // 8-15
    2.00e-2, 1.00e-2, 1.00e-2, 5.00e-3, 5.00e-3, 5.00e-3, 5.00e-3, 5.00e-3, // 16-23
    5.00e-3, 5.00e-3, 5.00e-3, 1.00e-3, 1.00e-3, 1.00e-3, 5.00e-3, 5.00e-3, // 24-31
];

/// Biogeochemical state across the 32 species.
#[derive(Clone, Copy)]
pub struct SpeciesVector {
    pub concentrations: [f64; NUM_SPECIES],
}

impl SpeciesVector {
    pub const fn new_zero() -> Self {
        Self {
            concentrations: [0.0; NUM_SPECIES],
        }
    }
    pub const fn new_target() -> Self {
        Self {
            concentrations: TARGET_CONCENTRATIONS,
        }
    }
}

/// Boundary actuator command representing active matter translocator currents.
#[derive(Clone, Copy)]
pub struct TranslocatorCommand {
    pub facet_id: usize,
    pub species_id: usize,
    /// Flux in mol / (m^2 * s); positive = injection, negative = extraction.
    pub flux_mol_m2_s: f64,
}

/// Planar interior face between adjacent cells.
#[derive(Clone, Copy)]
pub struct InteriorFace {
    pub cell_left: usize,
    pub cell_right: usize,
    pub area_m2: f64,
    pub distance_m: f64,
    pub normal_advection_velocity_m_s: f64,
}

/// Boundary facet abutting the nano-porous boundary interface.
#[derive(Clone, Copy)]
pub struct BoundaryFacet {
    pub cell_index: usize,
    pub area_m2: f64,
}

/// Finite-volume mesh tile.
pub struct TileMesh {
    pub num_cells: usize,
    pub num_faces: usize,
    pub num_boundary_facets: usize,
    pub volumes: [f64; MAX_CELLS],
    pub faces: [InteriorFace; MAX_CELLS * 3],
    pub boundary_facets: [BoundaryFacet; MAX_BOUNDARY_FACETS],
}
/// The biogeochemical reactor instance.
pub struct LyapunovReactor {
    pub mesh: TileMesh,
    pub state: [SpeciesVector; MAX_CELLS],
    pub commands: [TranslocatorCommand; MAX_BOUNDARY_FACETS * NUM_SPECIES],
    pub num_active_commands: usize,
}

impl LyapunovReactor {
    /// Initializes a reactor instance at the homeostatic steady state.
    pub fn new(mesh: TileMesh) -> Self {
        let dummy_cmd = TranslocatorCommand {
            facet_id: 0,
            species_id: 0,
            flux_mol_m2_s: 0.0,
        };

        Self {
            mesh,
            state: [SpeciesVector::new_target(); MAX_CELLS],
            commands: [dummy_cmd; MAX_BOUNDARY_FACETS * NUM_SPECIES],
            num_active_commands: 0,
        }
    }

    /// Evaluates the discrete global relative-entropy Lyapunov functional V[C].
    pub fn compute_lyapunov_functional(&self) -> f64 {
        let mut total_v = 0.0;
        let mut i = 0;
        while i < self.mesh.num_cells {
            let vol = self.mesh.volumes[i];
            let cell_state = &self.state[i];
            let mut k = 0;
            while k < NUM_SPECIES {
                let c_k = cell_state.concentrations[k];
                let c_star = TARGET_CONCENTRATIONS[k];
                if c_k > 1e-15 && c_star > 1e-15 {
                    let ratio = c_k / c_star;
                    let psi = c_k * log_pade_approx(ratio) - (c_k - c_star);
                    if psi > 0.0 {
                        total_v += vol * psi;
                    }
                }
                k += 1;
            }
            i += 1;
        }
        total_v
    }

    /// Computes internal Redfield reaction rates R_k(C).
    pub fn compute_reaction_rates(cell_state: &SpeciesVector, rates: &mut [f64;
NUM_SPECIES]) {
        let mut k = 0;
        while k < NUM_SPECIES {
            rates[k] = 0.0;
            k += 1;
        }

        let o2 = cell_state.concentrations[0];
        let co2 = cell_state.concentrations[1];
        let no3 = cell_state.concentrations[11];
        let hpo4 = cell_state.concentrations[15];
        let _so4 = cell_state.concentrations[17];
        let c_auto = cell_state.concentrations[27];
        let c_hetero = cell_state.concentrations[28];
        let c_micro = cell_state.concentrations[29];
        let dom = cell_state.concentrations[30];

        // Autotrophic Primary Fixation (Redfield Monod limitation)
        let k_co2 = 5.0e-3;
        let k_no3 = 5.0e-3;
        let k_p = 1.0e-4;
        let mu_max = 1.0e-4; // 1/s
        let limitation = (co2 / (co2 + k_co2))
            * (no3 / (no3 + k_no3))
            * (hpo4 / (hpo4 + k_p));
        let phi_auto = mu_max * limitation * c_auto;

        rates[1] -= 106.0 * phi_auto;  // CO2 uptake
        rates[11] -= 16.0 * phi_auto;  // NO3- uptake
        rates[15] -= 1.0 * phi_auto;   // HPO4(2-) uptake
        rates[17] -= 0.7 * phi_auto;   // SO4(2-) uptake
        rates[0] += 138.7 * phi_auto;  // O2 generation
        rates[27] += phi_auto;         // Biomass growth

        // Heterotrophic Respiration
        let mu_grazing = 2.0e-5;
        let k_o2 = 1.0e-2;
        let phi_hetero = mu_grazing * (o2 / (o2 + k_o2)) * c_hetero;
        rates[27] -= 1.2 * phi_hetero; // Herbivory
        rates[28] += 0.4 * phi_hetero; // Consumer assimilation
        rates[30] += 0.8 * phi_hetero; // DOM excretion
        rates[0] -= 1.0 * phi_hetero;  // O2 respiration
        rates[1] += 1.0 * phi_hetero;  // CO2 respiration

        // Saprophytic Microbial Remineralization
        let mu_micro = 3.0e-5;
        let phi_micro = mu_micro * (dom / (dom + 1.0e-3)) * c_micro;
        rates[30] -= phi_micro;        // DOM uptake
        rates[29] += 0.3 * phi_micro;  // Microbe growth
        rates[1] += 0.7 * phi_micro;   // CO2 mineralization
        rates[15] += 0.007 * phi_micro;// Orthophosphate return
        rates[11] += 0.11 * phi_micro; // Nitrate return
    }

    /// Advances the state by time step dt using a conservative finite-volume scheme.
    pub fn advance_step(&mut self, dt: f64) {
        let mut d_c_dt = [[0.0; NUM_SPECIES]; MAX_CELLS];
        self.num_active_commands = 0;

        // 1. Interior face fluxes: conservative diffusion and upwind advection
        let mut f = 0;
        while f < self.mesh.num_faces {
            let face = self.mesh.faces[f];
            let c_l = face.cell_left;
            let c_r = face.cell_right;
            let inv_d = 1.0 / face.distance_m;
            let vel = face.normal_advection_velocity_m_s;

            let mut k = 0;
            while k < NUM_SPECIES {
                let d_k = DIFFUSION_COEFFICIENTS[k];
                let val_l = self.state[c_l].concentrations[k];
                let val_r = self.state[c_r].concentrations[k];

                let diff_flux = -d_k * (val_r - val_l) * inv_d;
                let upwind_val = if vel >= 0.0 { val_l } else { val_r };
                let adv_flux = vel * upwind_val;

                let face_flux_mols = (diff_flux + adv_flux) * face.area_m2;
                d_c_dt[c_l][k] -= face_flux_mols;
                d_c_dt[c_r][k] += face_flux_mols;

                k += 1;
            }
            f += 1;
        }

        // 2. Active Robin boundary current: J = -kappa * (C - C^*)
        let mut b = 0;
        while b < self.mesh.num_boundary_facets {
            let facet = self.mesh.boundary_facets[b];
            let cell = facet.cell_index;

            let mut k = 0;
            while k < NUM_SPECIES {
                let kappa = BOUNDARY_CONDUCTANCES[k];
                let c_val = self.state[cell].concentrations[k];
                let c_star = TARGET_CONCENTRATIONS[k];

                let balancing_flux = -kappa * (c_val - c_star);
                let surface_current = balancing_flux * facet.area_m2;
                d_c_dt[cell][k] += surface_current;

                if self.num_active_commands < (MAX_BOUNDARY_FACETS * NUM_SPECIES) {
                    self.commands[self.num_active_commands] = TranslocatorCommand {
                        facet_id: b,
                        species_id: k,
                        flux_mol_m2_s: balancing_flux,
                    };
                    self.num_active_commands += 1;
                }
                k += 1;
            }
            b += 1;
        }

        // 3. Cell integration and non-linear reactions
        let mut target_reactions = [0.0; NUM_SPECIES];
        let target_state = SpeciesVector::new_target();
        Self::compute_reaction_rates(&target_state, &mut target_reactions);

        let mut i = 0;
        while i < self.mesh.num_cells {
            let inv_vol = 1.0 / self.mesh.volumes[i];
            let mut reactions = [0.0; NUM_SPECIES];
            Self::compute_reaction_rates(&self.state[i], &mut reactions);

            let mut k = 0;
            while k < NUM_SPECIES {
                // Net stoichiometric rate relative to homeostatic baseline: R(C) - R(C^*)
                let net_reaction = reactions[k] - target_reactions[k];
                let rate = (d_c_dt[i][k] * inv_vol) + net_reaction;
                let mut new_c = self.state[i].concentrations[k] + dt * rate;
                if new_c < 1.0e-12 {
                    new_c = 1.0e-12; // Maintain strict non-negativity
                }
                self.state[i].concentrations[k] = new_c;
                k += 1;
            }
            i += 1;
        }
    }
}

/// Zero-allocation rational approximation of ln(x) using a [2/2] Padé expansion.
#[inline(always)]
pub fn log_pade_approx(x: f64) -> f64 {
    if x <= 0.0 {
        return -100.0;
    }
    let z = (x - 1.0) / (x + 1.0);
    let z2 = z * z;
    2.0 * z * (1.0 + z2 * (1.0 / 3.0 + z2 * (1.0 / 5.0 + z2 * (1.0 / 7.0))))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[inline]
    fn f64_abs(x: f64) -> f64 {
        if x < 0.0 { -x } else { x }
    }

    #[test]
    fn test_log_pade_approx_accuracy() {
        assert_eq!(log_pade_approx(1.0), 0.0);
        let val_1p1 = log_pade_approx(1.1);
        assert!(f64_abs(val_1p1 - 0.09531018) < 1e-4);
        let val_0p9 = log_pade_approx(0.9);
        assert!(f64_abs(val_0p9 - (-0.1053605)) < 1e-4);
    }

    #[test]
    fn test_species_initialization_and_steady_lyapunov() {
        let dummy_mesh = TileMesh {
            num_cells: 2,
            num_faces: 1,
            num_boundary_facets: 2,
            volumes: [100.0; MAX_CELLS],
            faces: [InteriorFace {
                cell_left: 0,
                cell_right: 1,
                area_m2: 10.0,
                distance_m: 1.0,
                normal_advection_velocity_m_s: 0.0,
            }; MAX_CELLS * 3],
            boundary_facets: [BoundaryFacet {
                cell_index: 0,
                area_m2: 5.0,
            }; MAX_BOUNDARY_FACETS],
        };

        let reactor = LyapunovReactor::new(dummy_mesh);
        let v0 = reactor.compute_lyapunov_functional();
        assert!(v0 < 1e-12);
    }

    #[test]
    fn test_perturbation_and_lyapunov_exponential_decay() {
        let mut facets = [BoundaryFacet {
            cell_index: 0,
            area_m2: 5.0,
        }; MAX_BOUNDARY_FACETS];
        facets[1].cell_index = 1;

        let dummy_mesh = TileMesh {
            num_cells: 2,
            num_faces: 1,
            num_boundary_facets: 2,
            volumes: [100.0; MAX_CELLS],
            faces: [InteriorFace {
                cell_left: 0,
                cell_right: 1,
                area_m2: 10.0,
                distance_m: 1.0,
                normal_advection_velocity_m_s: 0.0,
            }; MAX_CELLS * 3],
            boundary_facets: facets,
        };

        let mut reactor = LyapunovReactor::new(dummy_mesh);
        reactor.state[0].concentrations[0] *= 1.20;
        reactor.state[0].concentrations[1] *= 0.85;

        let v_init = reactor.compute_lyapunov_functional();
        assert!(v_init > 0.0);

        for _ in 0..10 {
            reactor.advance_step(0.1);
        }

        let v_final = reactor.compute_lyapunov_functional();
        assert!(v_final < v_init);
        assert!(reactor.num_active_commands > 0);
    }

    #[test]
    fn test_reaction_rates_consistency() {
        let state = SpeciesVector::new_target();
        let mut rates = [0.0; NUM_SPECIES];
        LyapunovReactor::compute_reaction_rates(&state, &mut rates);
        assert!(rates[0] != 0.0 || rates[1] != 0.0);
    }
}
