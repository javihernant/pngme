use std::fmt::Display;

use crate::{ChunkType, Result};
use crc::{Crc, CRC_32_ISO_HDLC};

pub struct Chunk {
    length: u32,
    chunk_type: ChunkType,
    data: Vec<u8>,
    crc: u32,
}

impl TryFrom<&[u8]> for Chunk {
    type Error = Box<dyn std::error::Error>;
    fn try_from(bytes : &[u8]) -> Result<Self> {
        let len_bytes = bytes[..4].try_into().unwrap();
        let length = u32::from_be_bytes(len_bytes);
        let type_bytes: [u8;4] = bytes[4..8].try_into().unwrap();
        let data = bytes[8..8+length as usize].to_vec();
        let crc_bytes = bytes[8+length as usize..8+length as usize+4].try_into().unwrap();
        let passed_crc = u32::from_be_bytes(crc_bytes);
        let type_and_data_bytes = type_bytes.iter().chain(data.iter()).copied().collect::<Vec<u8>>();
        let calc_crc = Crc::<u32>::new(&CRC_32_ISO_HDLC).checksum(&type_and_data_bytes);
        if passed_crc == calc_crc
        {
            Ok(
                Chunk {
                    length,
                    chunk_type: ChunkType::try_from(type_bytes).unwrap(),
                    data,
                    crc: passed_crc,
                }
            )
        }
        else {
            println!("Passed:{}, Calculated:{}", passed_crc, calc_crc);
            Err("Crc data is not valid!".into())
        }

    }
}

impl Display for Chunk {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Length: {}\nChunk Type: {}\nCrc: {}\n", self.length, self.chunk_type,self.crc)
    }
}

impl Chunk {
    pub fn new(chunk_type: ChunkType, data: Vec<u8> ) -> Chunk {
        let mut type_and_data_bytes = chunk_type.bytes().to_vec();
        type_and_data_bytes.extend_from_slice(&data);
        Chunk { 
            length: data.len() as u32,
            chunk_type,
            data,
            crc: Crc::<u32>::new(&CRC_32_ISO_HDLC).checksum(&type_and_data_bytes),
        }
    }

    pub fn length(&self) -> u32 {
        self.length
    }

    pub fn crc(&self) -> u32 {
        self.crc
    }

    pub fn chunk_type(&self) -> &ChunkType {
        &self.chunk_type
    }

    pub fn data(&self) -> &[u8] {
        &self.data[..]
    }

    pub fn data_as_string(&self) -> Result<String> {
        Ok(String::from_utf8(self.data.clone())?)
    }

