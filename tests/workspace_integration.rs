//! `tests/workspace_integration.rs`
//! End-to-End Integration Test Suite for the `shbt-swarm` Workspace.

use shbt_swarm_hil::evaluate_all_104_gates;

#[test]
fn integration_verify_all_104_gates() {
    let gates = evaluate_all_104_gates();
    assert_eq!(gates.len(), 104);
    for gate in &gates {
        assert!(
            gate.passed,
            "Integration gate {} failed with value: {}",
            gate.gate_id, gate.measured_value
        );
    }
}
