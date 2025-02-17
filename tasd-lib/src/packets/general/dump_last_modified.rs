use chrono::{DateTime, Utc};
use tasd_lib_macro::Packet;

#[derive(Debug, Packet)]
#[key = 0x0C]
pub struct DumpLastModified {
    pub last_modified: DateTime<Utc>,
}
