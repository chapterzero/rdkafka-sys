use super::consumer::Consumer;
use super::producer::Producer;
use std::collections::HashMap;
use std::ffi::{CStr, CString};

use crate::bindings::{
    rd_kafka_conf_new, rd_kafka_conf_s, rd_kafka_conf_set, rd_kafka_new,
    rd_kafka_type_t_RD_KAFKA_CONSUMER, rd_kafka_type_t_RD_KAFKA_PRODUCER,
};

#[derive(Debug)]
pub struct Config {
    conf_map: HashMap<String, String>,
}

impl Config {
    pub fn new() -> Config {
        Config {
            conf_map: HashMap::new(),
        }
    }

    pub fn set(&mut self, name: &str, value: &str) -> &mut Self {
        self.conf_map
            .insert(String::from(name), String::from(value));
        self
    }

    pub fn build_consumer(mut self) -> Result<Consumer, ConfigError> {
        let conf = self.create_rdkafka_conf()?;
        unsafe {
            let err = CString::new(String::with_capacity(512)).unwrap().into_raw();
            let rk = rd_kafka_new(rd_kafka_type_t_RD_KAFKA_CONSUMER, conf, err, 512);
            let err = CStr::from_ptr(err).to_str().unwrap();
            if err.len() != 0 {
                return Err(ConfigError {
                    key: String::new(),
                    value: String::from("Creating kafka consumer object"),
                    reason: err.to_string(),
                });
            }
            Ok(Consumer::new(rk))
        }
    }

    pub fn build_producer(mut self) -> Result<Producer, ConfigError> {
        let conf = self.create_rdkafka_conf()?;
        unsafe {
            let err = CString::new(String::with_capacity(512)).unwrap().into_raw();
            let rk = rd_kafka_new(rd_kafka_type_t_RD_KAFKA_PRODUCER, conf, err, 512);
            let err = CStr::from_ptr(err).to_str().unwrap();
            if err.len() != 0 {
                return Err(ConfigError {
                    key: String::new(),
                    value: String::from("Creating kafka producer object"),
                    reason: err.to_string(),
                });
            }
            Ok(Producer::new(rk))
        }
    }

    fn create_rdkafka_conf(&mut self) -> Result<*mut rd_kafka_conf_s, ConfigError> {
        unsafe {
            let conf = rd_kafka_conf_new();
            for (k, v) in self.conf_map.drain() {
                let err = CString::new(String::with_capacity(512)).unwrap().into_raw();
                rd_kafka_conf_set(conf, cstr(&k).into_raw(), cstr(&v).into_raw(), err, 512);
                let err = CStr::from_ptr(err).to_str().unwrap();
                if err.len() != 0 {
                    return Err(ConfigError {
                        key: k,
                        value: v,
                        reason: err.to_string(),
                    });
                }
            }
            Ok(conf)
        }
    }
}

fn cstr(s: &str) -> CString {
    CString::new(s).unwrap()
}

use std::fmt;

#[derive(Debug, Clone)]
pub struct ConfigError {
    key: String,
    value: String,
    reason: String,
}

impl fmt::Display for ConfigError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Error when reading config")
    }
}

impl std::error::Error for ConfigError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        None
    }
}
