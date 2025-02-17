use tasd_lib_macro::Packet;

#[derive(Debug, Packet)]
#[key = 0x0804]
pub struct GameGenie {
    pub code: String,
}
