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
    request_header::RequestHeader,
    extension_object::ExtensionObject,
};

#[derive(Debug, Clone, PartialEq)]
pub struct HistoryUpdateRequest {
    pub request_header: RequestHeader,
    pub history_update_details: Option<Vec<ExtensionObject>>,
}

impl MessageInfo for HistoryUpdateRequest {
    fn object_id(&self) -> ObjectId {
        ObjectId::HistoryUpdateRequest_Encoding_DefaultBinary
    }
}

impl BinaryEncoder<HistoryUpdateRequest> for HistoryUpdateRequest {
    fn byte_len(&self) -> usize {
        let mut size = 0;
        size += self.request_header.byte_len();
        size += byte_len_array(&self.history_update_details);
        size
    }

    #[allow(unused_variables)]
    fn encode<S: Write>(&self, stream: &mut S) -> EncodingResult<usize> {
        let mut size = 0;
        size += self.request_header.encode(stream)?;
        size += write_array(stream, &self.history_update_details)?;
        Ok(size)
    }

    #[allow(unused_variables)]
    fn decode<S: Read>(stream: &mut S, decoding_limits: &DecodingLimits) -> EncodingResult<Self> {
        let request_header = RequestHeader::decode(stream, decoding_limits)?;
        let history_update_details: Option<Vec<ExtensionObject>> = read_array(stream, decoding_limits)?;
        Ok(HistoryUpdateRequest {
            request_header,
            history_update_details,
        })
    }
}
