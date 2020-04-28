use crate::bindings::{
    rd_kafka_consume_start, rd_kafka_s, rd_kafka_topic_conf_new, rd_kafka_topic_new,
    rd_kafka_topic_s, rd_kafka_poll, rd_kafka_consume, rd_kafka_last_error,rd_kafka_err2str, rd_kafka_brokers_add,
};
use std::ffi::{CString, CStr};
use std::os::raw::c_char;
use std::ptr;

#[derive(Debug)]
pub struct Consumer {
    rk: *mut rd_kafka_s,
    rkt: *mut rd_kafka_topic_s,
}

impl Consumer {
    pub fn new(rk: *mut rd_kafka_s) -> Consumer {
        Consumer {
            rk,
            rkt: ptr::null_mut(),
        }
    }

    pub fn subscribe(&mut self, topic: &str) {
        unsafe {
            let topic = CString::new(String::from(topic)).unwrap().into_raw();
            let topic_conf = rd_kafka_topic_conf_new();
            let rkt = rd_kafka_topic_new(self.rk, topic, topic_conf);
            self.rkt = rkt;
        }
    }

    pub fn run(&self) -> Result<(), ConsumerError> {
        if self.rkt.is_null() {
            return Err(ConsumerError::NoTopicSubscribed);
        }

        unsafe {
            let broker = CString::new(String::from("127.0.0.1:9092")).unwrap().into_raw();
            if rd_kafka_brokers_add(self.rk, broker) == 0 {
                return Err(ConsumerError::InvalidBroker)
            }
            if rd_kafka_consume_start(self.rkt, 0, 0) == -1 {
                let err = rd_kafka_last_error();
                let err = rd_kafka_err2str(err);
                println!("{}", CStr::from_ptr(err).to_str().unwrap());
                return Err(ConsumerError::StartError);
            }

            loop {
                rd_kafka_poll(self.rk, 0);
                let msg = rd_kafka_consume(self.rkt, 0, 1000);
                if msg.is_null() {
                    println!("No message");
                    continue
                }
                // let len = msg.len;
                // let b: &[u8] = std::slice::from_raw_parts(msg);
                let msg = CStr::from_ptr((*msg).payload as *const c_char).to_str().unwrap();
                println!("Got message {} ", msg);
            }
        }

        Ok(())
    }
}

#[derive(Debug)]
pub enum ConsumerError {
    InvalidBroker,
    NoTopicSubscribed,
    StartError,
}
