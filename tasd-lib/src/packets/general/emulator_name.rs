use tasd_lib_macro::Packet;

#[derive(Debug, Packet)]
#[key = 0x07]
pub struct EmulatorName {
    pub name: String,
}
