use tasd_lib_macro::Packet;

#[derive(Debug, Packet)]
#[key = 0x11]
pub struct Verified {
    pub verified: bool,
}
