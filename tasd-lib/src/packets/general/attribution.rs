use num_enum::{IntoPrimitive, TryFromPrimitive};
use tasd_lib_macro::Packet;

#[derive(Debug, Eq, PartialEq, TryFromPrimitive, IntoPrimitive, Copy, Clone)]
#[repr(u8)]
pub enum AttributionType {
    Author = 0x01,
    Verifier = 0x02,
    FileCreator = 0x03,
    FileEditor = 0x04,
    Other = 0xFF,
}

#[derive(Debug, Packet)]
#[key = 0x05]
pub struct Attribution {
    #[small_enum]
    pub attribution_type: AttributionType,
    pub name: String,
}
