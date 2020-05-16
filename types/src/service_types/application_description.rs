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
    string::UAString,
    localized_text::LocalizedText,
    service_types::enums::ApplicationType,
};

#[derive(Debug, Clone, PartialEq)]
pub struct ApplicationDescription {
    pub application_uri: UAString,
    pub product_uri: UAString,
    pub application_name: LocalizedText,
    pub application_type: ApplicationType,
    pub gateway_server_uri: UAString,
    pub discovery_profile_uri: UAString,
    pub discovery_urls: Option<Vec<UAString>>,
}

impl MessageInfo for ApplicationDescription {
    fn object_id(&self) -> ObjectId {
        ObjectId::ApplicationDescription_Encoding_DefaultBinary
    }
}

impl BinaryEncoder<ApplicationDescription> for ApplicationDescription {
    fn byte_len(&self) -> usize {
        let mut size = 0;
        size += self.application_uri.byte_len();
        size += self.product_uri.byte_len();
        size += self.application_name.byte_len();
        size += self.application_type.byte_len();
        size += self.gateway_server_uri.byte_len();
        size += self.discovery_profile_uri.byte_len();
        size += byte_len_array(&self.discovery_urls);
        size
    }

    #[allow(unused_variables)]
    fn encode<S: Write>(&self, stream: &mut S) -> EncodingResult<usize> {
        let mut size = 0;
        size += self.application_uri.encode(stream)?;
        size += self.product_uri.encode(stream)?;
        size += self.application_name.encode(stream)?;
        size += self.application_type.encode(stream)?;
        size += self.gateway_server_uri.encode(stream)?;
        size += self.discovery_profile_uri.encode(stream)?;
        size += write_array(stream, &self.discovery_urls)?;
        Ok(size)
    }

    #[allow(unused_variables)]
    fn decode<S: Read>(stream: &mut S, decoding_limits: &DecodingLimits) -> EncodingResult<Self> {
        let application_uri = UAString::decode(stream, decoding_limits)?;
        let product_uri = UAString::decode(stream, decoding_limits)?;
        let application_name = LocalizedText::decode(stream, decoding_limits)?;
        let application_type = ApplicationType::decode(stream, decoding_limits)?;
        let gateway_server_uri = UAString::decode(stream, decoding_limits)?;
        let discovery_profile_uri = UAString::decode(stream, decoding_limits)?;
        let discovery_urls: Option<Vec<UAString>> = read_array(stream, decoding_limits)?;
        Ok(ApplicationDescription {
            application_uri,
            product_uri,
            application_name,
            application_type,
            gateway_server_uri,
            discovery_profile_uri,
            discovery_urls,
        })
    }
}
