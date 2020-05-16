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
    byte_string::ByteString,
};

#[derive(Debug, Clone, PartialEq)]
pub struct SignedSoftwareCertificate {
    pub certificate_data: ByteString,
    pub signature: ByteString,
}

impl MessageInfo for SignedSoftwareCertificate {
    fn object_id(&self) -> ObjectId {
        ObjectId::SignedSoftwareCertificate_Encoding_DefaultBinary
    }
}

impl BinaryEncoder<SignedSoftwareCertificate> for SignedSoftwareCertificate {
    fn byte_len(&self) -> usize {
        let mut size = 0;
        size += self.certificate_data.byte_len();
        size += self.signature.byte_len();
        size
    }

    #[allow(unused_variables)]
    fn encode<S: Write>(&self, stream: &mut S) -> EncodingResult<usize> {
        let mut size = 0;
        size += self.certificate_data.encode(stream)?;
        size += self.signature.encode(stream)?;
        Ok(size)
    }

    #[allow(unused_variables)]
    fn decode<S: Read>(stream: &mut S, decoding_limits: &DecodingLimits) -> EncodingResult<Self> {
        let certificate_data = ByteString::decode(stream, decoding_limits)?;
        let signature = ByteString::decode(stream, decoding_limits)?;
        Ok(SignedSoftwareCertificate {
            certificate_data,
            signature,
        })
    }
}
