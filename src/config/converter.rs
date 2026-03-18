// Copyright (c) 2026 Xhelgi
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, version 3.

use bincode::{Decode, Encode};
use serde::{Serialize, de::DeserializeOwned};

pub fn convert_struct_in_toml<T>(data: &T) -> String
where
    T: Serialize,
{
    toml::to_string(data).expect("Cannot convert struct into toml!")
}

pub fn convert_struct_in_cache<T>(data: &T) -> Vec<u8>
where
    T: Encode,
{
    bincode::encode_to_vec(data, bincode::config::standard())
        .expect("Cannot convert struct into bincode!")
}

// TODO: Fix error handling (if file has bug, user has to delete)
pub fn convert_toml_in_structure<T>(content: &str) -> T
where
    T: DeserializeOwned,
{
    toml::from_str(content).expect("Cannot convert toml into string!")
}

// TODO: Fix error handling (if file has bug, user has to delete)
pub fn convert_cache_in_structure<T>(content: &[u8]) -> T
where
    T: Decode<()>,
{
    bincode::decode_from_slice(content, bincode::config::standard())
        .expect("Cannot convert bincode into struct!")
        .0
}
