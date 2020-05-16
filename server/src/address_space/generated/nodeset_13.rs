// OPCUA for Rust
// SPDX-License-Identifier: MPL-2.0
// Copyright (C) 2020 Adam Lock

// This file was autogenerated from Opc.Ua.NodeSet2.Part13.xml by tools/schema/gen_address_space.js
// DO NOT EDIT THIS FILE

#[allow(unused_imports)]
use std::{convert::TryFrom, str::FromStr};

#[allow(unused_imports)]
use crate::{
    address_space::{EventNotifier, types::*},
    prelude::{DataTypeId, ExtensionObject, LocalizedText, NodeId, ReferenceTypeId, service_types::Argument, UAString, Variant}
};

#[allow(unused_variables)]
pub fn populate_address_space(address_space: &mut AddressSpace) {
    add_object_1(address_space);
    add_object_2(address_space);
    add_object_3(address_space);
    add_object_4(address_space);
    add_object_5(address_space);
    add_object_6(address_space);
    add_object_7(address_space);
    add_object_8(address_space);
    add_object_9(address_space);
    add_object_10(address_space);
    add_object_11(address_space);
    add_object_12(address_space);
    add_object_13(address_space);
    add_object_14(address_space);
    add_object_15(address_space);
    add_object_16(address_space);
    add_object_17(address_space);
    add_object_18(address_space);
    add_object_19(address_space);
    add_object_20(address_space);
    add_object_21(address_space);
    add_object_22(address_space);
    add_object_23(address_space);
    add_object_24(address_space);
    add_object_25(address_space);
    add_object_26(address_space);
    add_object_27(address_space);
    add_object_28(address_space);
    add_object_29(address_space);
    add_object_30(address_space);
    add_object_31(address_space);
    add_object_32(address_space);
    add_object_33(address_space);
    add_object_34(address_space);
    add_object_35(address_space);
    add_object_36(address_space);
    add_object_37(address_space);
    add_objecttype_38(address_space);
    add_variable_39(address_space);
    add_variable_40(address_space);
    add_variable_41(address_space);
    add_variable_42(address_space);
}

fn add_object_1(address_space: &mut AddressSpace) {
    // Object
    let name = "Interpolative";
    let node_id = NodeId::new(0, 2341);
    let node = Object::new(&node_id, name, name, EventNotifier::empty());
    let _ = address_space.insert(node, Some(&[
        (&NodeId::new(0, 2340), &ReferenceTypeId::HasTypeDefinition, ReferenceDirection::Forward),
    ]));
}

fn add_object_2(address_space: &mut AddressSpace) {
    // Object
    let name = "Average";
    let node_id = NodeId::new(0, 2342);
    let node = Object::new(&node_id, name, name, EventNotifier::empty());
    let _ = address_space.insert(node, Some(&[
        (&NodeId::new(0, 2340), &ReferenceTypeId::HasTypeDefinition, ReferenceDirection::Forward),
    ]));
}

fn add_object_3(address_space: &mut AddressSpace) {
    // Object
    let name = "TimeAverage";
    let node_id = NodeId::new(0, 2343);
    let node = Object::new(&node_id, name, name, EventNotifier::empty());
    let _ = address_space.insert(node, Some(&[
        (&NodeId::new(0, 2340), &ReferenceTypeId::HasTypeDefinition, ReferenceDirection::Forward),
    ]));
}

fn add_object_4(address_space: &mut AddressSpace) {
    // Object
    let name = "TimeAverage2";
    let node_id = NodeId::new(0, 11285);
    let node = Object::new(&node_id, name, name, EventNotifier::empty());
    let _ = address_space.insert(node, Some(&[
        (&NodeId::new(0, 2340), &ReferenceTypeId::HasTypeDefinition, ReferenceDirection::Forward),
    ]));
}

fn add_object_5(address_space: &mut AddressSpace) {
    // Object
    let name = "Total";
    let node_id = NodeId::new(0, 2344);
    let node = Object::new(&node_id, name, name, EventNotifier::empty());
    let _ = address_space.insert(node, Some(&[
        (&NodeId::new(0, 2340), &ReferenceTypeId::HasTypeDefinition, ReferenceDirection::Forward),
    ]));
}

