//! `crates/shbt-swarm-cli/src/main.rs`
//! Master CLI tool for sys1own/shbt-swarm verification and gate audits.
//! Configured with Kardashev Type III Galactic Synthesis as the primary default operational mode.

use shbt_swarm_hil::evaluate_all_160_gates;
use shbt_swarm_sim::{GalacticSynthesisSimulation, SwarmOperationalMode};

fn main() -> std::process::ExitCode {
    let mut args = std::env::args().skip(1);
    match args.next().as_deref() {
        Some("verify") | None => {
            println!("========================================================================");
            println!("SHBT-SWARM MASTER GATE VERIFICATION MATRIX (160 GATES)");
            println!("Primary Operational Mode: Kardashev Type III Galactic Synthesis & Phase 6 Observer Ascension");
            println!("========================================================================");

            // Verify primary galactic synthesis digital twin invariants
            let sim = GalacticSynthesisSimulation::default();
            assert_eq!(sim.mode, SwarmOperationalMode::GalacticSynthesis);
            if let Err(e) = sim.verify_galactic_invariants() {
                eprintln!("[FAIL] Galactic synthesis simulation check failed: {}", e);
                return std::process::ExitCode::FAILURE;
            }

            let gates = evaluate_all_160_gates();
            let mut failed = 0;
            for g in &gates {
                let status_str = if g.passed { "PASS" } else { "FAIL" };
                println!("[{}] {} ({}) -> {}", status_str, g.gate_id, g.subsystem, g.measured_value);
                if !g.passed {
                    failed += 1;
                }
            }
            println!("========================================================================");
            println!("SUMMARY: {}/{} GATES PASSED ({} FAILED)", gates.len() - failed, gates.len(), failed);
            println!("========================================================================");
            if failed > 0 {
                std::process::ExitCode::FAILURE
            } else {
                std::process::ExitCode::SUCCESS
            }
        }
        Some(cmd) => {
            eprintln!("Unknown command: {}. Available commands: verify", cmd);
            std::process::ExitCode::FAILURE
        }
    }
}
