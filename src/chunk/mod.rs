use crc::{Crc, CRC_32_ISO_HDLC};

use crate::{chunk_type::ChunkType, Error};

struct Chunk {
    chunk_type : ChunkType,
    data : Vec<u8>,
    crc : Crc<u32>,
}

impl Chunk {
    pub fn new(chunk_type: ChunkType, data: Vec<u8>) -> Chunk {
        let crc = Crc::<u32>::new(&CRC_32_ISO_HDLC);
        let crc_32 = crc.digest();
        Chunk {
            chunk_type,
            data,
            crc,
        }
    }
}
