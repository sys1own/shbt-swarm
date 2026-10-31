//! `tests/workspace_integration.rs`
//! End-to-End Integration Test Suite for the `shbt-swarm` Workspace.

use shbt_swarm_hil::{
    evaluate_all_112_gates, evaluate_all_144_gates, evaluate_all_160_gates, evaluate_all_184_gates,
    evaluate_all_208_gates,
};

#[test]
fn integration_verify_all_112_gates() {
    let gates = evaluate_all_112_gates();
    assert_eq!(gates.len(), 112);
    for gate in &gates {
        assert!(
            gate.passed,
            "Integration gate {} failed with value: {}",
            gate.gate_id, gate.measured_value
        );
    }
}

#[test]
fn integration_verify_all_144_gates() {
    let gates = evaluate_all_144_gates();
    assert_eq!(gates.len(), 144);
    for gate in &gates {
        assert!(
            gate.passed,
            "Integration gate {} failed with value: {}",
            gate.gate_id, gate.measured_value
        );
    }
}

#[test]
fn integration_verify_all_160_gates() {
    let gates = evaluate_all_160_gates();
    assert_eq!(gates.len(), 160);
    for gate in &gates {
        assert!(
            gate.passed,
            "Integration gate {} failed with value: {}",
            gate.gate_id, gate.measured_value
        );
    }
}

#[test]
fn integration_verify_all_184_gates() {
    let gates = evaluate_all_184_gates();
    assert_eq!(gates.len(), 184);
    for gate in &gates {
        assert!(
            gate.passed,
            "Integration gate {} failed with value: {}",
            gate.gate_id, gate.measured_value
        );
    }
}

#[test]
fn integration_verify_all_208_gates() {
    let gates = evaluate_all_208_gates();
    assert_eq!(gates.len(), 208);
    for gate in &gates {
        assert!(
            gate.passed,
            "Integration gate {} failed with value: {}",
            gate.gate_id, gate.measured_value
        );
    }
}

#[test]
fn integration_verify_all_232_gates() {
    let gates = shbt_swarm_hil::evaluate_all_232_gates();
    assert_eq!(gates.len(), 232);
    for gate in &gates {
        assert!(
            gate.passed,
            "Integration gate {} failed with value: {}",
            gate.gate_id, gate.measured_value
        );
    }
}

#[test]
fn integration_verify_all_240_gates() {
    let gates = shbt_swarm_hil::evaluate_all_240_gates();
    assert_eq!(gates.len(), 240);
    for gate in &gates {
        assert!(
            gate.passed,
            "Integration gate {} failed with value: {}",
            gate.gate_id, gate.measured_value
        );
    }
}


