use core::str::FromStr;

use crate::{
    duration::Duration,
    units::{self, TimeUnit},
    util::should_apply_plural,
};

/// Max allowed string length of the raw time unit or int value.
pub static MAX_DATA_CHUNK_LENGTH: usize = 32;

impl FromStr for Duration {
    type Err = error::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Duration::try_from(s)
    }
}

impl TryFrom<&str> for Duration {
    type Error = error::Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        use error::Error::*;

        if value.is_empty() {
            return Err(EmptyInput);
        }

        let mut result = Duration::default();

        let mut was_week_repeated = false;

        let mut it = value.chars().into_iter().enumerate().peekable();
        // "12hours34m56secs" - you're at '1', then at '3', then at '5', etc.
        while let Some((firstindex, firstc)) = it.next() {
            if !firstc.is_ascii_digit() {
                return Err(NumberExpected {
                    index: firstindex,
                    value: firstc,
                });
            }

            // scanning the value
            let mut value = String::from(firstc);

            while let Some((index, c)) = it.next_if(|(_i, c)| c.is_ascii_digit()) {
                if index - firstindex == MAX_DATA_CHUNK_LENGTH {
                    return Err(InputIsTooLong);
                }

                value.push(c);
            }
            let value = u64::from_str(&value).map_err(|e| ValueParseError(e))?;

            // scanning the time unit
            let secondc = it
                .next()
                .filter(|(_i, c)| c != &' ')
                .or_else(|| it.next())
                .ok_or(ValueWithoutUnit)?;

            let unit_first_index = secondc.0;
            let mut unit_last_index = usize::default();
            let mut unit = String::from(secondc.1);

            while let Some((index, c)) = it.next_if(|(_i, c)| c.is_ascii_alphabetic() || c == &'μ')
            {
                if index - unit_first_index == MAX_DATA_CHUNK_LENGTH {
                    return Err(InputIsTooLong);
                }

                unit.push(c);
                unit_last_index = index;
            }

            // matching unit with actual type
            macro_rules! supply_matcher {
                // timeunit, container
                ($(($tu:path, $c:expr)),+) => {
                    let unit_t = TimeUnit::from_str(&unit).map_err(|e| match e {
                        units::error::Error::UnknownUnit => error::Error::UnknownUnit {
                            start: unit_first_index,
                            end: unit_last_index,
                            input_unit: unit,
                            value,
                        },
                    })?;

                    match unit_t {
                        $(
                            $tu => {
                                if $c != u64::default() {
                                    return Err(TimeUnitRepeated {
                                        start: unit_first_index,
                                        end: unit_last_index,
                                        unit: $tu,
                                        value,
                                    });
                                }

                                $c = value;
                            }
                        )+
                        TimeUnit::Week => {
                            if was_week_repeated {
                                return Err(TimeUnitRepeated {
                                    start: unit_first_index,
                                    end: unit_last_index,
                                    unit: TimeUnit::Week,
                                    value,
                                });
                            }
                            was_week_repeated = true;

                            result.days += value * 7;
                        }
                        TimeUnit::Day => {
                            if result.days != u64::default() && was_week_repeated == false {
                                return Err(TimeUnitRepeated {
                                    start: unit_first_index,
                                    end: unit_last_index,
                                    unit: TimeUnit::Day,
                                    value,
                                });
                            }

                            result.days += value;
                        }
                    }
                };
            }

            supply_matcher!(
                (TimeUnit::Nanosecond, result.nanoseconds),
                (TimeUnit::Microsecond, result.microseconds),
                (TimeUnit::Millisecond, result.milliseconds),
                (TimeUnit::Second, result.seconds),
                (TimeUnit::Minute, result.minutes),
                (TimeUnit::Hour, result.hours),
                (TimeUnit::Month, result.months),
                (TimeUnit::Year, result.years)
            );

            // skip whitespace after unit
            it.next_if(|(_i, c)| c == &' ');
        }

        Ok(result)
    }
}

