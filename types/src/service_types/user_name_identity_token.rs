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
pub struct UserNameIdentityToken {
    pub policy_id: UAString,
    pub user_name: UAString,
    pub password: ByteString,
    pub encryption_algorithm: UAString,
}

impl BinaryEncoder<UserNameIdentityToken> for UserNameIdentityToken {
    fn byte_len(&self) -> usize {
        let mut size = 0;
        size += self.policy_id.byte_len();
        size += self.user_name.byte_len();
        size += self.password.byte_len();
        size += self.encryption_algorithm.byte_len();
        size
    }

    #[allow(unused_variables)]
    fn encode<S: Write>(&self, stream: &mut S) -> EncodingResult<usize> {
        let mut size = 0;
        size += self.policy_id.encode(stream)?;
        size += self.user_name.encode(stream)?;
        size += self.password.encode(stream)?;
        size += self.encryption_algorithm.encode(stream)?;
        Ok(size)
    }

    #[allow(unused_variables)]
    fn decode<S: Read>(stream: &mut S, decoding_limits: &DecodingLimits) -> EncodingResult<Self> {
        let policy_id = UAString::decode(stream, decoding_limits)?;
        let user_name = UAString::decode(stream, decoding_limits)?;
        let password = ByteString::decode(stream, decoding_limits)?;
        let encryption_algorithm = UAString::decode(stream, decoding_limits)?;
        Ok(UserNameIdentityToken {
            policy_id,
            user_name,
            password,
            encryption_algorithm,
        })
    }
}
