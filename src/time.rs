use nix::sys::time::TimeSpec;
use std::fmt;

/// Hardware timestamp
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct Timestamp {
    /// Seconds component
    pub seconds: i64,
    /// Nanoseconds component
    pub nanoseconds: i64,
}

impl From<TimeSpec> for Timestamp {
    #[allow(clippy::unnecessary_cast)]
    fn from(value: TimeSpec) -> Self {
        Timestamp {
            seconds: value.tv_sec() as i64,
            nanoseconds: value.tv_nsec() as i64,
        }
    }
}

impl fmt::Display for Timestamp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:010}.{:09}", self.seconds, self.nanoseconds)
    }
}
