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
    node_id::NodeId,
    service_types::enums::PermissionType,
};

#[derive(Debug, Clone, PartialEq)]
pub struct RolePermissionType {
    pub role_id: NodeId,
    pub permissions: PermissionType,
}

impl MessageInfo for RolePermissionType {
    fn object_id(&self) -> ObjectId {
        ObjectId::RolePermissionType_Encoding_DefaultBinary
    }
}

impl BinaryEncoder<RolePermissionType> for RolePermissionType {
    fn byte_len(&self) -> usize {
        let mut size = 0;
        size += self.role_id.byte_len();
        size += self.permissions.byte_len();
        size
    }

    #[allow(unused_variables)]
    fn encode<S: Write>(&self, stream: &mut S) -> EncodingResult<usize> {
        let mut size = 0;
        size += self.role_id.encode(stream)?;
        size += self.permissions.encode(stream)?;
        Ok(size)
    }

    #[allow(unused_variables)]
    fn decode<S: Read>(stream: &mut S, decoding_limits: &DecodingLimits) -> EncodingResult<Self> {
        let role_id = NodeId::decode(stream, decoding_limits)?;
        let permissions = PermissionType::decode(stream, decoding_limits)?;
        Ok(RolePermissionType {
            role_id,
            permissions,
        })
    }
}
