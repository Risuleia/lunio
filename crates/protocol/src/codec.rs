use bincode::{config, serde::{decode_from_slice, encode_to_vec}};
use serde::{Serialize, de::DeserializeOwned};

pub fn encode<T: Serialize>(value: &T) -> Vec<u8> {
    encode_to_vec(value, config::standard()).expect("bincode encode failed")
}

pub fn decode<T: DeserializeOwned>(bytes: &[u8]) -> T {
    decode_from_slice(bytes, config::standard()).expect("bincode decode failed").0
}