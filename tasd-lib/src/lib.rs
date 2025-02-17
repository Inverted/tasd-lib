use nom::IResult;
pub use tasd_lib_traits::Serializable;

use crate::header::Header;
use crate::packets::general::attribution::Attribution;
use crate::packets::general::blank_frames::BlankFrames;
use crate::packets::general::category::Category;
use crate::packets::general::console_region::ConsoleRegion;
use crate::packets::general::console_type::ConsoleType;
use crate::packets::general::dump_created::DumpCreated;
use crate::packets::general::dump_last_modified::DumpLastModified;
use crate::packets::general::emulator_name::EmulatorName;
use crate::packets::general::emulator_version::EmulatorVersion;
use crate::packets::general::game_identifier::GameIdentifier;
use crate::packets::general::game_title::GameTitle;
use crate::packets::general::memory_init::MemoryInit;
use crate::packets::general::movie_file::MovieFile;
use crate::packets::general::movie_license::MovieLicense;
use crate::packets::general::port_controller::PortController;
use crate::packets::general::port_overread::PortOverread;
use crate::packets::general::rom_name::RomName;
use crate::packets::general::source_link::SourceLink;
use crate::packets::general::tas_last_modified::TASLastModified;
use crate::packets::general::total_frames::TotalFrames;
use crate::packets::general::total_rerecords::TotalRerecords;
use crate::packets::general::unknown::Unknown;
use crate::packets::general::verified::Verified;

use crate::packets::extra::comment::Comment;
use crate::packets::extra::experimental::Experimental;
use crate::packets::extra::unspecified::Unspecified;

use crate::packets::input::input_chunk::InputChunk;
use crate::packets::input::input_moment::InputMoment;
use crate::packets::input::lag_chunk::LagChunk;
use crate::packets::input::movie_transition::MovieTransition;
use crate::packets::input::transition::Transition;

use crate::packets::genesis::game_genie::GameGenie as GenesisGameGenie;

use crate::packets::nes::clock_filter::ClockFilter as NESClockFilter;
use crate::packets::nes::game_genie::GameGenie as NESGameGenie;
use crate::packets::nes::latch_filter::LatchFilter as NESLatchFilter;

use crate::packets::snes::clock_filter::ClockFilter as SNESClockFilter;
use crate::packets::snes::game_genie::GameGenie as SNESGameGenie;
use crate::packets::snes::latch_filter::LatchFilter as SNESLatchFilter;
use crate::packets::snes::latch_train::LatchTrain as SNESLatchTrain;

mod header;
mod macros;
mod packets;

create_parsers!(
    Attribution,
    Category,
    ConsoleRegion,
    ConsoleType,
    EmulatorName,
    EmulatorVersion,
    GameTitle,
    RomName,
    TASLastModified,
    DumpCreated,
    DumpLastModified,
    TotalFrames,
    TotalRerecords,
    SourceLink,
    BlankFrames,
    Verified,
    MemoryInit,
    GameIdentifier,
    MovieLicense,
    MovieFile,
    PortController,
    PortOverread,
    Comment,
    Experimental,
    Unspecified,
    InputChunk,
    LagChunk,
    InputMoment,
    MovieTransition,
    Transition,
    GenesisGameGenie,
    NESGameGenie,
    NESLatchFilter,
    NESClockFilter,
    SNESGameGenie,
    SNESLatchFilter,
    SNESClockFilter,
    SNESLatchTrain,
    Unknown
);

#[derive(Debug)]
pub struct TASD {
    pub header: Header,
    pub packets: Vec<Packet>,
}

impl Serializable for TASD {
    fn size(&self) -> u64 {
        return 0; // not needed
    }

    fn serialize(&self) -> Vec<u8> {
        let mut bytes: Vec<u8> = Vec::new();
        bytes.extend(self.header.serialize());
        for packet in &self.packets {
            bytes.extend(packet.serialize());
        }
        bytes
    }

    fn deserialize(input: &[u8]) -> IResult<&[u8], Self> where Self: Sized {
        let (input, header) = Header::deserialize(input)?;
        let mut packets: Vec<Packet> = Vec::new();
        let mut current_input = input;
        loop {
            let (new_input, packet) = Packet::deserialize(current_input)?;
            packets.push(packet);
            if new_input.is_empty() {
                break;
            }
            current_input = new_input;
        }
        Ok((input, TASD { header, packets }))
    }
}

#[cfg(test)]
mod tests {
    use std::fs;
    use std::io::Write;
    use crate::packets::input::transition::{TransitionIndexType, TransitionType};
    use super::*;

    #[test]
    fn test_file() {
        let data = include_bytes!("../assets/5256M.tasd");
        TASD::deserialize(data).unwrap();
    }

    #[test]
    fn test_sample() {
        let data = include_bytes!("../assets/sample.tasd");
        TASD::deserialize(data).unwrap();
    }

    #[test]
    fn test_file2() {
        let data = include_bytes!("../assets/4616M.tasd");
        TASD::deserialize(data).unwrap();
    }

    #[test]
    fn test_serialize() {
        let test_data = TASD {
            header: Header { version: 1},
            packets: vec![
                Packet::Comment(Comment{ comment: "Hello world!".parse().unwrap() }),
                Packet::Transition(Transition{
                    transition_type: TransitionType::Packet,
                    index_type: TransitionIndexType::Frame,
                    port: 1,
                    index: 999,
                    inner_packet: Some(Box::new(Packet::Experimental(Experimental{experimental: true})))
                })
            ]
        };
        let mut file = fs::OpenOptions::new()
            .create(true)
            .write(true)
            .open("foo.tasd").unwrap();
        file.write_all(&test_data.serialize()).unwrap();
        file.flush().unwrap();
    }

    #[test]
    fn test_deserialize_serialized() {
        let data = include_bytes!("../foo.tasd");
        TASD::deserialize(data).unwrap();
    }
}
