use crate::bindings::{
    rd_kafka_flush, rd_kafka_last_error, rd_kafka_poll, rd_kafka_produce, rd_kafka_s,
    rd_kafka_topic_conf_new, rd_kafka_topic_destroy, rd_kafka_topic_new, rd_kafka_topic_t,
    RD_KAFKA_MSG_F_FREE,
};
use std::ffi::c_void;
use std::ffi::CString;
use std::ptr;

#[derive(Debug, Clone)]
pub struct Producer {
    rk: *mut rd_kafka_s,
    rkt: *mut rd_kafka_topic_t,
}

impl Producer {
    pub fn new(rk: *mut rd_kafka_s) -> Producer {
        Producer {
            rk,
            rkt: ptr::null_mut(),
        }
    }

    pub fn set_topic(&mut self, topic: &str) {
        unsafe {
            if !self.rkt.is_null() {
                rd_kafka_topic_destroy(self.rkt);
            }
            let topic = CString::new(String::from(topic)).unwrap().into_raw();
            let topic_conf = rd_kafka_topic_conf_new();
            self.rkt = rd_kafka_topic_new(self.rk, topic, topic_conf);
        }
    }

    pub fn send(&self, payload: &[u8], partition: Option<i32>) -> Result<(), ProducerError> {
        if self.rkt.is_null() {
            return Err(ProducerError::NoTopic);
        }

        unsafe {
            let mut payload = payload.to_vec();
            let res = rd_kafka_produce(
                self.rkt,
                partition.unwrap_or(-1),
                RD_KAFKA_MSG_F_FREE as i32,
                payload.as_mut_ptr() as *mut c_void,
                payload.len() as u64,
                ptr::null(),
                0,
                ptr::null_mut(),
            );
            if res == -1 {
                if let Some(err) = super::get_error_str(rd_kafka_last_error()) {
                    return Err(ProducerError::SendError(err));
                }
            }

            std::mem::forget(payload);
        }

        Ok(())
    }

    pub fn poll(&self) {
        unsafe {
            rd_kafka_poll(self.rk, 0);
        }
    }

    pub fn flush(&self, timeout_ms: i32) {
        unsafe {
            rd_kafka_flush(self.rk, timeout_ms);
        }
    }
}

#[derive(Debug, Clone)]
pub enum ProducerError {
    NoTopic,
    SendError(String),
}

use std::fmt;
impl fmt::Display for ProducerError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Kafka Producer Error")
    }
}

impl std::error::Error for ProducerError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        None
    }
}

unsafe impl Send for Producer {}
unsafe impl Sync for Producer {}
