use tasd_lib_macro::Packet;

#[derive(Debug, Packet)]
#[key = 0xFFFE]
pub struct Experimental {
    pub experimental: bool,
}
