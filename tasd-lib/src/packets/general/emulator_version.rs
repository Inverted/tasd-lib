use tasd_lib_macro::Packet;

#[derive(Debug, Packet)]
#[key = 0x08]
pub struct EmulatorVersion {
    pub version: String,
}
