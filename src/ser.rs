use data_encoding::{Encoding, BASE32HEX_NOPAD};
use data_encoding_macro::new_encoding;
use serde::{de, ser};
use std::fmt;

use crate::{Error, Geotime};

fn lexify(n: i128) -> i128 {
    n ^ (1 << 127)
}

fn delexify(n: i128) -> i128 {
    n ^ (1 << 127)
}

/// Hex encoding that can be lexically sorted.
///
/// ```
/// use geotime::{Geotime, Lexical16};
///
/// let dt: Lexical16 = Geotime::from(0).into();
/// assert_eq!(dt.to_string(), "80000000000000000000000000000000");
/// ```
///
/// For offsets in milliseconds from 1970:
///
/// | Offset | Serialization                      |
/// |--------|------------------------------------|
/// | -10e21 | `7fffffffffffffc9ca36523a21600000` |
/// | -100   | `7fffffffffffffffffffffffffffff9c` |
/// | -1     | `7fffffffffffffffffffffffffffffff` |
/// | 0      | `80000000000000000000000000000000` |
/// | 1      | `80000000000000000000000000000001` |
/// | 100    | `80000000000000000000000000000064` |
/// | 10e21  | `800000000000003635c9adc5dea00000` |
///
#[derive(Debug, Eq, PartialEq)]
pub struct Lexical16(i128);

impl From<Geotime> for Lexical16 {
    fn from(ts: Geotime) -> Self {
        Self(ts.0)
    }
}

impl From<Lexical16> for Geotime {
    fn from(ts: Lexical16) -> Self {
        Self(ts.0)
    }
}

impl ser::Serialize for Lexical16 {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: ser::Serializer,
    {
        let v = lexify(self.0);
        let s = hex::encode(v.to_be_bytes());
        serializer.serialize_str(&s)
    }
}

impl std::fmt::Display for Lexical16 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use serde::Serialize;
        self.serialize(f)
    }
}

struct Lexical16Visitor;

impl<'de> serde::de::Visitor<'de> for Lexical16Visitor {
    type Value = Lexical16;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("a Lexical16-encoded i128 value")
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
        Ok(Lexical16(v))
    }
}

impl<'de> de::Deserialize<'de> for Lexical16 {
    fn deserialize<D>(deserializer: D) -> Result<Lexical16, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        deserializer.deserialize_string(Lexical16Visitor)
    }
}

/// Base 32 encoding that can be lexically sorted.
///
/// ```
/// use geotime::{Geotime, Lexical32};
///
/// let dt: Lexical32 = Geotime::from(0).into();
/// assert_eq!(dt.to_string(), "G0000000000000000000000000");
/// ```
///
/// For offsets in milliseconds from 1970:
///
/// | Offset | Serialization                |
/// |--------|------------------------------|
/// | -10e21 | `FVVVVVVVVVVSJIHMA8T22O0000` |
/// | -100   | `FVVVVVVVVVVVVVVVVVVVVVVVJG` |
/// | -1     | `FVVVVVVVVVVVVVVVVVVVVVVVVS` |
/// | 0      | `G0000000000000000000000000` |
/// | 1      | `G0000000000000000000000004` |
/// | 100    | `G00000000000000000000000CG` |
/// | 10e21  | `G00000000003CDE9LN2TT80000` |
///
#[derive(Debug, Eq, PartialEq)]
pub struct Lexical32(i128);

impl From<Geotime> for Lexical32 {
    fn from(ts: Geotime) -> Self {
        Self(ts.0)
    }
}

impl From<Lexical32> for Geotime {
    fn from(ts: Lexical32) -> Self {
        Self(ts.0)
    }
}

impl ser::Serialize for Lexical32 {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: ser::Serializer,
    {
        let v = lexify(self.0);
        let s = BASE32HEX_NOPAD.encode(&v.to_be_bytes());
        serializer.serialize_str(&s)
    }
}

impl std::fmt::Display for Lexical32 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use serde::Serialize;
        self.serialize(f)
    }
}

struct Lexical32Visitor;

impl<'de> serde::de::Visitor<'de> for Lexical32Visitor {
    type Value = Lexical32;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("a Lexical32Hex-encoded i128 value")
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        let input = v.as_bytes();
        let size = BASE32HEX_NOPAD
            .decode_len(input.len())
            .map_err(de::Error::custom)?;
        let mut output = vec![0; size];

        BASE32HEX_NOPAD
            .decode_mut(input, &mut output)
            .map_err(Error::from)
            .map_err(de::Error::custom)?;

        let mut b: [u8; 16] = Default::default();
        b.copy_from_slice(&output[0..16]);
        let n = i128::from_be_bytes(b);
        let v = delexify(n);
        Ok(Lexical32(v))
    }
}

