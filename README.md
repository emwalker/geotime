# Geotime

Geotime provides a 128-bit signed integer timestamp compatible with Unix `time_t` and anchored at the [Unix epoch](https://en.wikipedia.org/wiki/Unix_time).

A 128-bit timestamp allows one to represent times of events in geological, historical and present-day time to millisecond precision.  We go down to milliseconds as a convenience for handling timestamps for recent events.  In order to maintain a clean mapping to Unix timestamps, we inherit whatever is going on with leap seconds.  Timestamps can represent any date within +- 5e27 years of 1970.

## Display strings

A simple date formatting method is provided to render the timestamps in a human-friendly string.  If a timestamp is too large for `chrono` to render using the template provided, we fall back to the [human_format crate](https://docs.rs/human_format/latest/human_format/).  If the timestamp is unsafe for `human_format` to render, we fall back to the debug format.

```rust
let ts = Geotime::from(0);
assert_eq!(ts.display_string("%Y"), "1970");
assert_eq!(ts.display_string("%Y-%m"), "1970-01");
assert_eq!(ts.display_string("%Y-%m-%d"), "1970-01-01");

let ts = Geotime::from((i32::MAX as i128) * 1000);
assert_eq!(ts.display_string("%Y-%m-%d"), "2038-01-19");

let ts = Geotime::from((i64::MAX as i128) + 1);
assert_eq!(ts.display_string("%Y"), "299.87 M years from now");

let ts = Geotime::from(-(i64::MAX as i128) * 100);
assert_eq!(ts.display_string("%Y"), "29.99 B years ago");

let ts = Geotime::from(((MAX_YEARS - 1.0) as i128) * MILLISECONDS_IN_YEAR_APPROX);
assert_eq!(ts.display_string("%Y"), "1000.00 B years from now");

let ts = Geotime::from(-((MAX_YEARS - 1.0) as i128) * MILLISECONDS_IN_YEAR_APPROX);
assert_eq!(ts.display_string("%Y"), "1000.00 B years ago");

let ts = Geotime::from(-i128::MAX - 1);
assert_eq!(
    ts.display_string("%Y"),
    "Geotime(-170141183460469231731687303715884105728) ms ago"
);
```

## Serialization formats

Several structs are provided for serializing timestamps to strings, shown in the tables below.  Offsets are milliseconds from January 1, 1970. In each format, lexical ordering of the encoded timestamps is perserved.

### LexicalHex

| Offset | Serialization |
| ----- | ------------- |
| -10e21 | `7fffffffffffffc9ca36523a21600000` |
| -100 | `7fffffffffffffffffffffffffffff9c` |
| -1 | `7fffffffffffffffffffffffffffffff` |
| 0 | `80000000000000000000000000000000` |
| 1 | `80000000000000000000000000000001` |
| 100 | `80000000000000000000000000000064` |
| 10e21 | `800000000000003635c9adc5dea00000` |

### LexicalBase32HexNopad

| Offset | Serialization |
| ------ | ------------- |
| -10e21 | `FVVVVVVVVVVSJIHMA8T22O0000` |
| -100 | `FVVVVVVVVVVVVVVVVVVVVVVVJG` |
| -1 | `FVVVVVVVVVVVVVVVVVVVVVVVVS` |
| 0 | `G0000000000000000000000000` |
| 1 | `G0000000000000000000000004` |
| 100 | `G00000000000000000000000CG` |
| 10e21 | `G00000000003CDE9LN2TT80000` |

### LexicalGeohash

| Offset | Serialization |
| ------ | ------------- |
| -10e21 | `gzzzzzzzzzzwmkjqb8x22s0000` |
| -100 | `gzzzzzzzzzzzzzzzzzzzzzzzmh` |
| -1 | `gzzzzzzzzzzzzzzzzzzzzzzzzw` |
| 0 | `h0000000000000000000000000` |
| 1 | `h0000000000000000000000004` |
| 100 | `h00000000000000000000000dh` |
| 10e21 | `h00000000003def9pr2xx80000` |

### Lexical64

| Offset | Serialization |
| ------ | ------------- |
| -10e21 | `Uzzzzzzzzwb=C_8u8L0000` |
| -100 | `Uzzzzzzzzzzzzzzzzzzzb0` |
| -1 | `Uzzzzzzzzzzzzzzzzzzzzk` |
| 0 | `V000000000000000000000` |
| 1 | `V00000000000000000000F` |
| 100 | `V0000000000000000000O0` |
| 10e21 | `V000000003NpmPr5re0000` |
