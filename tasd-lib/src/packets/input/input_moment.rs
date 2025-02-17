use num_enum::{IntoPrimitive, TryFromPrimitive};
use tasd_lib_macro::Packet;

#[derive(Debug, Eq, PartialEq, TryFromPrimitive, IntoPrimitive, Copy, Clone)]
#[repr(u8)]
pub enum MomentType {
    Frame = 0x01,
    CycleCount = 0x02,
    MilliSeconds = 0x03,
    MicroSeconds10 = 0x04,
}

#[derive(Debug, Packet)]
#[key = 0xFE02]
pub struct InputMoment {
    pub port: u8,
    pub hold: bool,
    #[small_enum]
    pub index_type: MomentType,
    pub index: u64,
    pub inputs: Vec<u8>,
}
