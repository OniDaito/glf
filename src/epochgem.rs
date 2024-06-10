//!    ___  __    ____ 
//!   / __)(  )  (  __)
//!  ( (_ \/ (_/\ ) _) 
//!   \___/\____/(__) 
//!   
//! # EpochGem
//! The Tritech Gemini format uses a different Epoch to normal.
//! It starts from 1980-01-01 00:00:00 in BST so we need to convert to UTC.
//! However, at this time, BST is the same as UTC so \o/

use chrono::{DateTime, Utc, TimeZone};
use chrono_tz::GB;

/// Return the epoch of the Tritech Gemini in UTC - not the same as the Linux (or any other) time epoch. 
pub fn epoch_gem() -> DateTime<Utc> {
    let start = GB.with_ymd_and_hms(1980, 1, 1, 0, 0, 0).unwrap();
    return start.with_timezone(&Utc);
}