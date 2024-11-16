//! * Parse human-readable strings like `15 years 5 weeks 2 hours` or `28m15s10ns` into `zuck::Duration`.
//! * Normalize the input via `zuck::Duration.normalize()` method.
//! * Convert `zuck::Duration` back into human-readable string.
//! * You can also convert from and into `std::time::Duration`.
//!
//! ## Aliases that can be used as input for parsing
//!
//! * Nanoseconds: `ns`, `nsec`, `nsecs`, `nanosec`, `nanosecs`, `nanosecond`, `nanoseconds`
//! * Microseconds: `μs`, `us`, `usec`, `usecs`, `microsec`, `microsecs`, `microsecond`, `microseconds`
//! * Milliseconds: `ms`, `msec`, `msecs`, `millisecond`, `milliseconds`
//! * Seconds: `s`, `sec`, `secs`, `second`, `seconds`
//! * Minutes: `m`, `min`, `mins`, `minute`, `minutes`
//! * Hours: `h`, `hr`, `hrs`, `hour`, `hours`
//! * Days: `d`, `day`, `days`
//! * Weeks: `w`, `wk`, `wks`, `week`, `weeks`
//! * Months: `mo`, `month`, `months`
//! * Years: `y`, `yr`, `yrs`, `year`, `years`

#![forbid(unsafe_code, non_ascii_idents)]
#![warn(
    missing_debug_implementations,
    clippy::exit,
    clippy::pattern_type_mismatch,
    clippy::exhaustive_enums
)]
#![cfg_attr(all(not(test), not(debug_assertions)), warn(missing_docs))]
#![cfg_attr(
    all(not(test), not(debug_assertions)),
    forbid(
        clippy::dbg_macro,
        clippy::print_stderr,
        clippy::print_stdout,
        clippy::expect_used,
        clippy::unwrap_used,
        unused_crate_dependencies,
    )
)]

mod duration;
mod formatter;
mod units;
mod util;

pub use duration::error::Error as DurationConversionError;
pub use duration::Duration;
pub use formatter::error::Error as FormatterError;
pub use formatter::FormatterOptions;

// Exported in case if a library consumer needs to perform their own checks somewhere.
pub use formatter::MAX_DATA_CHUNK_LENGTH;

pub mod unit {
    pub mod nanosecond {
        pub use crate::units::nanosecond::*;
    }
    pub mod second {
        pub use crate::units::second::*;
    }
}
