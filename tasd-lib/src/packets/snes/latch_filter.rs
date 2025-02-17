use tasd_lib_macro::Packet;

#[derive(Debug, Packet)]
#[key = 0x0201]
pub struct LatchFilter {
    pub time: u16,
}
