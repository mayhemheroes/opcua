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
    status_codes::StatusCode,
    diagnostic_info::DiagnosticInfo,
};

#[derive(Debug, Clone, PartialEq)]
pub struct HistoryUpdateResult {
    pub status_code: StatusCode,
    pub operation_results: Option<Vec<StatusCode>>,
    pub diagnostic_infos: Option<Vec<DiagnosticInfo>>,
}

impl MessageInfo for HistoryUpdateResult {
    fn object_id(&self) -> ObjectId {
        ObjectId::HistoryUpdateResult_Encoding_DefaultBinary
    }
}

impl BinaryEncoder<HistoryUpdateResult> for HistoryUpdateResult {
    fn byte_len(&self) -> usize {
        let mut size = 0;
        size += self.status_code.byte_len();
        size += byte_len_array(&self.operation_results);
        size += byte_len_array(&self.diagnostic_infos);
        size
    }

    #[allow(unused_variables)]
    fn encode<S: Write>(&self, stream: &mut S) -> EncodingResult<usize> {
        let mut size = 0;
        size += self.status_code.encode(stream)?;
        size += write_array(stream, &self.operation_results)?;
        size += write_array(stream, &self.diagnostic_infos)?;
        Ok(size)
    }

    #[allow(unused_variables)]
    fn decode<S: Read>(stream: &mut S, decoding_limits: &DecodingLimits) -> EncodingResult<Self> {
        let status_code = StatusCode::decode(stream, decoding_limits)?;
        let operation_results: Option<Vec<StatusCode>> = read_array(stream, decoding_limits)?;
        let diagnostic_infos: Option<Vec<DiagnosticInfo>> = read_array(stream, decoding_limits)?;
        Ok(HistoryUpdateResult {
            status_code,
            operation_results,
            diagnostic_infos,
        })
    }
}
