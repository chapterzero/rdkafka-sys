use crate::bindings::{
    rd_kafka_destroy, rd_kafka_flush, rd_kafka_last_error, rd_kafka_poll, rd_kafka_produce,
    rd_kafka_s, rd_kafka_topic_conf_new, rd_kafka_topic_destroy, rd_kafka_topic_new,
    rd_kafka_topic_t, RD_KAFKA_MSG_F_FREE,
};
use std::ffi::{c_void, CString};
use std::ptr;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Producer {
    rk: *mut rd_kafka_s,
    rkt: HashMap<String, *mut rd_kafka_topic_t>,
}

impl Producer {
    pub fn new(rk: *mut rd_kafka_s) -> Producer {
        Producer {
            rk,
            rkt: HashMap::new(),
        }
    }

    pub fn set_topics(&mut self, topics: &[&str]) {
        unsafe {
            for topic_name in topics {
                if self.rkt.contains_key(*topic_name) {
                    continue;
                }

                let topic = CString::new(String::from(*topic_name)).unwrap().into_raw();
                let topic_conf = rd_kafka_topic_conf_new();
                self.rkt.insert(
                    topic_name.to_string(),
                    rd_kafka_topic_new(self.rk, topic, topic_conf),
                );
            }
        }
    }

    pub fn send(&self, payload: &[u8], topic: &str, partition: Option<i32>) -> Result<(), ProducerError> {
        let rkt = match self.rkt.get(topic) {
            None => return Err(ProducerError::UnregisteredTopic),
            Some(v) => v,
        };

        unsafe {
            let mut payload = payload.to_vec();
            let res = rd_kafka_produce(
                *rkt,
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

impl Drop for Producer {
    fn drop(&mut self) {
        unsafe {
            // destroy producer topics
            let keys: Vec<String> = self.rkt.keys().map(|s| s.to_string()).collect();
            for key in keys {
                let rkt = self.rkt.remove(&key).unwrap();
                rd_kafka_topic_destroy(rkt);
            }

            rd_kafka_destroy(self.rk);
        }
    }
}

#[derive(Debug, Clone)]
pub enum ProducerError {
    UnregisteredTopic,
    SendError(String),
}

use std::fmt;
impl fmt::Display for ProducerError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Kafka Producer Error")
    }
}

impl std::error::Error for ProducerError {}

unsafe impl Send for Producer {}
unsafe impl Sync for Producer {}
