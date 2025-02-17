use tasd_lib_macro::Packet;

#[derive(Debug, Packet)]
#[key = 0x0D]
pub struct TotalFrames {
    pub frames: u32,
}
