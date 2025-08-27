#![cfg_attr(docsrs, feature(doc_auto_cfg))]
#![cfg_attr(not(test), no_std)]
#![deny(unsafe_code)]
#![warn(missing_docs)]
#![cfg_attr(not(doctest), doc = include_str!("../README.md"))]

pub mod datetime;
mod ds3231;
pub mod error;
pub mod registers;
pub mod square_wave;

// Re-export Ds3231
pub use ds3231::Ds3231;

// Re-export RTC HAL
pub use rtc_hal::{datetime::DateTime, rtc::Rtc};
