use crate::Packet;
use num_enum::{IntoPrimitive, TryFromPrimitive};
use tasd_lib_macro::Packet;

#[derive(Debug, Eq, PartialEq, TryFromPrimitive, IntoPrimitive, Copy, Clone)]
#[repr(u8)]
pub enum TransitionIndexType {
    Frame = 0x01,
    CycleCount = 0x02,
    MilliSeconds = 0x03,
    MicroSeconds10 = 0x04,
    InputChunkIndex = 0x05,
}

#[derive(Debug, Eq, PartialEq, TryFromPrimitive, IntoPrimitive, Copy, Clone)]
#[repr(u8)]
pub enum TransitionType {
    SoftReset = 0x01,
    PowerReset = 0x02,
    Restart = 0x03,
    Packet = 0xFF,
}

#[derive(Debug, Packet)]
#[key = 0xFE03]
pub struct Transition {
    #[small_enum]
    pub index_type: TransitionIndexType,
    pub port: u8,
    pub index: u64,
    #[small_enum]
    pub transition_type: TransitionType,
    pub inner_packet: Option<Box<Packet>>,
}
