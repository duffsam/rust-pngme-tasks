use super::CHUNK_TYPE_SIZE;

pub fn pre_check_validity(data: &[u8]) -> Result<bool, String> {

    if data.len() != CHUNK_TYPE_SIZE {
        return Err("Incorrect number of bytes passed. Must be 4".into())
    }

    for byte in data {
        if byte.is_ascii_uppercase() || byte.is_ascii_lowercase(){
            continue;
        } else {
            return Err("Byte is not correct ASCII format (a-z, A-Z)".into())
        }            
    }
    Ok(true)
}
