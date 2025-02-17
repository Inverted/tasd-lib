use tasd_lib_macro::Packet;

#[derive(Packet, Debug)]
#[key = 0x04]
pub struct RomName {
    pub name: String,
}