impl<'de> de::Deserialize<'de> for Lexical32 {
    fn deserialize<D>(deserializer: D) -> Result<Lexical32, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        deserializer.deserialize_string(Lexical32Visitor)
    }
}

// See https://stackoverflow.com/a/11379574/61048
const GEOHASH: Encoding = new_encoding! {
    symbols: "0123456789bcdefghjkmnpqrstuvwxyz",
};

/// Geohash-like encoding that can be lexically sorted.
///
/// ```
/// use geotime::{Geotime, LexicalGeohash};
///
/// let dt: LexicalGeohash = Geotime::from(0).into();
/// assert_eq!(dt.to_string(), "h0000000000000000000000000");
/// ```
///
/// For offsets in milliseconds from 1970:
///
/// | Offset | Serialization                |
/// |--------|------------------------------|
/// | -10e21 | `gzzzzzzzzzzwmkjqb8x22s0000` |
/// | -100   | `gzzzzzzzzzzzzzzzzzzzzzzzmh` |
/// | -1     | `gzzzzzzzzzzzzzzzzzzzzzzzzw` |
/// | 0      | `h0000000000000000000000000` |
/// | 1      | `h0000000000000000000000004` |
/// | 100    | `h00000000000000000000000dh` |
/// | 10e21  | `h00000000003def9pr2xx80000` |
///
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct LexicalGeohash(i128);

impl From<Geotime> for LexicalGeohash {
    fn from(ts: Geotime) -> Self {
        Self(ts.0)
    }
}

impl From<LexicalGeohash> for Geotime {
    fn from(ts: LexicalGeohash) -> Self {
        Self(ts.0)
    }
}

impl ser::Serialize for LexicalGeohash {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: ser::Serializer,
    {
        let v = lexify(self.0);
        let s = GEOHASH.encode(&v.to_be_bytes());
        serializer.serialize_str(&s)
    }
}

impl std::fmt::Display for LexicalGeohash {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use serde::Serialize;
        self.serialize(f)
    }
}

struct LexicalGeohashVisitor;

impl<'de> serde::de::Visitor<'de> for LexicalGeohashVisitor {
    type Value = LexicalGeohash;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("a Lexical32Hex-encoded i128 value")
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        let input = v.as_bytes();
        let size = GEOHASH.decode_len(input.len()).map_err(de::Error::custom)?;
        let mut output = vec![0; size];

        GEOHASH
            .decode_mut(input, &mut output)
            .map_err(Error::from)
            .map_err(de::Error::custom)?;

        let mut b: [u8; 16] = Default::default();
        b.copy_from_slice(&output[0..16]);
        let n = i128::from_be_bytes(b);
        let v = delexify(n);
        Ok(LexicalGeohash(v))
    }
}

impl<'de> de::Deserialize<'de> for LexicalGeohash {
    fn deserialize<D>(deserializer: D) -> Result<LexicalGeohash, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        deserializer.deserialize_string(LexicalGeohashVisitor)
    }
}

// See https://stackoverflow.com/a/11379574/61048
const LEXICAL64: Encoding = new_encoding! {
    symbols: "0123456789=ABCDEFGHIJKLMNOPQRSTUVWXYZ_abcdefghijklmnopqrstuvwxyz",
};

/// Base 64 encoding that can be lexically sorted.
///
/// ```
/// use geotime::{Geotime, Lexical64};
///
/// let dt: Lexical64 = Geotime::from(0).into();
/// assert_eq!(dt.to_string(), "V000000000000000000000");
/// ```
///
/// For offsets in milliseconds from 1970:
///
/// | Offset | Serialization            |
/// |--------|--------------------------|
/// | -10e21 | `Uzzzzzzzzwb=C_8u8L0000` |
/// | -100   | `Uzzzzzzzzzzzzzzzzzzzb0` |
/// | -1     | `Uzzzzzzzzzzzzzzzzzzzzk` |
/// | 0      | `V000000000000000000000` |
/// | 1      | `V00000000000000000000F` |
/// | 100    | `V0000000000000000000O0` |
/// | 10e21  | `V000000003NpmPr5re0000` |
///
#[derive(Debug, Eq, PartialEq)]
pub struct Lexical64(i128);

impl From<Geotime> for Lexical64 {
    fn from(ts: Geotime) -> Self {
        Self(ts.0)
    }
}

impl From<Lexical64> for Geotime {
    fn from(ts: Lexical64) -> Self {
        Self(ts.0)
    }
}

impl ser::Serialize for Lexical64 {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: ser::Serializer,
    {
        let v = lexify(self.0);
        let s = LEXICAL64.encode(&v.to_be_bytes());
        serializer.serialize_str(&s)
    }
}