fn add_object_6(address_space: &mut AddressSpace) {
    // Object
    let name = "Total2";
    let node_id = NodeId::new(0, 11304);
    let node = Object::new(&node_id, name, name, EventNotifier::empty());
    let _ = address_space.insert(node, Some(&[
        (&NodeId::new(0, 2340), &ReferenceTypeId::HasTypeDefinition, ReferenceDirection::Forward),
    ]));
}

fn add_object_7(address_space: &mut AddressSpace) {
    // Object
    let name = "Minimum";
    let node_id = NodeId::new(0, 2346);
    let node = Object::new(&node_id, name, name, EventNotifier::empty());
    let _ = address_space.insert(node, Some(&[
        (&NodeId::new(0, 2340), &ReferenceTypeId::HasTypeDefinition, ReferenceDirection::Forward),
    ]));
}

fn add_object_8(address_space: &mut AddressSpace) {
    // Object
    let name = "Maximum";
    let node_id = NodeId::new(0, 2347);
    let node = Object::new(&node_id, name, name, EventNotifier::empty());
    let _ = address_space.insert(node, Some(&[
        (&NodeId::new(0, 2340), &ReferenceTypeId::HasTypeDefinition, ReferenceDirection::Forward),
    ]));
}

fn add_object_9(address_space: &mut AddressSpace) {
    // Object
    let name = "MinimumActualTime";
    let node_id = NodeId::new(0, 2348);
    let node = Object::new(&node_id, name, name, EventNotifier::empty());
    let _ = address_space.insert(node, Some(&[
        (&NodeId::new(0, 2340), &ReferenceTypeId::HasTypeDefinition, ReferenceDirection::Forward),
    ]));
}

fn add_object_10(address_space: &mut AddressSpace) {
    // Object
    let name = "MaximumActualTime";
    let node_id = NodeId::new(0, 2349);
    let node = Object::new(&node_id, name, name, EventNotifier::empty());
    let _ = address_space.insert(node, Some(&[
        (&NodeId::new(0, 2340), &ReferenceTypeId::HasTypeDefinition, ReferenceDirection::Forward),
    ]));
}

fn add_object_11(address_space: &mut AddressSpace) {
    // Object
    let name = "Range";
    let node_id = NodeId::new(0, 2350);
    let node = Object::new(&node_id, name, name, EventNotifier::empty());
    let _ = address_space.insert(node, Some(&[
        (&NodeId::new(0, 2340), &ReferenceTypeId::HasTypeDefinition, ReferenceDirection::Forward),
    ]));
}

fn add_object_12(address_space: &mut AddressSpace) {
    // Object
    let name = "Minimum2";
    let node_id = NodeId::new(0, 11286);
    let node = Object::new(&node_id, name, name, EventNotifier::empty());
    let _ = address_space.insert(node, Some(&[
        (&NodeId::new(0, 2340), &ReferenceTypeId::HasTypeDefinition, ReferenceDirection::Forward),
    ]));
}

fn add_object_13(address_space: &mut AddressSpace) {
    // Object
    let name = "Maximum2";
    let node_id = NodeId::new(0, 11287);
    let node = Object::new(&node_id, name, name, EventNotifier::empty());
    let _ = address_space.insert(node, Some(&[
        (&NodeId::new(0, 2340), &ReferenceTypeId::HasTypeDefinition, ReferenceDirection::Forward),
    ]));
}

fn add_object_14(address_space: &mut AddressSpace) {
    // Object
    let name = "MinimumActualTime2";
    let node_id = NodeId::new(0, 11305);
    let node = Object::new(&node_id, name, name, EventNotifier::empty());
    let _ = address_space.insert(node, Some(&[
        (&NodeId::new(0, 2340), &ReferenceTypeId::HasTypeDefinition, ReferenceDirection::Forward),
    ]));
}

fn add_object_15(address_space: &mut AddressSpace) {
    // Object
    let name = "MaximumActualTime2";
    let node_id = NodeId::new(0, 11306);
    let node = Object::new(&node_id, name, name, EventNotifier::empty());
    let _ = address_space.insert(node, Some(&[
        (&NodeId::new(0, 2340), &ReferenceTypeId::HasTypeDefinition, ReferenceDirection::Forward),
    ]));
}