    pub fn as_bytes(&self) -> Vec<u8> {
        self.length
            .to_be_bytes()
            .iter()
            .chain(self.chunk_type.bytes().iter())
            .chain(self.data.iter())
            .chain(self.crc.to_be_bytes().iter())
            .copied()
            .collect::<Vec<u8>>()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{chunk_type::ChunkType};
    use std::str::FromStr;

    fn testing_chunk() -> Chunk {
        let data_length: u32 = 42;
        let chunk_type = "RuSt".as_bytes();
        let message_bytes = "This is where your secret message will be!".as_bytes();
        let crc: u32 = 2882656334;

        let chunk_data: Vec<u8> = data_length
            .to_be_bytes()
            .iter()
            .chain(chunk_type.iter())
            .chain(message_bytes.iter())
            .chain(crc.to_be_bytes().iter())
            .copied()
            .collect();
        
        Chunk::try_from(chunk_data.as_ref()).unwrap()
    }

    #[test]
    fn test_new_chunk() {
        let chunk_type = ChunkType::from_str("RuSt").unwrap();
        let data = "This is where your secret message will be!".as_bytes().to_vec();
        let chunk = Chunk::new(chunk_type, data);
        assert_eq!(chunk.length(), 42);
        assert_eq!(chunk.crc(), 2882656334);
    }

    #[test]
    fn test_as_bytes() {
        
        let data_length: u32 = 42;
        let chunk_type = "RuSt".as_bytes();
        let chunk_type_arr: [u8;4] = chunk_type.try_into().unwrap();
        let message_bytes = "This is where your secret message will be!".as_bytes();
        let crc: u32 = 2882656334;
        let chunk = Chunk::new(ChunkType::try_from(chunk_type_arr).unwrap(), message_bytes.to_vec());

        let bytes: Vec<u8> = data_length
            .to_be_bytes()
            .iter()
            .chain(chunk_type.iter())
            .chain(message_bytes.iter())
            .chain(crc.to_be_bytes().iter())
            .copied()
            .collect();
        assert_eq!(bytes ,chunk.as_bytes())
    }

    #[test]
    fn test_crc() {
        let chunk_type = "RuSt".as_bytes();
        let chunk_type_arr: [u8;4] = chunk_type.try_into().unwrap();
        let message_bytes = "This is where your secret message will be!".as_bytes();
        let chunk = Chunk::new(ChunkType::try_from(chunk_type_arr).unwrap(), message_bytes.to_vec());
        let crc: u32 = 2882656334;

        assert_eq!(crc ,chunk.crc())
    }


    #[test]
    fn test_chunk_length() {
        let chunk = testing_chunk();
        assert_eq!(chunk.length(), 42);
    }

    #[test]
    fn test_chunk_type() {
        let chunk = testing_chunk();
        assert_eq!(chunk.chunk_type().to_string(), String::from("RuSt"));
    }

    #[test]
    fn test_chunk_string() {
        let chunk = testing_chunk();
        let chunk_string = chunk.data_as_string().unwrap();
        let expected_chunk_string = String::from("This is where your secret message will be!");
        assert_eq!(chunk_string, expected_chunk_string);
    }

    #[test]
    fn test_chunk_crc() {
        let chunk = testing_chunk();
        assert_eq!(chunk.crc(), 2882656334);
    }

    #[test]
    fn test_valid_chunk_from_bytes() {
        let data_length: u32 = 42;
        let chunk_type = "RuSt".as_bytes();
        let message_bytes = "This is where your secret message will be!".as_bytes();
        let crc: u32 = 2882656334;

        let chunk_data: Vec<u8> = data_length
            .to_be_bytes()
            .iter()
            .chain(chunk_type.iter())
            .chain(message_bytes.iter())
            .chain(crc.to_be_bytes().iter())
            .copied()
            .collect();

        let chunk = Chunk::try_from(chunk_data.as_ref()).unwrap();

        let chunk_string = chunk.data_as_string().unwrap();
        let expected_chunk_string = String::from("This is where your secret message will be!");

        assert_eq!(chunk.length(), 42);
        assert_eq!(chunk.chunk_type().to_string(), String::from("RuSt"));
        assert_eq!(chunk_string, expected_chunk_string);
        assert_eq!(chunk.crc(), 2882656334);
    }

    #[test]
    fn test_invalid_chunk_from_bytes() {
        let data_length: u32 = 42;
        let chunk_type = "RuSt".as_bytes();
        let message_bytes = "This is where your secret message will be!".as_bytes();
        let crc: u32 = 2882656333;

        let chunk_data: Vec<u8> = data_length
            .to_be_bytes()
            .iter()
            .chain(chunk_type.iter())
            .chain(message_bytes.iter())
            .chain(crc.to_be_bytes().iter())
            .copied()
            .collect();

        let chunk = Chunk::try_from(chunk_data.as_ref());

        assert!(chunk.is_err());
    }

    #[test]
    pub fn test_chunk_trait_impls() {
        let data_length: u32 = 42;
        let chunk_type = "RuSt".as_bytes();
        let message_bytes = "This is where your secret message will be!".as_bytes();
        let crc: u32 = 2882656334;

        let chunk_data: Vec<u8> = data_length
            .to_be_bytes()
            .iter()
            .chain(chunk_type.iter())
            .chain(message_bytes.iter())
            .chain(crc.to_be_bytes().iter())
            .copied()
            .collect();
        
        let chunk: Chunk = TryFrom::try_from(chunk_data.as_ref()).unwrap();
        
        let _chunk_string = format!("{}", chunk);
    }
    
}