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
    string::UAString,
    byte_string::ByteString,
};

#[derive(Debug, Clone, PartialEq)]
pub struct X509IdentityToken {
    pub policy_id: UAString,
    pub certificate_data: ByteString,
}

impl BinaryEncoder<X509IdentityToken> for X509IdentityToken {
    fn byte_len(&self) -> usize {
        let mut size = 0;
        size += self.policy_id.byte_len();
        size += self.certificate_data.byte_len();
        size
    }

    #[allow(unused_variables)]
    fn encode<S: Write>(&self, stream: &mut S) -> EncodingResult<usize> {
        let mut size = 0;
        size += self.policy_id.encode(stream)?;
        size += self.certificate_data.encode(stream)?;
        Ok(size)
    }

    #[allow(unused_variables)]
    fn decode<S: Read>(stream: &mut S, decoding_limits: &DecodingLimits) -> EncodingResult<Self> {
        let policy_id = UAString::decode(stream, decoding_limits)?;
        let certificate_data = ByteString::decode(stream, decoding_limits)?;
        Ok(X509IdentityToken {
            policy_id,
            certificate_data,
        })
    }
}
