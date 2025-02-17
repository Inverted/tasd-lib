use tasd_lib_macro::Packet;

#[derive(Packet, Debug)]
#[key = 0x15]
pub struct MovieFile {
    #[has_length]
    pub name: String,
    pub data: Vec<u8>,
}