fn add_object_16(address_space: &mut AddressSpace) {
    // Object
    let name = "Range2";
    let node_id = NodeId::new(0, 11288);
    let node = Object::new(&node_id, name, name, EventNotifier::empty());
    let _ = address_space.insert(node, Some(&[
        (&NodeId::new(0, 2340), &ReferenceTypeId::HasTypeDefinition, ReferenceDirection::Forward),
    ]));
}

fn add_object_17(address_space: &mut AddressSpace) {
    // Object
    let name = "AnnotationCount";
    let node_id = NodeId::new(0, 2351);
    let node = Object::new(&node_id, name, name, EventNotifier::empty());
    let _ = address_space.insert(node, Some(&[
        (&NodeId::new(0, 2340), &ReferenceTypeId::HasTypeDefinition, ReferenceDirection::Forward),
    ]));
}

fn add_object_18(address_space: &mut AddressSpace) {
    // Object
    let name = "Count";
    let node_id = NodeId::new(0, 2352);
    let node = Object::new(&node_id, name, name, EventNotifier::empty());
    let _ = address_space.insert(node, Some(&[
        (&NodeId::new(0, 2340), &ReferenceTypeId::HasTypeDefinition, ReferenceDirection::Forward),
    ]));
}

fn add_object_19(address_space: &mut AddressSpace) {
    // Object
    let name = "DurationInStateZero";
    let node_id = NodeId::new(0, 11307);
    let node = Object::new(&node_id, name, name, EventNotifier::empty());
    let _ = address_space.insert(node, Some(&[
        (&NodeId::new(0, 2340), &ReferenceTypeId::HasTypeDefinition, ReferenceDirection::Forward),
    ]));
}

fn add_object_20(address_space: &mut AddressSpace) {
    // Object
    let name = "DurationInStateNonZero";
    let node_id = NodeId::new(0, 11308);
    let node = Object::new(&node_id, name, name, EventNotifier::empty());
    let _ = address_space.insert(node, Some(&[
        (&NodeId::new(0, 2340), &ReferenceTypeId::HasTypeDefinition, ReferenceDirection::Forward),
    ]));
}

fn add_object_21(address_space: &mut AddressSpace) {
    // Object
    let name = "NumberOfTransitions";
    let node_id = NodeId::new(0, 2355);
    let node = Object::new(&node_id, name, name, EventNotifier::empty());
    let _ = address_space.insert(node, Some(&[
        (&NodeId::new(0, 2340), &ReferenceTypeId::HasTypeDefinition, ReferenceDirection::Forward),
    ]));
}

fn add_object_22(address_space: &mut AddressSpace) {
    // Object
    let name = "Start";
    let node_id = NodeId::new(0, 2357);
    let node = Object::new(&node_id, name, name, EventNotifier::empty());
    let _ = address_space.insert(node, Some(&[
        (&NodeId::new(0, 2340), &ReferenceTypeId::HasTypeDefinition, ReferenceDirection::Forward),
    ]));
}

fn add_object_23(address_space: &mut AddressSpace) {
    // Object
    let name = "End";
    let node_id = NodeId::new(0, 2358);
    let node = Object::new(&node_id, name, name, EventNotifier::empty());
    let _ = address_space.insert(node, Some(&[
        (&NodeId::new(0, 2340), &ReferenceTypeId::HasTypeDefinition, ReferenceDirection::Forward),
    ]));
}

fn add_object_24(address_space: &mut AddressSpace) {
    // Object
    let name = "Delta";
    let node_id = NodeId::new(0, 2359);
    let node = Object::new(&node_id, name, name, EventNotifier::empty());
    let _ = address_space.insert(node, Some(&[
        (&NodeId::new(0, 2340), &ReferenceTypeId::HasTypeDefinition, ReferenceDirection::Forward),
    ]));
}

fn add_object_25(address_space: &mut AddressSpace) {
    // Object
    let name = "StartBound";
    let node_id = NodeId::new(0, 11505);
    let node = Object::new(&node_id, name, name, EventNotifier::empty());
    let _ = address_space.insert(node, Some(&[
        (&NodeId::new(0, 2340), &ReferenceTypeId::HasTypeDefinition, ReferenceDirection::Forward),
    ]));
}

fn add_object_26(address_space: &mut AddressSpace) {
    // Object
    let name = "EndBound";
    let node_id = NodeId::new(0, 11506);
    let node = Object::new(&node_id, name, name, EventNotifier::empty());
    let _ = address_space.insert(node, Some(&[
        (&NodeId::new(0, 2340), &ReferenceTypeId::HasTypeDefinition, ReferenceDirection::Forward),
    ]));
}

