use tasd_lib_macro::Packet;

#[derive(Debug, Packet)]
#[key = 0x06]
pub struct Category {
    pub category: String,
}
