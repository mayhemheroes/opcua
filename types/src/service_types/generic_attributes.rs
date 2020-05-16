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
    localized_text::LocalizedText,
    service_types::GenericAttributeValue,
};

#[derive(Debug, Clone, PartialEq)]
pub struct GenericAttributes {
    pub specified_attributes: u32,
    pub display_name: LocalizedText,
    pub description: LocalizedText,
    pub write_mask: u32,
    pub user_write_mask: u32,
    pub attribute_values: Option<Vec<GenericAttributeValue>>,
}

impl BinaryEncoder<GenericAttributes> for GenericAttributes {
    fn byte_len(&self) -> usize {
        let mut size = 0;
        size += self.specified_attributes.byte_len();
        size += self.display_name.byte_len();
        size += self.description.byte_len();
        size += self.write_mask.byte_len();
        size += self.user_write_mask.byte_len();
        size += byte_len_array(&self.attribute_values);
        size
    }

    #[allow(unused_variables)]
    fn encode<S: Write>(&self, stream: &mut S) -> EncodingResult<usize> {
        let mut size = 0;
        size += self.specified_attributes.encode(stream)?;
        size += self.display_name.encode(stream)?;
        size += self.description.encode(stream)?;
        size += self.write_mask.encode(stream)?;
        size += self.user_write_mask.encode(stream)?;
        size += write_array(stream, &self.attribute_values)?;
        Ok(size)
    }

    #[allow(unused_variables)]
    fn decode<S: Read>(stream: &mut S, decoding_limits: &DecodingLimits) -> EncodingResult<Self> {
        let specified_attributes = u32::decode(stream, decoding_limits)?;
        let display_name = LocalizedText::decode(stream, decoding_limits)?;
        let description = LocalizedText::decode(stream, decoding_limits)?;
        let write_mask = u32::decode(stream, decoding_limits)?;
        let user_write_mask = u32::decode(stream, decoding_limits)?;
        let attribute_values: Option<Vec<GenericAttributeValue>> = read_array(stream, decoding_limits)?;
        Ok(GenericAttributes {
            specified_attributes,
            display_name,
            description,
            write_mask,
            user_write_mask,
            attribute_values,
        })
    }
}
