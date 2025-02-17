use tasd_lib_macro::Packet;

#[derive(Debug, Packet)]
#[key = 0xFE04]
pub struct LagChunk {
    pub movie_frame: u32,
    pub count: u32,
}
