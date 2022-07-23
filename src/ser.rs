use serde::{de, ser};
use std::fmt;

use crate::Geotime;

impl ser::Serialize for Geotime {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: ser::Serializer,
    {
        let s = hex::encode(self.0.to_be_bytes());
        serializer.serialize_str(&s)
    }
}

pub struct GeotimeVisitor;

impl<'de> serde::de::Visitor<'de> for GeotimeVisitor {
    type Value = Geotime;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("a hex-encoded integer between -2^127 and 2^127")
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        let bytes = hex::decode(v).map_err(de::Error::custom)?;
        let mut b: [u8; 16] = Default::default();
        b.copy_from_slice(&bytes[0..16]);
        let n = i128::from_be_bytes(b);
        Ok(Geotime::from(n))
    }
}

impl<'de> de::Deserialize<'de> for Geotime {
    fn deserialize<D>(deserializer: D) -> Result<Geotime, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        deserializer.deserialize_string(GeotimeVisitor)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use serde_test::{assert_tokens, Token};

    #[test]
    fn serialization() {
        let gt = Geotime::from(-100);
        assert_tokens(&gt, &[Token::Str("ffffffffffffffffffffffffffffff9c")]);
    }
}
