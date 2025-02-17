use num_enum::{IntoPrimitive, TryFromPrimitive};
use tasd_lib_macro::Packet;

#[derive(Debug, Eq, PartialEq, TryFromPrimitive, IntoPrimitive, Copy, Clone)]
#[repr(u8)]
pub enum IdentifierType {
    MD5 = 0x01,
    SHA1 = 0x02,
    SHA224 = 0x03,
    SHA256 = 0x04,
    SHA384 = 0x05,
    SHA512 = 0x06,
    SHA512_224 = 0x07,
    SHA512_256 = 0x08,
    SHA3_224 = 0x09,
    SHA3_256 = 0x0A,
    SHA3_384 = 0x0B,
    SHA3_512 = 0x0C,
    Shake128 = 0x1D,
    Shake256 = 0x1E,
    Other = 0xFF,
}

#[derive(Debug, Eq, PartialEq, TryFromPrimitive, IntoPrimitive, Copy, Clone)]
#[repr(u8)]
pub enum Base {
    Raw = 0x01,
    Base16 = 0x02,
    Base32 = 0x03,
    Base64 = 0x04,
    Other = 0xFF,
}

#[derive(Debug, Packet)]
#[key = 0x13]
pub struct GameIdentifier {
    #[small_enum]
    pub identifier_type: IdentifierType,
    #[small_enum]
    pub base: Base,
    pub identifier: Vec<u8>,
}
