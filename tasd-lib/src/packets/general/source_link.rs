use tasd_lib_macro::Packet;

#[derive(Packet, Debug)]
#[key = 0x0F]
pub struct SourceLink {
    pub link: String,
}
