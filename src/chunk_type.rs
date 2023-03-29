use std::str::FromStr;
use std::fmt;

#[derive(Debug, PartialEq)]
pub struct ChunkType {
    ancilliary: u8,
    private: u8,
    reserved: u8,
    safe2copy: u8
}

impl ChunkType {
    pub fn bytes(&self) -> [u8; 4] {
        [self.ancilliary, self.private, self.reserved, self.safe2copy]
    }

    fn is_valid(&self) -> bool {
        self.reserved >> 5 & 1 == 0
    }

    fn is_critical(&self) -> bool {
        self.ancilliary >> 5 & 1 == 0
    }

    fn is_public(&self) -> bool {
        self.private >> 5 & 1 == 0
    }

    fn is_reserved_bit_valid(&self) -> bool {
        self.is_valid()
    }

    fn is_safe_to_copy(&self) -> bool {
        self.safe2copy >> 5 & 1 != 0
    }
}

impl TryFrom<[u8;4]> for ChunkType {
    type Error = &'static str;
    fn try_from(value: [u8;4]) -> Result<Self, Self::Error> {
        let is_err = !value.iter().all(u8::is_ascii_alphabetic);
        if is_err
        {
            Err("invalid byte(s)")
        }
        else {
            Ok(
                ChunkType { 
                    ancilliary: value[0],
                    private: value[1],
                    reserved: value[2],
                    safe2copy: value[3] 
                }
            )
        }
        
    }
}

impl FromStr for ChunkType {
    type Err = &'static str;
    fn from_str(s: &str) -> Result<Self, Self::Err> {

        let is_err = !s.bytes().all(|b| u8::is_ascii_alphabetic(&b));

        if is_err {
            Err("invalid byte(s)")
        }
        else if s.len() != 4
        {
            Err("incorrect number of bytes provided while creating ChunkType")
        }
        else
        {
            let mut bytes = s.bytes();
            Ok(
                ChunkType {
                    ancilliary: bytes.next().unwrap(),
                    private: bytes.next().unwrap(),
                    reserved: bytes.next().unwrap(),
                    safe2copy: bytes.next().unwrap(),
                }
            )
        }
        
    }
}

impl fmt::Display for ChunkType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}{}{}{}",char::from(self.ancilliary), char::from(self.private), char::from(self.reserved), char::from(self.safe2copy))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::convert::TryFrom;
    use std::str::FromStr;
    #[test]
    pub fn test_from_string_creates_same_as_from_slice()
    {
        let left = ChunkType::try_from([82, 117, 83, 116]).unwrap();
        let right = ChunkType::from_str("RuSt").unwrap();
        assert_eq!(left, right);
    }

    #[test]
    pub fn test_bytes_method() {
        let expected = [82, 117, 83, 116];
        let chunk = ChunkType::try_from([82, 117, 83, 116]).unwrap();
        assert_eq!(expected, chunk.bytes());
    }

    #[test]
    pub fn test_chunk_type_is_critical() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(chunk.is_critical());
    }

    #[test]
    pub fn test_chunk_type_is_not_critical() {
        let chunk = ChunkType::from_str("ruSt").unwrap();
        assert!(!chunk.is_critical());
    }

    #[test]
    pub fn test_valid_chunk_is_valid() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(chunk.is_valid());
    }

    #[test]
    pub fn test_invalid_chunk_is_valid() {
        let chunk = ChunkType::from_str("Rust").unwrap();
        assert!(!chunk.is_valid());

        let chunk = ChunkType::from_str("Ru1t");
        assert!(chunk.is_err());
    }

    #[test]
    pub fn test_chunk_type_is_public() {
        let chunk = ChunkType::from_str("RUSt").unwrap();
        assert!(chunk.is_public());
    }

    #[test]
    pub fn test_chunk_type_is_not_public() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(!chunk.is_public());
    }

    #[test]
    pub fn test_chunk_type_is_reserved_bit_valid() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(chunk.is_reserved_bit_valid());
    }

    #[test]
    pub fn test_chunk_type_is_reserved_bit_invalid() {
        let chunk = ChunkType::from_str("Rust").unwrap();
        assert!(!chunk.is_reserved_bit_valid());
    }

    #[test]
    pub fn test_chunk_type_is_safe_to_copy() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(chunk.is_safe_to_copy());
    }

    #[test]
    pub fn test_chunk_type_is_unsafe_to_copy() {
        let chunk = ChunkType::from_str("RuST").unwrap();
        assert!(!chunk.is_safe_to_copy());
    }

    #[test]
    pub fn test_chunk_type_string() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert_eq!(&chunk.to_string(), "RuSt");
    }

    #[test]
    pub fn test_chunk_type_trait_impls() {
        let chunk_type_1: ChunkType = TryFrom::try_from([82, 117, 83, 116]).unwrap();
        let chunk_type_2: ChunkType = FromStr::from_str("RuSt").unwrap();
        let _chunk_string = format!("{}", chunk_type_1);
        let _are_chunks_equal = chunk_type_1 == chunk_type_2;
    }
}