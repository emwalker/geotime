use data_encoding::BASE32HEX;

use serde::{de, ser};
use std::fmt;

use crate::{Error, Geotime};

// See https://stackoverflow.com/a/11379574/61048
fn lexify(n: i128) -> i128 {
    n ^ (1 << 127)
}

fn delexify(n: i128) -> i128 {
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
        let v = lexify(self.0);
        let s = hex::encode(v.to_be_bytes());
        serializer.serialize_str(&s)
    }
}

struct LexicalHexVisitor;

impl<'de> serde::de::Visitor<'de> for LexicalHexVisitor {
    type Value = LexicalHex;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("a LexicalHex-encoded i128 value")
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        let bytes = hex::decode(v).map_err(de::Error::custom)?;
        let mut b: [u8; 16] = Default::default();
        b.copy_from_slice(&bytes[0..16]);
        let n = i128::from_be_bytes(b);
        let v = delexify(n);
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

#[derive(Debug, Eq, PartialEq)]
pub struct LexicalBase32Hex(i128);

impl From<Geotime> for LexicalBase32Hex {
    fn from(ts: Geotime) -> Self {
        Self(ts.0)
    }
}

impl ser::Serialize for LexicalBase32Hex {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: ser::Serializer,
    {
        let v = lexify(self.0);
        let s = BASE32HEX.encode(&v.to_be_bytes());
        serializer.serialize_str(&s)
    }
}

struct Base32HexVisitor;

impl<'de> serde::de::Visitor<'de> for Base32HexVisitor {
    type Value = LexicalBase32Hex;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("a LexicalBase32Hex-encoded i128 value")
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        let input = v.as_bytes();
        let size = BASE32HEX
            .decode_len(input.len())
            .map_err(de::Error::custom)?;
        let mut output = vec![0; size];

        BASE32HEX
            .decode_mut(input, &mut output)
            .map_err(Error::from)
            .map_err(de::Error::custom)?;

        let mut b: [u8; 16] = Default::default();
        b.copy_from_slice(&output[0..16]);
        let n = i128::from_be_bytes(b);
        let v = delexify(n);
        Ok(LexicalBase32Hex(v))
    }
}

impl<'de> de::Deserialize<'de> for LexicalBase32Hex {
    fn deserialize<D>(deserializer: D) -> Result<LexicalBase32Hex, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        deserializer.deserialize_string(Base32HexVisitor)
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

    mod lexical_base32_hex {
        use super::*;

        fn assert_serializes(n: i128, s: &'static str) {
            let ts = LexicalBase32Hex(n);
            assert_tokens(&ts, &[Token::Str(s)]);
        }

        #[test]
        fn serde() {
            assert_serializes(-100, "FVVVVVVVVVVVVVVVVVVVVVVVJG======");
            assert_serializes(-1, "FVVVVVVVVVVVVVVVVVVVVVVVVS======");
            assert_serializes(0, "G0000000000000000000000000======");
            assert_serializes(1, "G0000000000000000000000004======");
            assert_serializes(100, "G00000000000000000000000CG======");
        }
    }
}
