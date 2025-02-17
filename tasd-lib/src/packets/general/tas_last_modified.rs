use chrono::{DateTime, Utc};
use tasd_lib_macro::Packet;

#[derive(Debug, Packet)]
#[key = 0x0A]
pub struct TASLastModified {
    pub last_modified: DateTime<Utc>,
}
