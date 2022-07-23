# Geotime

Geotime provides a 128-bit signed integer timestamp compatible with Unix `time_t` and anchored at the [Unix epoch](https://en.wikipedia.org/wiki/Unix_time).

A 128-bit timestamp allows one to represent times of events in geological, historical and present-day time to millisecond precision.  We go down to milliseconds as a convenience for handling timestamps for recent events.  In order to maintain a clean mapping to Unix timestamps, we inherit whatever is going on with leap seconds.
