use tasd_lib_macro::Packet;

#[derive(Debug, Packet)]
#[key = 0xFFFF]
pub struct Unspecified {
    pub data: Vec<u8>,
}
