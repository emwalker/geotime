#![crate_type = "lib"]

#[macro_use]
extern crate quick_error;

use chrono::{DateTime, Utc};

mod ser;

quick_error! {
    #[derive(Clone, Debug)]
    pub enum Error {
        TryFromInt(err: std::num::TryFromIntError) {
            from()
        }

        DecodePartial(err: String) {
            from(err: data_encoding::DecodePartial) -> (format!("{:?}", err))
        }
    }
}

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Geotime(i128);

impl From<i32> for Geotime {
    fn from(n: i32) -> Self {
        Self(n.into())
    }
}

impl From<i64> for Geotime {
    fn from(n: i64) -> Self {
        Self(n.into())
    }
}

impl From<i128> for Geotime {
    fn from(n: i128) ->Geotime {
        Self(n)
    }
}

impl From<&DateTime<Utc>> for Geotime {
    fn from(dt: &DateTime<Utc>) -> Self {
        Self::from(dt.timestamp_millis())
    }
}

impl Geotime {
    pub fn timestamp_millis(&self) -> Result<i64> {
        Ok(self.0.try_into()?)
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
            let ut = Utc.ymd(1800, 1, 1).and_hms_milli(0, 0, 0, 0);
            let gt = Geotime::from(&ut);
            assert_eq!(ut.timestamp_millis(), gt.timestamp_millis().unwrap());
        }
    }
}
