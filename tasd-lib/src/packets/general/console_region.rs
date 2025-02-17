use num_enum::{IntoPrimitive, TryFromPrimitive};
use tasd_lib_macro::Packet;

#[derive(Debug, Eq, PartialEq, TryFromPrimitive, IntoPrimitive, Copy, Clone)]
#[repr(u8)]
pub enum VideoSignal {
    NTSC = 0x01,
    PAL = 0x02,
    Other = 0xFF,
}

#[derive(Packet, Debug)]
#[key = 0x02]
pub struct ConsoleRegion {
    #[small_enum]
    pub video_signal: VideoSignal,
}

#[cfg(test)]
mod tests {
    use super::*;
    use tasd_lib_traits::Serializable;

    #[test]
    fn region_test() {
        let data = [0x00, 0x02, 0x01, 0x01, 0x02];
        let (_, region) = ConsoleRegion::deserialize(&data).unwrap();
        assert_eq!(VideoSignal::PAL, region.video_signal);
    }
}
