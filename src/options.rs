use std::time::Duration;

#[non_exhaustive]
pub struct Options {
    /// POST digest URLs of the calendar servers.
    pub calendars: Vec<String>,

    /// The minimum number of calendars needed for a timestamp to be considered usable.
    ///
    /// Default: 2
    pub at_least: usize,

    /// Overall timeout for each request to a calendar in milliseconds.
    ///
    /// Default: 5 seconds
    pub timeout: Duration,
}

impl Default for Options {
    fn default() -> Self {
        Self {
            calendars: CALENDARS.map(|s| s.to_string()).to_vec(),
            at_least: 2,
            timeout: Duration::from_secs(5),
        }
    }
}

pub const CALENDARS: [&str; 4] = [
    "https://a.pool.opentimestamps.org/digest",
    "https://b.pool.opentimestamps.org/digest",
    "https://a.pool.eternitywall.com/digest",
    "https://ots.btc.catallaxy.com/digest",
];
