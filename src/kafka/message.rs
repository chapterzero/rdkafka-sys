use crate::bindings::{
    rd_kafka_consumer_poll, rd_kafka_message_destroy, rd_kafka_message_t, rd_kafka_s,
};

#[derive(Debug, Clone)]
pub struct Message {
    pub payload: Vec<u8>,
    pub partition: i32,
    pub offset: i64,
}

impl Message {
    pub fn read(&self) -> Result<&str, std::str::Utf8Error> {
        std::str::from_utf8(&self.payload)
    }

    fn from_kafka_msg(msg: *mut rd_kafka_message_t) -> Self {
        unsafe {
            let payload_len = (*msg).len as usize;
            let b = std::slice::from_raw_parts((*msg).payload as *const u8, payload_len);
            Self {
                payload: b.to_vec(),
                partition: (*msg).partition,
                offset: (*msg).offset,
            }
        }
    }
}

pub struct Messages {
    rk: *mut rd_kafka_s,
    poll_timeout_ms: i32,
}

impl Messages {
    pub fn new(rk: *mut rd_kafka_s, poll_timeout_ms: i32) -> Messages {
        Messages {
            rk,
            poll_timeout_ms,
        }
    }
}

impl Iterator for Messages {
    type Item = Result<Message, MessageError>;

    fn next(&mut self) -> Option<Self::Item> {
        unsafe {
            let msg = rd_kafka_consumer_poll(self.rk, self.poll_timeout_ms);
            match msg.is_null() {
                true => None,
                false => {
                    let res = match super::get_error_str((*msg).err) {
                        Some(s) => Some(Err(MessageError::KafkaError(s))),
                        None => Some(Ok(Message::from_kafka_msg(msg))),
                    };
                    rd_kafka_message_destroy(msg);
                    res
                }
            }
        }
    }
}

use std::fmt;
#[derive(Debug, Clone)]
pub enum MessageError {
    KafkaError(String),
}

impl fmt::Display for MessageError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Kafka Message Error")
    }
}

impl std::error::Error for MessageError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        None
    }
}
