use tasd_lib_macro::Packet;

#[derive(Debug, Packet)]
#[key = 0xFE01]
pub struct InputChunk {
    pub port: u8,
    pub inputs: Vec<u8>,
}