fn add_object_27(address_space: &mut AddressSpace) {
    // Object
    let name = "DeltaBounds";
    let node_id = NodeId::new(0, 11507);
    let node = Object::new(&node_id, name, name, EventNotifier::empty());
    let _ = address_space.insert(node, Some(&[
        (&NodeId::new(0, 2340), &ReferenceTypeId::HasTypeDefinition, ReferenceDirection::Forward),
    ]));
}

fn add_object_28(address_space: &mut AddressSpace) {
    // Object
    let name = "DurationGood";
    let node_id = NodeId::new(0, 2360);
    let node = Object::new(&node_id, name, name, EventNotifier::empty());
    let _ = address_space.insert(node, Some(&[
        (&NodeId::new(0, 2340), &ReferenceTypeId::HasTypeDefinition, ReferenceDirection::Forward),
    ]));
}

fn add_object_29(address_space: &mut AddressSpace) {
    // Object
    let name = "DurationBad";
    let node_id = NodeId::new(0, 2361);
    let node = Object::new(&node_id, name, name, EventNotifier::empty());
    let _ = address_space.insert(node, Some(&[
        (&NodeId::new(0, 2340), &ReferenceTypeId::HasTypeDefinition, ReferenceDirection::Forward),
    ]));
}

fn add_object_30(address_space: &mut AddressSpace) {
    // Object
    let name = "PercentGood";
    let node_id = NodeId::new(0, 2362);
    let node = Object::new(&node_id, name, name, EventNotifier::empty());
    let _ = address_space.insert(node, Some(&[
        (&NodeId::new(0, 2340), &ReferenceTypeId::HasTypeDefinition, ReferenceDirection::Forward),
    ]));
}

fn add_object_31(address_space: &mut AddressSpace) {
    // Object
    let name = "PercentBad";
    let node_id = NodeId::new(0, 2363);
    let node = Object::new(&node_id, name, name, EventNotifier::empty());
    let _ = address_space.insert(node, Some(&[
        (&NodeId::new(0, 2340), &ReferenceTypeId::HasTypeDefinition, ReferenceDirection::Forward),
    ]));
}

fn add_object_32(address_space: &mut AddressSpace) {
    // Object
    let name = "WorstQuality";
    let node_id = NodeId::new(0, 2364);
    let node = Object::new(&node_id, name, name, EventNotifier::empty());
    let _ = address_space.insert(node, Some(&[
        (&NodeId::new(0, 2340), &ReferenceTypeId::HasTypeDefinition, ReferenceDirection::Forward),
    ]));
}

fn add_object_33(address_space: &mut AddressSpace) {
    // Object
    let name = "WorstQuality2";
    let node_id = NodeId::new(0, 11292);
    let node = Object::new(&node_id, name, name, EventNotifier::empty());
    let _ = address_space.insert(node, Some(&[
        (&NodeId::new(0, 2340), &ReferenceTypeId::HasTypeDefinition, ReferenceDirection::Forward),
    ]));
}

fn add_object_34(address_space: &mut AddressSpace) {
    // Object
    let name = "StandardDeviationSample";
    let node_id = NodeId::new(0, 11426);
    let node = Object::new(&node_id, name, name, EventNotifier::empty());
    let _ = address_space.insert(node, Some(&[
        (&NodeId::new(0, 2340), &ReferenceTypeId::HasTypeDefinition, ReferenceDirection::Forward),
    ]));
}

fn add_object_35(address_space: &mut AddressSpace) {
    // Object
    let name = "StandardDeviationPopulation";
    let node_id = NodeId::new(0, 11427);
    let node = Object::new(&node_id, name, name, EventNotifier::empty());
    let _ = address_space.insert(node, Some(&[
        (&NodeId::new(0, 2340), &ReferenceTypeId::HasTypeDefinition, ReferenceDirection::Forward),
    ]));
}

fn add_object_36(address_space: &mut AddressSpace) {
    // Object
    let name = "VarianceSample";
    let node_id = NodeId::new(0, 11428);
    let node = Object::new(&node_id, name, name, EventNotifier::empty());
    let _ = address_space.insert(node, Some(&[
        (&NodeId::new(0, 2340), &ReferenceTypeId::HasTypeDefinition, ReferenceDirection::Forward),
    ]));
}