impl Duration {
    /// Formats the duration based on the provided options.
    #[rustfmt::skip]
    pub fn format(&self, options: &FormatterOptions) -> String {
        let mut string = String::with_capacity(3); // 3 as in "0ms".len()

        macro_rules! add_if_enabled {
            ($cond:expr, $var:expr, $short:literal, $long_singular:literal, $long_plural:literal) => {
                if $cond && !(!options.show_value_if_zero && $var == 0) {
                    string.push_str(&($var).to_string());

                    if options.long_unit_names {
                        string.push_str(if should_apply_plural($var) {
                            $long_plural
                        } else {
                            $long_singular
                        });
                    } else {
                        string.push_str($short);
                    }
                }
            };
        }

        add_if_enabled!(options.show_years, self.years, "y", " year ", " years ");
        add_if_enabled!(options.show_months, self.months, "mo", " month ", " months ");
        add_if_enabled!(options.show_days, self.days, "d", " day ", " days ");
        add_if_enabled!(options.show_hours, self.hours, "h", " hour ", " hours ");
        add_if_enabled!(options.show_minutes, self.minutes, "m", " minute ", " minutes ");
        add_if_enabled!(options.show_seconds, self.seconds, "s", " second ", " seconds ");
        add_if_enabled!(options.show_milliseconds, self.milliseconds, "ms", " millisecond ", " milliseconds ");
        add_if_enabled!(options.show_microseconds, self.microseconds, "μs", " microsecond ", " microseconds ");
        add_if_enabled!(options.show_nanoseconds, self.nanoseconds, "ns", " nanosecond "," nanoseconds ");

        string.chars().last().inspect(|ch| {
            if ch == &' ' {
                string.pop();
            }
        });

        if string.is_empty() {
            return (if options.long_unit_names {
                "0 nanoseconds"
            } else {
                "0ns"
            })
            .to_owned();
        }

        string
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct FormatterOptions {
    /// `true` by default
    pub show_nanoseconds: bool,

    /// `true` by default
    pub show_microseconds: bool,

    /// `true` by default
    pub show_milliseconds: bool,

    /// `true` by default
    pub show_seconds: bool,

    /// `true` by default
    pub show_minutes: bool,

    /// `true` by default
    pub show_hours: bool,

    /// `true` by default
    pub show_days: bool,

    /// `true` by default
    pub show_months: bool,

    /// `true` by default
    pub show_years: bool,

    /// Whether to use long unit names (ex: "seconds") or short unit names (ex: "s").
    ///
    /// `false` by default
    pub long_unit_names: bool,

    /// Whether to include units with a value of zero in the formatted string.
    ///
    /// `false` by default
    pub show_value_if_zero: bool,
}

impl Default for FormatterOptions {
    fn default() -> Self {
        FormatterOptions {
            show_nanoseconds: true,
            show_microseconds: true,
            show_milliseconds: true,
            show_seconds: true,
            show_minutes: true,
            show_hours: true,
            show_days: true,
            show_months: true,
            show_years: true,

            long_unit_names: false,
            show_value_if_zero: false,
        }
    }
}

pub mod error {
    use crate::units::TimeUnit;

    #[derive(Debug, PartialEq, Clone)]
    pub enum Error {
        NumberExpected {
            /// The character that was found instead of a number.
            value: char,

            /// The index at which the error occurred.
            index: usize,
        },

        /// Unknown time unit was provided.
        UnknownUnit {
            /// The starting index of the unknown unit in the input string.
            start: usize,

            /// The ending index of the unknown unit in the input string.
            end: usize,

            /// The unknown unit that was provided.
            input_unit: String,

            /// The value associated with the unknown unit.
            value: u64,
        },

        /// A time unit was repeated in the input.
        TimeUnitRepeated {
            /// The starting index of the repeated unit in the input string.
            start: usize,

            /// The ending index of the repeated unit in the input string.
            end: usize,

            /// The time unit that was repeated.
            unit: TimeUnit,

            /// The value associated with the repeated unit.
            value: u64,
        },

        /// Input time unit or value is too long.
        InputIsTooLong,

        /// A value was provided without a corresponding time unit.
        ValueWithoutUnit,

        /// A value cannot be parsed as an integer.
        ValueParseError(core::num::ParseIntError),

        /// Input string is empty.
        EmptyInput,
    }

    impl core::error::Error for Error {}

    impl core::fmt::Display for Error {
        #[rustfmt::skip]
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            use Error::*;

            match self {
                NumberExpected { value, index } =>
                    write!(f, "expected number at index {index} but received {value}"),

                UnknownUnit { input_unit, value, .. } =>
                    write!(f, r#"unknown time unit "{input_unit}" was provided, assigned number for it was {value}"#),

                TimeUnitRepeated { unit, .. } => write!(f, "unit {unit} was provided 2x times or more"),

                InputIsTooLong =>
                    write!(f, "input time unit name or value was too long"),

                ValueWithoutUnit => write!(f, "value was provided but the time unit name was not"),

                ValueParseError(e) => write!(f, "got invalid int in the input, parse error: {e}"),

                EmptyInput => write!(f, "input is empty"),
            }
        }
    }
}

#[cfg(test)]
mod test {
    use core::str::FromStr;

    use crate::Duration;
    use crate::FormatterOptions;

    use super::MAX_DATA_CHUNK_LENGTH;

    #[test]
    fn from_str_normal() {
        let result =
            Duration::from_str("6yr3mo2d5h7m20s600ms200microsec80ns").expect("fail on valid input");
        let expected = Duration {
            years: 6,
            months: 3,
            days: 2,
            hours: 5,
            minutes: 7,
            seconds: 20,
            milliseconds: 600,
            microseconds: 200,
            nanoseconds: 80,
        };

        assert_eq!(result, expected);
    }

    #[test]
    fn from_str_with_spaces() {
        let result = Duration::from_str("3 days 2 hours 1 minute").expect("fail on valid input");

        let expected = Duration {
            days: 3,
            hours: 2,
            minutes: 1,
            ..Default::default()
        };

        assert_eq!(result, expected);
    }

    #[test]
    #[should_panic = "ValueParseError(ParseIntError { kind: PosOverflow })"]
    /// inner representation overflow
    fn from_str_larger_than_int_bounds() {
        Duration::from_str("99999999999999999999999999yrs").unwrap();
    }

    #[test]
    fn from_str_larger_non_ascii_matching() {
        let result = Duration::from_str("200μs").unwrap();
        let expected = Duration {
            microseconds: 200,
            ..Default::default()
        };

        assert_eq!(result, expected);
    }

    #[test]
    #[should_panic(expected = "EmptyInput")]
    fn from_str_input_zero() {
        Duration::from_str("").unwrap();
    }

    #[test]
    #[should_panic(expected = "InputIsTooLong")]
    fn from_str_too_large_value() {
        let n = "9".repeat(MAX_DATA_CHUNK_LENGTH + 1);

        Duration::from_str(&format!("{n}d")).unwrap();
    }

    #[test]
    #[should_panic(expected = "InputIsTooLong")]
    fn from_str_too_large_unit() {
        let n = "d".repeat(MAX_DATA_CHUNK_LENGTH + 1);

        Duration::from_str(&format!("9{n}")).unwrap();
    }

    #[test]
    #[should_panic(expected = "NumberExpected { value: 'd', index: 0 }")]
    /// input doesn't start with number
    fn from_str_unexpected_char() {
        Duration::from_str("dad2bm").unwrap();
    }

    #[test]
    #[should_panic(expected = r#"UnknownUnit { start: 2, end: 5, input_unit: "yays", value: 23 }"#)]
    /// "yays" is not a valid time unit
    fn from_str_unknown_unit() {
        Duration::from_str("23yays").unwrap();
    }

    #[test]
    #[should_panic(expected = "TimeUnitRepeated { start: 6, end: 7, unit: Month, value: 1 }")]
    fn from_string_time_unit_repeated() {
        Duration::from_str("2mo3h1mo5s").unwrap();
    }

    #[test]
    fn into_string() {
        let orig = "2d3h15m";
        let result = Duration::from_str(orig)
            .expect("fail on valid input")
            .format(&FormatterOptions::default());

        assert_eq!(orig, result)
    }

    #[test]
    fn into_string_with_long_unit_names() {
        let d =
            Duration::from_str("1yr2mo3w4d5h6m7s8ms9microsec10ns").expect("fail on valid input");
        let result = d.format(&FormatterOptions {
            long_unit_names: true,
            ..Default::default()
        });

        assert_eq!(result,
            "1 year 2 months 25 days 5 hours 6 minutes 7 seconds 8 milliseconds 9 microseconds 10 nanoseconds");
    }

    #[test]
    fn into_string_with_short_unit_names() {
        let d = Duration::from_str("1year2mo3weeks4d5h6m7s8ms9microsec10ns")
            .expect("fail on valid input");
        let result = d.format(&FormatterOptions {
            long_unit_names: false,
            ..Default::default()
        });

        assert_eq!(result, "1y2mo25d5h6m7s8ms9μs10ns");
    }

    #[test]
    fn into_string_no_nanoseconds() {
        let d =
            Duration::from_str("1yr2mo3w4d5h6m7s8ms9microsec10ns").expect("fail on valid input");
        let result = d.format(&FormatterOptions {
            long_unit_names: false,
            show_nanoseconds: false,
            ..Default::default()
        });

        assert_eq!(result, "1y2mo25d5h6m7s8ms9μs");
    }

    #[test]
    fn into_string_no_microseconds() {
        let d =
            Duration::from_str("1yr2mo3w4d5h6m7s8ms9microsec10ns").expect("fail on valid input");
        let result = d.format(&FormatterOptions {
            long_unit_names: false,
            show_microseconds: false,
            ..Default::default()
        });

        assert_eq!(result, "1y2mo25d5h6m7s8ms10ns");
    }

    #[test]
    fn into_string_no_milliseconds() {
        let d =
            Duration::from_str("1yr2mo3w4d5h6m7s8ms9microsec10ns").expect("fail on valid input");
        let result = d.format(&FormatterOptions {
            long_unit_names: false,
            show_milliseconds: false,
            ..Default::default()
        });

        assert_eq!(result, "1y2mo25d5h6m7s9μs10ns");
    }

    #[test]
    fn into_string_no_seconds() {
        let d =
            Duration::from_str("1yr2mo3w4d5h6m7s8ms9microsec10ns").expect("fail on valid input");
        let result = d.format(&FormatterOptions {
            long_unit_names: false,
            show_seconds: false,
            ..Default::default()
        });

        assert_eq!(result, "1y2mo25d5h6m8ms9μs10ns");
    }

    #[test]
    fn into_string_no_minutes() {
        let d =
            Duration::from_str("1yr2mo3w4d5h6m7s8ms9microsec10ns").expect("fail on valid input");
        let result = d.format(&FormatterOptions {
            long_unit_names: false,
            show_minutes: false,
            ..Default::default()
        });

        assert_eq!(result, "1y2mo25d5h7s8ms9μs10ns");
    }

    #[test]
    fn into_string_no_hours() {
        let d =
            Duration::from_str("1yr2mo3w4d5h6m7s8ms9microsec10ns").expect("fail on valid input");
        let result = d.format(&FormatterOptions {
            long_unit_names: false,
            show_hours: false,
            ..Default::default()
        });

        assert_eq!(result, "1y2mo25d6m7s8ms9μs10ns");
    }

    #[test]
    fn into_string_no_days() {
        let d =
            Duration::from_str("1yr2mo3w4d5h6m7s8ms9microsec10ns").expect("fail on valid input");
        let result = d.format(&FormatterOptions {
            long_unit_names: false,
            show_days: false,
            ..Default::default()
        });

        assert_eq!(result, "1y2mo5h6m7s8ms9μs10ns");
    }

    #[test]
    fn into_string_no_months() {
        let d =
            Duration::from_str("1yr2mo3w4d5h6m7s8ms9microsec10ns").expect("fail on valid input");
        let result = d.format(&FormatterOptions {
            long_unit_names: false,
            show_months: false,
            ..Default::default()
        });

        assert_eq!(result, "1y25d5h6m7s8ms9μs10ns");
    }

    #[test]
    fn into_string_no_years() {
        let d =
            Duration::from_str("1yr2mo3w4d5h6m7s8ms9microsec10ns").expect("fail on valid input");
        let result = d.format(&FormatterOptions {
            long_unit_names: false,
            show_years: false,
            ..Default::default()
        });

        assert_eq!(result, "2mo25d5h6m7s8ms9μs10ns");
    }
}
