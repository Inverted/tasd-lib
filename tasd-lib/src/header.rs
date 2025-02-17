use nom::bytes::streaming::tag;
use nom::number::streaming::{be_u16, be_u8};
use nom::IResult;
use std::fmt;
use std::fmt::Formatter;
use tasd_lib_traits::Serializable;

#[derive(Debug)]
pub struct Header {
    pub version: u16,
}

impl Serializable for Header {
    fn size(&self) -> u64 {
        return 0; // not needed
    }

    fn serialize(&self) -> Vec<u8> {
        let mut bytes: Vec<u8> = Vec::new();
        bytes.extend("TASD".as_bytes());
        bytes.extend((self.version as u16).to_be_bytes());
        bytes.push(2u8);

        bytes
    }
    fn deserialize(input: &[u8]) -> IResult<&[u8], Self> {
        let (input, _) = tag("TASD")(input)?;
        let (input, version) = be_u16(input)?;
        let (input, keylen) = be_u8(input)?;

        assert!(keylen == 2);

        Ok((input, Header { version }))
    }
}

impl fmt::Display for Header {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "Version: {}", self.version)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn header_test() {
        let data = include_bytes!("../assets/sample.tasd");
        let (_, header) = Header::deserialize(data).unwrap();
        assert_eq!(1, header.version);
    }
}
