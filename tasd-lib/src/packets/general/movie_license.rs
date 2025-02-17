use tasd_lib_macro::Packet;

#[derive(Packet, Debug)]
#[key = 0x14]
pub struct MovieLicense {
    pub license: String,
}
