#![crate_type = "lib"]

#[macro_use]
extern crate quick_error;
extern crate human_format;

use chrono::{DateTime, NaiveDateTime, Utc};
use human_format::Formatter;

const SECONDS_IN_DAY: i128 = 86400;
const MILLISECONDS_IN_YEAR_APPROX: i128 = SECONDS_IN_DAY * 356 * 1000;
const MAX_YEARS: f64 = 1000000000000.0;

mod ser;
pub use ser::{Lexical64, LexicalBase32HexNopad, LexicalGeohash, LexicalHex};

quick_error! {
    #[derive(Clone, Debug)]
    pub enum Error {
        Chrono(err: String) { }

        DecodePartial(err: String) {
            from(err: data_encoding::DecodePartial) -> (format!("{:?}", err))
        }

        TryFromInt(err: std::num::TryFromIntError) {
            from()
        }
    }
}

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Geotime(i128);

impl From<i32> for Geotime {
    fn from(n: i32) -> Self {
        Self::from(n as i128)
    }
}

impl From<i64> for Geotime {
    fn from(n: i64) -> Self {
        Self(n.into())
    }
}

impl From<i128> for Geotime {
    fn from(n: i128) -> Geotime {
        Self(n)
    }
}

impl From<&DateTime<Utc>> for Geotime {
    fn from(dt: &DateTime<Utc>) -> Self {
        Self::from(dt.timestamp_millis())
    }
}

impl Geotime {
    pub fn now() -> Self {
        Self::from(&Utc::now())
    }

    pub fn display_string(&self, format: &str) -> String {
        match DateTime::try_from(*self) {
            Ok(dt) => dt.format(format).to_string(),
            Err(_) => {
                let years = (self.0 as f64) / (MILLISECONDS_IN_YEAR_APPROX as f64);
                let past = years < 0.0;
                let years = years.abs();

                let (desc, unit) = if years < MAX_YEARS {
                    (Formatter::new().format(years), "years")
                } else {
                    (format!("{:?}", self), "ms")
                };

                if past {
                    format!("{} {} ago", desc, unit)
                } else {
                    format!("{} {} from now", desc, unit)
                }
            }
        }
    }

    pub fn timestamp_millis(&self) -> Result<i64> {
        Ok(self.0.try_into()?)
    }
}

impl TryFrom<Geotime> for DateTime<Utc> {
    type Error = Error;

    fn try_from(value: Geotime) -> std::result::Result<Self, Self::Error> {
        let n = i64::try_from(value.0)?;
        let (secs, nsecs) = (n / 1000, ((n % 1000) * 1000) as u32);
        let naive = NaiveDateTime::from_timestamp_opt(secs, nsecs).ok_or_else(|| Error::Chrono(
            "unable to convert to chrono::DateTime".to_string(),
        ))?;
        let dt: DateTime<Utc> = DateTime::from_utc(naive, Utc);
        Ok(dt)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod chrono {
        use super::*;
        use ::chrono::{TimeZone, Utc};

        #[test]
        fn from_chrono() {
            let dt = Utc.ymd(1800, 1, 1).and_hms_milli(0, 0, 0, 0);
            let ts = Geotime::from(&dt);
            assert_eq!(dt.timestamp_millis(), ts.timestamp_millis().unwrap());
        }

        #[test]
        fn to_chrono() {
            let ts = Geotime::from(0);
            let dt = Utc.ymd(1970, 1, 1).and_hms_milli(0, 0, 0, 0);
            assert_eq!(DateTime::try_from(ts).unwrap(), dt);
        }

        #[test]
        fn now() {
            assert!(Geotime::now() > Geotime::from(0));
        }

        #[test]
        fn display_string() {
            let ts = Geotime::from(0);
            assert_eq!(ts.display_string("%Y"), "1970");
            assert_eq!(ts.display_string("%Y-%m"), "1970-01");
            assert_eq!(ts.display_string("%Y-%m-%d"), "1970-01-01");

            let ts = Geotime::from((i32::MAX as i128) * 1000);
            assert_eq!(ts.display_string("%Y-%m-%d"), "2038-01-19");

            let ts = Geotime::from((i64::MAX as i128) + 1);
            assert_eq!(ts.display_string("%Y"), "299.87 M years from now");

            let ts = Geotime::from(-(i64::MAX as i128) - 1);
            assert_eq!(ts.display_string("%Y"), "299.87 M years ago");

            let ts = Geotime::from(-(i64::MAX as i128) * 100);
            assert_eq!(ts.display_string("%Y"), "29.99 B years ago");

            let ts = Geotime::from((i64::MAX as i128) * 100);
            assert_eq!(ts.display_string("%Y"), "29.99 B years from now");

            let ts = Geotime::from(((MAX_YEARS - 1.0) as i128) * MILLISECONDS_IN_YEAR_APPROX);
            assert_eq!(ts.display_string("%Y"), "1000.00 B years from now");

            let ts = Geotime::from(-((MAX_YEARS - 1.0) as i128) * MILLISECONDS_IN_YEAR_APPROX);
            assert_eq!(ts.display_string("%Y"), "1000.00 B years ago");

            let ts = Geotime::from(-i128::MAX - 1);
            assert_eq!(
                ts.display_string("%Y"),
                "Geotime(-170141183460469231731687303715884105728) ms ago"
            );
        }
    }
}
