//! `crates/shbt-swarm-cli/src/main.rs`
//! Master CLI tool for sys1own/shbt-swarm verification and gate audits.

use shbt_swarm_hil::evaluate_all_104_gates;

fn main() -> std::process::ExitCode {
    let mut args = std::env::args().skip(1);
    match args.next().as_deref() {
        Some("verify") | None => {
            println!("========================================================================");
            println!("SHBT-SWARM MASTER GATE VERIFICATION MATRIX (104 GATES)");
            println!("========================================================================");
            let gates = evaluate_all_104_gates();
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
