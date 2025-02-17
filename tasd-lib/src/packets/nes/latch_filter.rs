use tasd_lib_macro::Packet;

#[derive(Debug, Packet)]
#[key = 0x0101]
pub struct LatchFilter {
    pub time: u16,
}
