pub mod config;
pub mod consumer;
pub mod producer;
pub mod message;

use crate::bindings::{rd_kafka_resp_err_t, rd_kafka_err2str};
use std::ffi::CStr;

pub fn get_error_str(code: rd_kafka_resp_err_t) -> Option<String> {
    if code == 0 {
        return None;
    }

    unsafe {
        Some(
            CStr::from_ptr(rd_kafka_err2str(code))
                .to_string_lossy()
                .to_string(),
        )
    }
}
