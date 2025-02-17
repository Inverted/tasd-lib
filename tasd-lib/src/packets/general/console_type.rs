use num_enum::{IntoPrimitive, TryFromPrimitive};
use std::str;
use tasd_lib_macro::Packet;

#[derive(Debug, Eq, PartialEq, TryFromPrimitive, IntoPrimitive, Copy, Clone)]
#[repr(u8)]
pub enum Console {
    NES = 0x01,
    SNES = 0x02,
    N64 = 0x03,
    GC = 0x04,
    GB = 0x05,
    GBC = 0x06,
    GBA = 0x07,
    GENESIS = 0x08,
    A2600 = 0x09,
    Custom = 0xFF,
}

#[derive(Debug, Packet)]
#[key = 0x01]
pub struct ConsoleType {
    #[small_enum]
    pub console: Console,
    pub name: String,
}

#[cfg(test)]
mod tests {
    use super::*;
    use tasd_lib_traits::Serializable;

    #[test]
    fn console_test() {
        let data: [u8; 7] = [0x00, 0x01, 0x01, 0x03, 0x0FF, 0x48, 0x69];
        let (_, console) = ConsoleType::deserialize(&data).unwrap();
        assert_eq!(Console::Custom, console.console);
    }
}
