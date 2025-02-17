use tasd_lib_macro::Packet;

#[derive(Debug, Packet)]
#[key = 0x0201]
pub struct LatchTrain {
    pub latch_trains: Vec<u8>,
}
