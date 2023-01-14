# Geotime

Geotime provides a 128-bit signed integer timestamp compatible with Unix `time_t` and anchored at
the [Unix epoch](https://en.wikipedia.org/wiki/Unix_time). See the documentation [here](https://docs.rs/geotime/latest/geotime/).


```rust
use geotime::Geotime;

let dt = Geotime::from(0);
assert_eq!(dt.display_string("%Y-%m-%d"), "1970-01-01");

let dt = Geotime::from((i64::MAX as i128) + 1);
assert_eq!(dt.display_string("%Y"), "299.87 M years from now");

let dt = Geotime::from(-(i64::MAX as i128) * 100);
assert_eq!(dt.display_string("%Y"), "29.99 B years ago");
```
