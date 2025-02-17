use tasd_lib_macro::Packet;

#[derive(Debug, Packet)]
#[key = 0x09]
pub struct EmulatorCore {
    pub core: String,
}