fn add_object_37(address_space: &mut AddressSpace) {
    // Object
    let name = "VariancePopulation";
    let node_id = NodeId::new(0, 11429);
    let node = Object::new(&node_id, name, name, EventNotifier::empty());
    let _ = address_space.insert(node, Some(&[
        (&NodeId::new(0, 2340), &ReferenceTypeId::HasTypeDefinition, ReferenceDirection::Forward),
    ]));
}

fn add_objecttype_38(address_space: &mut AddressSpace) {
    // ObjectType
    let name = "AggregateConfigurationType";
    let node_id = NodeId::new(0, 11187);
    let node = ObjectType::new(&node_id, name, name, false);
    let _ = address_space.insert(node, Some(&[
        (&NodeId::new(0, 11188), &ReferenceTypeId::HasProperty, ReferenceDirection::Forward),
        (&NodeId::new(0, 11189), &ReferenceTypeId::HasProperty, ReferenceDirection::Forward),
        (&NodeId::new(0, 11190), &ReferenceTypeId::HasProperty, ReferenceDirection::Forward),
        (&NodeId::new(0, 11191), &ReferenceTypeId::HasProperty, ReferenceDirection::Forward),
        (&NodeId::new(0, 58), &ReferenceTypeId::HasSubtype, ReferenceDirection::Inverse),
    ]));
}

fn add_variable_39(address_space: &mut AddressSpace) {
    // Variable
    let name = "TreatUncertainAsBad";
    let value = Variant::Empty;
    let node_id = NodeId::new(0, 11188);
    let node = Variable::new_data_value(&node_id, name, name, NodeId::new(0, 1), value);
    let _ = address_space.insert(node, Some(&[
        (&NodeId::new(0, 68), &ReferenceTypeId::HasTypeDefinition, ReferenceDirection::Forward),
        (&NodeId::new(0, 78), &ReferenceTypeId::HasModellingRule, ReferenceDirection::Forward),
        (&NodeId::new(0, 11187), &ReferenceTypeId::HasProperty, ReferenceDirection::Inverse),
    ]));
}

fn add_variable_40(address_space: &mut AddressSpace) {
    // Variable
    let name = "PercentDataBad";
    let value = Variant::Empty;
    let node_id = NodeId::new(0, 11189);
    let node = Variable::new_data_value(&node_id, name, name, NodeId::new(0, 3), value);
    let _ = address_space.insert(node, Some(&[
        (&NodeId::new(0, 68), &ReferenceTypeId::HasTypeDefinition, ReferenceDirection::Forward),
        (&NodeId::new(0, 78), &ReferenceTypeId::HasModellingRule, ReferenceDirection::Forward),
        (&NodeId::new(0, 11187), &ReferenceTypeId::HasProperty, ReferenceDirection::Inverse),
    ]));
}

fn add_variable_41(address_space: &mut AddressSpace) {
    // Variable
    let name = "PercentDataGood";
    let value = Variant::Empty;
    let node_id = NodeId::new(0, 11190);
    let node = Variable::new_data_value(&node_id, name, name, NodeId::new(0, 3), value);
    let _ = address_space.insert(node, Some(&[
        (&NodeId::new(0, 68), &ReferenceTypeId::HasTypeDefinition, ReferenceDirection::Forward),
        (&NodeId::new(0, 78), &ReferenceTypeId::HasModellingRule, ReferenceDirection::Forward),
        (&NodeId::new(0, 11187), &ReferenceTypeId::HasProperty, ReferenceDirection::Inverse),
    ]));
}

fn add_variable_42(address_space: &mut AddressSpace) {
    // Variable
    let name = "UseSlopedExtrapolation";
    let value = Variant::Empty;
    let node_id = NodeId::new(0, 11191);
    let node = Variable::new_data_value(&node_id, name, name, NodeId::new(0, 1), value);
    let _ = address_space.insert(node, Some(&[
        (&NodeId::new(0, 68), &ReferenceTypeId::HasTypeDefinition, ReferenceDirection::Forward),
        (&NodeId::new(0, 78), &ReferenceTypeId::HasModellingRule, ReferenceDirection::Forward),
        (&NodeId::new(0, 11187), &ReferenceTypeId::HasProperty, ReferenceDirection::Inverse),
    ]));
}

