mod tests;
mod helpers;

use crate::Error;
use std::{str::{self, FromStr}, fmt::Display};

pub const CHUNK_TYPE_SIZE : usize = 4;

#[derive(PartialEq, Eq, Debug)]
pub struct ChunkType {
    raw: [u8; CHUNK_TYPE_SIZE],
}

impl ChunkType {
    fn bytes(&self) -> [u8; 4] {
        return self.raw
    }
    fn is_valid(&self) -> bool {
        let x = helpers::pre_check_validity(&self.raw);
        match x {
            Ok(_) => true,
            Err(_) => false,
        }
    }
    fn is_critical(&self) -> bool {
        let critical = (&self.raw[0] >> 5) & 0b00000001;
        if critical == 0 {
            true
        } else {
            false
        }
    }
    fn is_public(&self) -> bool {
        let public = (&self.raw[1] >> 5) & 0b00000001;
        if public == 0 {
            true
        } else {
            false
        }
    }
    fn is_reserved_bit_valid(&self) -> bool {
        let reserved = (&self.raw[2] >> 5) & 0b00000001;
        if reserved == 0 {
            true
        } else {
            false
        }
    }
    fn is_safe_to_copy(&self) -> bool {
        let safe_to_copy = (&self.raw[3] >> 5) & 0b00000001;
        if safe_to_copy == 0 {
            false
        } else {
            true
        }        
    }
}

impl TryFrom<[u8; 4]> for ChunkType{
    type Error = Error;

    fn try_from(value: [u8; 4]) -> Result<Self, Self::Error> {
        let x = helpers::pre_check_validity(&value);
        if x.is_ok() {
            Ok(ChunkType {
                raw : value,
            })
        } else {
            Err(x.err().unwrap().into())
        }
    }
}

impl FromStr for ChunkType {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let x = helpers::pre_check_validity(&s.as_bytes());
        if x.is_ok() {
            let mut y = ChunkType {
                raw: [0; CHUNK_TYPE_SIZE],
            };
            for (i, byte) in s.as_bytes().iter().enumerate() {
                y.raw[i] = *byte;
            }
            return Ok(y);
        } else {
            Err(x.err().unwrap().into())
        }

    }
}
impl Display for ChunkType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = str::from_utf8(&self.raw);
        match s {
            Ok(s) => write!(f, "{s}"),
            Err(_) => write!(f, "invalid"),
        }
    }
}
