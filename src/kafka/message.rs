use crate::bindings::{
    rd_kafka_consumer_poll, rd_kafka_message_destroy, rd_kafka_message_t, rd_kafka_s,
};
use std::ffi::CStr;
use std::os::raw::c_char;

#[derive(Debug, Clone)]
pub struct Message {
    payload: String,
    partition: i32,
    offset: i64,
}

impl Message {
    fn from_kafka_msg(msg: *mut rd_kafka_message_t) -> Self {
        unsafe {
            let b = CStr::from_ptr((*msg).payload as *const c_char).to_bytes();
            let payload_len = (*msg).len as usize;
            Self {
                payload: String::from_utf8_lossy(&b[..payload_len]).to_string(),
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
                        Some(s) => Some(Err(MessageError { kafka_msg: s })),
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
pub struct MessageError {
    pub kafka_msg: String,
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
