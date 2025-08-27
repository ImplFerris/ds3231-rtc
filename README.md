# DS3231 RTC Driver

A Rust driver for the DS3231 Real-Time Clock (RTC) chip, implementing the embedded-hal and rtc-hal traits for integration with embedded Rust projects.

## Features

- Read and set date/time

## Basic usage

```rust
use ds3231_rtc::Ds3231;
use rtc_hal::rtc::Rtc;  // rtc_hal trait required to be imported to be used
use rtc_hal::datetime::DateTime;

// Set up I2C (depends on your board)
let i2c = /* your I2C setup */;

// Create the driver
let mut rtc = Ds3231::new(i2c);

// Set time to August 21, 2025 at 2:30 PM
let time = DateTime::new(2025, 8, 21, 14, 30, 0).unwrap();
rtc.set_datetime(&time).unwrap();

// Read current time
let now = rtc.get_datetime().unwrap();
```

## Examples

Example projects are available in the separate [ds3231-examples](https://github.com/implferris/ds3231-examples) repository to help you get started.


## License

This project is licensed under the MIT License.
