use tasd_lib_macro::Packet;

#[derive(Debug, Packet)]
#[key = 0x10]
pub struct BlankFrames {
    pub blank_frames: i16,
}
