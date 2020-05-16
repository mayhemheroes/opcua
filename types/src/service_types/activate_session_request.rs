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
    string::UAString,
    extension_object::ExtensionObject,
    service_types::SignatureData,
    service_types::SignedSoftwareCertificate,
};

#[derive(Debug, Clone, PartialEq)]
pub struct ActivateSessionRequest {
    pub request_header: RequestHeader,
    pub client_signature: SignatureData,
    pub client_software_certificates: Option<Vec<SignedSoftwareCertificate>>,
    pub locale_ids: Option<Vec<UAString>>,
    pub user_identity_token: ExtensionObject,
    pub user_token_signature: SignatureData,
}

impl MessageInfo for ActivateSessionRequest {
    fn object_id(&self) -> ObjectId {
        ObjectId::ActivateSessionRequest_Encoding_DefaultBinary
    }
}

impl BinaryEncoder<ActivateSessionRequest> for ActivateSessionRequest {
    fn byte_len(&self) -> usize {
        let mut size = 0;
        size += self.request_header.byte_len();
        size += self.client_signature.byte_len();
        size += byte_len_array(&self.client_software_certificates);
        size += byte_len_array(&self.locale_ids);
        size += self.user_identity_token.byte_len();
        size += self.user_token_signature.byte_len();
        size
    }

    #[allow(unused_variables)]
    fn encode<S: Write>(&self, stream: &mut S) -> EncodingResult<usize> {
        let mut size = 0;
        size += self.request_header.encode(stream)?;
        size += self.client_signature.encode(stream)?;
        size += write_array(stream, &self.client_software_certificates)?;
        size += write_array(stream, &self.locale_ids)?;
        size += self.user_identity_token.encode(stream)?;
        size += self.user_token_signature.encode(stream)?;
        Ok(size)
    }

    #[allow(unused_variables)]
    fn decode<S: Read>(stream: &mut S, decoding_limits: &DecodingLimits) -> EncodingResult<Self> {
        let request_header = RequestHeader::decode(stream, decoding_limits)?;
        let client_signature = SignatureData::decode(stream, decoding_limits)?;
        let client_software_certificates: Option<Vec<SignedSoftwareCertificate>> = read_array(stream, decoding_limits)?;
        let locale_ids: Option<Vec<UAString>> = read_array(stream, decoding_limits)?;
        let user_identity_token = ExtensionObject::decode(stream, decoding_limits)?;
        let user_token_signature = SignatureData::decode(stream, decoding_limits)?;
        Ok(ActivateSessionRequest {
            request_header,
            client_signature,
            client_software_certificates,
            locale_ids,
            user_identity_token,
            user_token_signature,
        })
    }
}
