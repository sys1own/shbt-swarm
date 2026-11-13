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

#[test]
fn integration_verify_all_250_gates() {
    let gates = shbt_swarm_hil::evaluate_all_250_gates();
    assert_eq!(gates.len(), 250);
    for gate in &gates {
        assert!(
            gate.passed,
            "Integration gate {} failed with value: {}",
            gate.gate_id, gate.measured_value
        );
    }
}

#[test]
fn integration_verify_all_280_gates() {
    let gates = shbt_swarm_hil::evaluate_all_280_gates();
    assert_eq!(gates.len(), 280);
    for gate in &gates {
        assert!(
            gate.passed,
            "Integration gate {} failed with value: {}",
            gate.gate_id, gate.measured_value
        );
    }
}

#[test]
fn integration_verify_all_304_gates() {
    let gates = shbt_swarm_hil::evaluate_all_304_gates();
    assert_eq!(gates.len(), 304);
    for gate in &gates {
        assert!(
            gate.passed,
            "Integration gate {} failed with value: {}",
            gate.gate_id, gate.measured_value
        );
    }
}

#[test]
fn integration_verify_all_320_gates() {
    let gates = shbt_swarm_hil::evaluate_all_320_gates();
    assert_eq!(gates.len(), 320);
    for gate in &gates {
        assert!(
            gate.passed,
            "Integration gate {} failed with value: {}",
            gate.gate_id, gate.measured_value
        );
    }
}

#[test]
fn integration_verify_all_344_gates() {
    let gates = shbt_swarm_hil::evaluate_all_344_gates();
    assert_eq!(gates.len(), 344);
    for gate in &gates {
        assert!(
            gate.passed,
            "Integration gate {} failed with value: {}",
            gate.gate_id, gate.measured_value
        );
    }
}




#[test]
fn integration_verify_all_376_gates() {
    let gates = shbt_swarm_hil::evaluate_all_376_gates();
    assert_eq!(gates.len(), 376);
    for gate in &gates {
        assert!(
            gate.passed,
            "Integration gate {} failed with value: {}",
            gate.gate_id, gate.measured_value
        );
    }
}
