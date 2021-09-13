use std::sync::Arc;

use serde_json::{json, Value};

use crate::behaviour::entity::gate::LogicalGate;
use crate::behaviour::entity::gate::LogicalGateProperties;
use crate::behaviour::entity::gate::LOGICAL_GATES;
use crate::model::{
    DataType, EntityInstance, EntityType, PropertyType, ReactiveEntityInstance, SocketType,
};
use crate::model::{PropertyInstanceGetter, PropertyInstanceSetter};
use inexor_rgf_core_reactive::{Gate, Operation};
use std::collections::HashMap;
use uuid::Uuid;

const LHS: LogicalGateProperties = LogicalGateProperties::LHS;
const RHS: LogicalGateProperties = LogicalGateProperties::RHS;
const RESULT: LogicalGateProperties = LogicalGateProperties::RESULT;

const COMPONENT_NAME_LOGICAL_GATE: &'static str = "logical_gate";
const TYPE_NAME_AND: &str = "and";

#[test]
fn behaviour_function_should_exist() {
    assert!(LOGICAL_GATES.contains_key(TYPE_NAME_AND));
    assert!(LOGICAL_GATES.get(TYPE_NAME_AND).is_some());
}

#[test]
fn and_gate_test() {
    let property_types = vec![
        PropertyType::new_with_socket(LHS, DataType::Number, SocketType::Input),
        PropertyType::new_with_socket(RHS, DataType::Number, SocketType::Input),
        PropertyType::new_with_socket(RESULT, DataType::Number, SocketType::Output),
    ];
    let and_type = EntityType::new(
        TYPE_NAME_AND,
        "",
        vec![String::from(COMPONENT_NAME_LOGICAL_GATE)],
        Vec::new(),
        property_types,
        Vec::new(),
    );
    let and_function = LOGICAL_GATES.get(TYPE_NAME_AND).unwrap();
    let mut properties = HashMap::new();
    properties.insert(LHS.into(), json!(LHS.default_value()));
    properties.insert(RHS.into(), json!(LHS.default_value()));
    properties.insert(RESULT.into(), json!(RESULT.default_value()));
    let and_entity = EntityInstance::new(and_type.name.clone(), Uuid::new_v4(), properties);
    let and_reactive_entity = Arc::new(ReactiveEntityInstance::from(and_entity));
    let and_behaviour = LogicalGate::new(and_reactive_entity.clone(), *and_function);
    assert_eq!(TYPE_NAME_AND, and_behaviour.type_name().as_str());

    // === Reactive Entity API ===

    and_reactive_entity.set(LHS, json!(true));
    and_reactive_entity.set(RHS, json!(true));
    // Expect the correct output value -> behaviour has modified the output
    assert_eq!(true, and_reactive_entity.as_bool(RESULT).unwrap());
    and_reactive_entity.set(LHS, json!(false));
    assert_eq!(false, and_reactive_entity.as_bool(RESULT).unwrap());
    and_reactive_entity.set(RHS, json!(false));
    assert_eq!(false, and_reactive_entity.as_bool(RESULT).unwrap());
    and_reactive_entity.set(LHS, json!(true));
    assert_eq!(false, and_reactive_entity.as_bool(RESULT).unwrap());
    and_reactive_entity.set(RHS, json!(true));
    assert_eq!(true, and_reactive_entity.as_bool(RESULT).unwrap());

    // === Behaviour API ===

    and_behaviour.lhs(json!(true));
    and_behaviour.rhs(json!(true));
    assert_eq!(true, and_behaviour.result().as_bool().unwrap());
    and_behaviour.lhs(json!(false));
    assert_eq!(false, and_behaviour.result().as_bool().unwrap());
    and_behaviour.rhs(json!(false));
    assert_eq!(false, and_behaviour.result().as_bool().unwrap());
    and_behaviour.lhs(json!(true));
    assert_eq!(false, and_behaviour.result().as_bool().unwrap());
    and_behaviour.rhs(json!(true));
    assert_eq!(true, and_behaviour.result().as_bool().unwrap());
}