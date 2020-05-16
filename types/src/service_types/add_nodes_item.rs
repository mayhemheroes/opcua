// OPCUA for Rust
// SPDX-License-Identifier: MPL-2.0
// Copyright (C) 2020 Adam Lock

// This file was autogenerated from Opc.Ua.Types.bsd.xml by tools/schema/gen_types.js
// DO NOT EDIT THIS FILE

use std::io::{Read, Write};

#[allow(unused_imports)]
use crate::{
    encoding::*,
    basic_types::*,
    service_types::impls::MessageInfo,
    node_ids::ObjectId,
    node_id::ExpandedNodeId,
    node_id::NodeId,
    qualified_name::QualifiedName,
    service_types::enums::NodeClass,
    extension_object::ExtensionObject,
};

#[derive(Debug, Clone, PartialEq)]
pub struct AddNodesItem {
    pub parent_node_id: ExpandedNodeId,
    pub reference_type_id: NodeId,
    pub requested_new_node_id: ExpandedNodeId,
    pub browse_name: QualifiedName,
    pub node_class: NodeClass,
    pub node_attributes: ExtensionObject,
    pub type_definition: ExpandedNodeId,
}

impl MessageInfo for AddNodesItem {
    fn object_id(&self) -> ObjectId {
        ObjectId::AddNodesItem_Encoding_DefaultBinary
    }
}

impl BinaryEncoder<AddNodesItem> for AddNodesItem {
    fn byte_len(&self) -> usize {
        let mut size = 0;
        size += self.parent_node_id.byte_len();
        size += self.reference_type_id.byte_len();
        size += self.requested_new_node_id.byte_len();
        size += self.browse_name.byte_len();
        size += self.node_class.byte_len();
        size += self.node_attributes.byte_len();
        size += self.type_definition.byte_len();
        size
    }

    #[allow(unused_variables)]
    fn encode<S: Write>(&self, stream: &mut S) -> EncodingResult<usize> {
        let mut size = 0;
        size += self.parent_node_id.encode(stream)?;
        size += self.reference_type_id.encode(stream)?;
        size += self.requested_new_node_id.encode(stream)?;
        size += self.browse_name.encode(stream)?;
        size += self.node_class.encode(stream)?;
        size += self.node_attributes.encode(stream)?;
        size += self.type_definition.encode(stream)?;
        Ok(size)
    }

    #[allow(unused_variables)]
    fn decode<S: Read>(stream: &mut S, decoding_limits: &DecodingLimits) -> EncodingResult<Self> {
        let parent_node_id = ExpandedNodeId::decode(stream, decoding_limits)?;
        let reference_type_id = NodeId::decode(stream, decoding_limits)?;
        let requested_new_node_id = ExpandedNodeId::decode(stream, decoding_limits)?;
        let browse_name = QualifiedName::decode(stream, decoding_limits)?;
        let node_class = NodeClass::decode(stream, decoding_limits)?;
        let node_attributes = ExtensionObject::decode(stream, decoding_limits)?;
        let type_definition = ExpandedNodeId::decode(stream, decoding_limits)?;
        Ok(AddNodesItem {
            parent_node_id,
            reference_type_id,
            requested_new_node_id,
            browse_name,
            node_class,
            node_attributes,
            type_definition,
        })
    }
}
