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
    extension_object::ExtensionObject,
};

#[derive(Debug, Clone, PartialEq)]
pub struct MonitoringParameters {
    pub client_handle: u32,
    pub sampling_interval: f64,
    pub filter: ExtensionObject,
    pub queue_size: u32,
    pub discard_oldest: bool,
}

impl MessageInfo for MonitoringParameters {
    fn object_id(&self) -> ObjectId {
        ObjectId::MonitoringParameters_Encoding_DefaultBinary
    }
}

impl BinaryEncoder<MonitoringParameters> for MonitoringParameters {
    fn byte_len(&self) -> usize {
        let mut size = 0;
        size += self.client_handle.byte_len();
        size += self.sampling_interval.byte_len();
        size += self.filter.byte_len();
        size += self.queue_size.byte_len();
        size += self.discard_oldest.byte_len();
        size
    }

    #[allow(unused_variables)]
    fn encode<S: Write>(&self, stream: &mut S) -> EncodingResult<usize> {
        let mut size = 0;
        size += self.client_handle.encode(stream)?;
        size += self.sampling_interval.encode(stream)?;
        size += self.filter.encode(stream)?;
        size += self.queue_size.encode(stream)?;
        size += self.discard_oldest.encode(stream)?;
        Ok(size)
    }

    #[allow(unused_variables)]
    fn decode<S: Read>(stream: &mut S, decoding_limits: &DecodingLimits) -> EncodingResult<Self> {
        let client_handle = u32::decode(stream, decoding_limits)?;
        let sampling_interval = f64::decode(stream, decoding_limits)?;
        let filter = ExtensionObject::decode(stream, decoding_limits)?;
        let queue_size = u32::decode(stream, decoding_limits)?;
        let discard_oldest = bool::decode(stream, decoding_limits)?;
        Ok(MonitoringParameters {
            client_handle,
            sampling_interval,
            filter,
            queue_size,
            discard_oldest,
        })
    }
}
