use tasd_lib_macro::Packet;

#[derive(Debug, Packet)]
#[key = 0xF1]
pub struct PortOverread {
    pub port: u8,
    pub high: bool,
}
