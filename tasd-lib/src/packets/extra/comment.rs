use tasd_lib_macro::Packet;

#[derive(Debug, Packet)]
#[key = 0xFF01]
pub struct Comment {
    pub comment: String,
}
