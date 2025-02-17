use chrono::{DateTime, Utc};
use tasd_lib_macro::Packet;

#[derive(Debug, Packet)]
#[key = 0x0B]
pub struct DumpCreated {
    pub created: DateTime<Utc>,
}
