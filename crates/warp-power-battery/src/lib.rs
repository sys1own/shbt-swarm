//! `warp-power-battery`: Dual-Reactor Hybrid Power Architecture (`SHBT-MAR-312X` + `SHBT-MAR-OMEGA`).
//!
//! 1. **SHBT-MAR-312X**: 25.00 EW discrete core (376.99 kg bipartite wafers, 5-zone Weyl DEC-X duct,
//!    48.0 T clathrate superhydride coils) for deep-space traversal, warp soliton generation, and
//!    initial siphon ignition.
//! 2. **SHBT-MAR-OMEGA**: 1.00–10.00 ZW continuous-flow boundary annihilation reactor (`mar_omega_flow`)
//!    for stellar mass extraction, degenerate core deconstruction, and continuous in-line computronium folding.
//! 3. **Dual-Reactor Power Bus Arbiter**: Hardware state machine (`power_arbiter`) coordinating zero-sag
//!    handoffs between `SHBT-MAR-312X` and `SHBT-MAR-OMEGA`.

#![no_std]

pub mod divertor_rmhd;
pub mod lindblad_unspool;
pub mod mar_omega_flow;
pub mod power_arbiter;

pub use divertor_rmhd::*;
pub use lindblad_unspool::*;
pub use mar_omega_flow::*;
pub use power_arbiter::*;
pub use shbt_swarm_mar312x::*;