impl std::fmt::Display for Lexical64 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use serde::Serialize;
        self.serialize(f)
    }
}

struct Lexical64Visitor;

impl<'de> serde::de::Visitor<'de> for Lexical64Visitor {
    type Value = Lexical64;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("a Lexical32-encoded i128 value")
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        let input = v.as_bytes();
        let size = LEXICAL64
            .decode_len(input.len())
            .map_err(de::Error::custom)?;
        let mut output = vec![0; size];

        LEXICAL64
            .decode_mut(input, &mut output)
            .map_err(Error::from)
            .map_err(de::Error::custom)?;

        let mut b: [u8; 16] = Default::default();
        b.copy_from_slice(&output[0..16]);
        let n = i128::from_be_bytes(b);
        let v = delexify(n);
        Ok(Lexical64(v))
    }
}

impl<'de> de::Deserialize<'de> for Lexical64 {
    fn deserialize<D>(deserializer: D) -> Result<Lexical64, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        deserializer.deserialize_string(Lexical64Visitor)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use serde_test::{assert_tokens, Token};

    type Value = (i128, &'static str);

    fn assert_order_preserved(left: &[Value]) {
        let mut right = left.to_vec();
        right.sort_by_key(|k| k.1);
        assert_eq!(left, &right);
    }

    mod lexical_16 {
        use super::*;

        fn assert_serialize(values: &[Value]) {
            for (n, ser) in values {
                let ts = Lexical16(*n);
                assert_tokens(&ts, &[Token::Str(ser)]);
            }
            assert_order_preserved(values);
        }

        #[test]
        fn serde() {
            assert_serialize(&[
                (-i128::pow(10, 21), "7fffffffffffffc9ca36523a21600000"),
                (-100, "7fffffffffffffffffffffffffffff9c"),
                (-1, "7fffffffffffffffffffffffffffffff"),
                (0, "80000000000000000000000000000000"),
                (1, "80000000000000000000000000000001"),
                (100, "80000000000000000000000000000064"),
                (i128::pow(10, 21), "800000000000003635c9adc5dea00000"),
            ]);
        }
    }

    mod lexical_32 {
        use super::*;

        fn assert_serialize(values: &[Value]) {
            for (n, ser) in values {
                let ts = Lexical32(*n);
                assert_tokens(&ts, &[Token::Str(ser)]);
            }
            assert_order_preserved(values);
        }

        #[test]
        fn serde() {
            assert_serialize(&[
                (-i128::pow(10, 21), "FVVVVVVVVVVSJIHMA8T22O0000"),
                (-100, "FVVVVVVVVVVVVVVVVVVVVVVVJG"),
                (-1, "FVVVVVVVVVVVVVVVVVVVVVVVVS"),
                (0, "G0000000000000000000000000"),
                (1, "G0000000000000000000000004"),
                (100, "G00000000000000000000000CG"),
                (i128::pow(10, 21), "G00000000003CDE9LN2TT80000"),
            ]);
        }
    }

    mod lexical_geohash {
        use super::*;

        fn assert_serialize(values: &[Value]) {
            for (n, ser) in values {
                let ts = LexicalGeohash(*n);
                assert_tokens(&ts, &[Token::Str(ser)]);
            }
            assert_order_preserved(values);
        }

        #[test]
        fn serde() {
            assert_serialize(&[
                (-i128::pow(10, 21), "gzzzzzzzzzzwmkjqb8x22s0000"),
                (-100, "gzzzzzzzzzzzzzzzzzzzzzzzmh"),
                (-1, "gzzzzzzzzzzzzzzzzzzzzzzzzw"),
                (0, "h0000000000000000000000000"),
                (1, "h0000000000000000000000004"),
                (100, "h00000000000000000000000dh"),
                (i128::pow(10, 21), "h00000000003def9pr2xx80000"),
            ]);
        }
    }

    mod lexical_64 {
        use super::*;

        fn assert_serialize(values: &[Value]) {
            for (n, ser) in values {
                let ts = Lexical64(*n);
                assert_tokens(&ts, &[Token::Str(ser)]);
            }
            assert_order_preserved(values);
        }

        #[test]
        fn serde() {
            assert_serialize(&[
                (-i128::pow(10, 21), "Uzzzzzzzzwb=C_8u8L0000"),
                (-100, "Uzzzzzzzzzzzzzzzzzzzb0"),
                (-1, "Uzzzzzzzzzzzzzzzzzzzzk"),
                (0, "V000000000000000000000"),
                (1, "V00000000000000000000F"),
                (100, "V0000000000000000000O0"),
                (i128::pow(10, 21), "V000000003NpmPr5re0000"),
            ]);
        }
    }
}
