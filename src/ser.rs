use hex;
use serde::{de, ser};
use std::fmt;

use crate::Geotime;

// See https://stackoverflow.com/a/11379574/61048
fn encode_lexical(n: i128) -> i128 {
    n ^ (1 << 127)
}

fn decode_lexical(n: i128) -> i128 {
    n ^ (1 << 127)
}

#[derive(Debug, Eq, PartialEq)]
pub struct LexicalHex(i128);

impl From<Geotime> for LexicalHex {
    fn from(ts: Geotime) -> Self {
        Self(ts.0)
    }
}

impl ser::Serialize for LexicalHex {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: ser::Serializer,
    {
        let v = encode_lexical(self.0);
        let s = hex::encode(v.to_be_bytes());
        serializer.serialize_str(&s)
    }
}

pub struct LexicalHexVisitor;

impl<'de> serde::de::Visitor<'de> for LexicalHexVisitor {
    type Value = LexicalHex;

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
        let v = decode_lexical(n);
        Ok(LexicalHex(v))
    }
}

impl<'de> de::Deserialize<'de> for LexicalHex {
    fn deserialize<D>(deserializer: D) -> Result<LexicalHex, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        deserializer.deserialize_string(LexicalHexVisitor)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use serde_test::{assert_tokens, Token};

    mod lexical_hex {
        use super::*;

        fn assert_serializes(n: i128, s: &'static str) {
            let ts = LexicalHex(n);
            assert_tokens(&ts, &[Token::Str(s)]);
        }

        #[test]
        fn serde() {
            assert_serializes(-100, "7fffffffffffffffffffffffffffff9c");
            assert_serializes(-1, "7fffffffffffffffffffffffffffffff");
            assert_serializes(0, "80000000000000000000000000000000");
            assert_serializes(1, "80000000000000000000000000000001");
            assert_serializes(100, "80000000000000000000000000000064");
        }
    }
}
