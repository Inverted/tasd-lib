use num_enum::{IntoPrimitive, TryFromPrimitive};
use tasd_lib_macro::Packet;

#[derive(Debug, Eq, PartialEq, TryFromPrimitive, IntoPrimitive, Copy, Clone)]
#[repr(u16)]
pub enum ControllerType {
    NESStandardController = 0x0101,
    NESFourScore = 0x0102,
    NESZapper = 0x0103,
    NESPowerPad = 0x0104,
    FamicomFamilyBASICKeyboard = 0x0105,
    SNESStandardController = 0x0201,
    SNESSuperMultitap = 0x0202,
    SNESMouse = 0x0203,
    SNESSuperscope = 0x0204,
    N64StandardController = 0x0301,
    N64StandardControllerWithRumblePak = 0x0302,
    N64StandardControllerWithControllerPak = 0x0303,
    N64StandardControllerWithTransferPak = 0x0304,
    N64Mouse = 0x0305,
    N64VoiceRecognitionUnit = 0x0306,
    N64RandNetKeyboard = 0x0307,
    N64DenshadeGo = 0x0308,
    GCStandardController = 0x0401,
    GCKeyboard = 0x0402,
    GBGamepad = 0x0501,
    GBCGamepad = 0x0601,
    GBAGamepad = 0x0701,
    Genesis3Button = 0x0801,
    Genesis6Button = 0x0802,
    A2600Joystick = 0x0901,
    A2600Paddle = 0x0902,
    A2600KeyboardController = 0x0903,
    Other = 0xFFFF,
}

#[derive(Debug, Packet)]
#[key = 0xF0]
pub struct PortController {
    pub port: u8,
    #[big_enum]
    pub controller: ControllerType,
}
