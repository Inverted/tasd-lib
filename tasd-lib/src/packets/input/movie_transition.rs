use crate::Packet;
use num_enum::{IntoPrimitive, TryFromPrimitive};
use tasd_lib_macro::Packet;

#[derive(Debug, Eq, PartialEq, TryFromPrimitive, IntoPrimitive, Copy, Clone)]
#[repr(u8)]
pub enum MovieTransitionType {
    SoftReset = 0x01,
    PowerReset = 0x02,
    Restart = 0x03,
    Packet = 0xFF,
}

#[derive(Debug, Packet)]
#[key = 0xFE05]
pub struct MovieTransition {
    pub frame: u32,
    #[small_enum]
    pub transition_type: MovieTransitionType,
    pub inner_packet: Option<Box<Packet>>,
}
