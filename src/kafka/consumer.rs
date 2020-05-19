use super::message::Messages;
use crate::bindings::{
    rd_kafka_consumer_close, rd_kafka_destroy, rd_kafka_s, rd_kafka_subscribe,
    rd_kafka_topic_partition_list_add, rd_kafka_topic_partition_list_destroy,
    rd_kafka_topic_partition_list_new, rd_kafka_topic_partition_list_t,
};

use std::ffi::CString;

#[derive(Debug, Clone)]
pub struct Consumer {
    rk: *mut rd_kafka_s,
    topic_partition: *mut rd_kafka_topic_partition_list_t,
}

impl Consumer {
    pub fn new(rk: *mut rd_kafka_s) -> Consumer {
        Consumer {
            rk,
            topic_partition: std::ptr::null_mut(),
        }
    }

    pub fn subscribe(&mut self, topics: &[&str]) {
        unsafe {
            self.destroy_topic_partition();

            let top_par = rd_kafka_topic_partition_list_new(topics.len() as i32);
            for topic in topics {
                let topic = CString::new(String::from(*topic)).unwrap().into_raw();
                rd_kafka_topic_partition_list_add(top_par, topic, -1);
            }

            rd_kafka_subscribe(self.rk, top_par);
            self.topic_partition = top_par;
        }
    }

    pub fn get_messages(&self, poll_timeout_ms: i32) -> Result<Messages, ConsumerError> {
        Ok(Messages::new(self.rk, poll_timeout_ms))
    }

    fn close(&self) -> Result<(), ConsumerError> {
        unsafe {
            self.destroy_topic_partition();
            let err_code = rd_kafka_consumer_close(self.rk);
            if let Some(err) = super::get_error_str(err_code) {
                return Err(ConsumerError::CloseError(err));
            }
            Ok(())
        }
    }

    fn destroy_topic_partition(&self) {
        if !self.topic_partition.is_null() {
            unsafe { rd_kafka_topic_partition_list_destroy(self.topic_partition) }
        }
    }
}

impl Drop for Consumer {
    fn drop(&mut self) {
        if let Err(err) = self.close() {
            eprintln!("Error when closing kafka consumer: {} {:?}", err, err);
        }

        unsafe {
            rd_kafka_destroy(self.rk);
        }
    }
}

use std::fmt;
#[derive(Debug, Clone)]
pub enum ConsumerError {
    CloseError(String),
}

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

unsafe impl Send for Consumer {}
unsafe impl Sync for Consumer {}
