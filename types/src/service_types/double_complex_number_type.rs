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
};

#[derive(Debug, Clone, PartialEq)]
pub struct DoubleComplexNumberType {
    pub real: f64,
    pub imaginary: f64,
}

impl MessageInfo for DoubleComplexNumberType {
    fn object_id(&self) -> ObjectId {
        ObjectId::DoubleComplexNumberType_Encoding_DefaultBinary
    }
}

impl BinaryEncoder<DoubleComplexNumberType> for DoubleComplexNumberType {
    fn byte_len(&self) -> usize {
        let mut size = 0;
        size += self.real.byte_len();
        size += self.imaginary.byte_len();
        size
    }

    #[allow(unused_variables)]
    fn encode<S: Write>(&self, stream: &mut S) -> EncodingResult<usize> {
        let mut size = 0;
        size += self.real.encode(stream)?;
        size += self.imaginary.encode(stream)?;
        Ok(size)
    }

    #[allow(unused_variables)]
    fn decode<S: Read>(stream: &mut S, decoding_limits: &DecodingLimits) -> EncodingResult<Self> {
        let real = f64::decode(stream, decoding_limits)?;
        let imaginary = f64::decode(stream, decoding_limits)?;
        Ok(DoubleComplexNumberType {
            real,
            imaginary,
        })
    }
}
