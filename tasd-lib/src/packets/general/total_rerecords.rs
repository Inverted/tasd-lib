use tasd_lib_macro::Packet;

#[derive(Debug, Packet)]
#[key = 0x0E]
pub struct TotalRerecords {
    pub rerecords: u32,
}
