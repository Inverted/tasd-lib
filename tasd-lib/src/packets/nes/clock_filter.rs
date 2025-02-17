use tasd_lib_macro::Packet;

#[derive(Debug, Packet)]
#[key = 0x0102]
pub struct ClockFilter {
    pub time: u16,
}
