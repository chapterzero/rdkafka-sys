use super::message::Messages;
use crate::bindings::{
    rd_kafka_s, rd_kafka_subscribe, rd_kafka_topic_partition_list_add,
    rd_kafka_topic_partition_list_new,
};

use std::ffi::CString;

#[derive(Debug)]
pub struct Consumer {
    rk: *mut rd_kafka_s,
}

impl Consumer {
    pub fn new(rk: *mut rd_kafka_s) -> Consumer {
        Consumer { rk }
    }

    pub fn subscribe(&mut self, topics: &[&str]) {
        unsafe {
            let top_par = rd_kafka_topic_partition_list_new(topics.len() as i32);
            for topic in topics {
                let topic = CString::new(String::from(*topic)).unwrap().into_raw();
                rd_kafka_topic_partition_list_add(top_par, topic, -1);
            }
            rd_kafka_subscribe(self.rk, top_par);
        }
    }

    pub fn get_messages(&self, poll_timeout_ms: i32) -> Result<Messages, ConsumerError> {
        Ok(Messages::new(self.rk, poll_timeout_ms))
    }
}

use std::fmt;
#[derive(Debug, Clone)]
pub enum ConsumerError {}

impl fmt::Display for ConsumerError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Kafka Consumer Error")
    }
}

impl std::error::Error for ConsumerError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        None
    }
}
