use num_enum::{IntoPrimitive, TryFromPrimitive};
use tasd_lib_macro::Packet;

#[derive(Debug, Eq, PartialEq, TryFromPrimitive, IntoPrimitive, Copy, Clone)]
#[repr(u8)]
pub enum DataType {
    NoInit = 0x01,
    All00 = 0x02,
    AllFF = 0x03,
    Repeat = 0x04,
    Random = 0x05,
    Custom = 0xFF,
}

#[derive(Debug, Eq, PartialEq, TryFromPrimitive, IntoPrimitive, Copy, Clone)]
#[repr(u16)]
pub enum DeviceData {
    NesCpuRam = 0x0101,
    NesCartridgeSavaData = 0x0102,
    SnesCpuRam = 0x0201,
    SnesCartridgeSaveData = 0x0202,
    GBCpuRam = 0x0501,
    GBCartridgeSaveData = 0x0502,
    GBCCpuRam = 0x0601,
    GBCCartridgeSaveData = 0x0602,
    GBACpuRam = 0x0701,
    GBACartridgeSaveData = 0x0702,
    GenesisCpuRam = 0x0801,
    GenesisCartridgeSaveData = 0x0802,
    A2600CpuRam = 0x0901,
    A2600CartridgeSaveData = 0x0902,
    Custom = 0xFFFF,
}

#[derive(Debug, Packet)]
#[key = 0x12]
pub struct MemoryInit {
    #[small_enum]
    pub data_type: DataType,
    #[big_enum]
    pub device_data: DeviceData,
    pub required: bool,
    #[has_length]
    pub name: String,
    pub data: Vec<u8>,
}
