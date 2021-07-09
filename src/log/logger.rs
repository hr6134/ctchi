use log::{Record, Level, Metadata, SetLoggerError, LevelFilter};
use chrono::{Datelike, Timelike, Utc};

use std::fs::{File, OpenOptions};
use std::io::prelude::*;
use std::path::Path;

use crate::core::config::get_configuration;

struct SimpleLogger;

impl log::Log for SimpleLogger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        metadata.level() <= Level::Debug
    }

    fn log(&self, record: &Record) {
        if self.enabled(record.metadata()) {
            let log_message = format!(
                "{}: {} - {}\n",
                Utc::now(),
                record.level(),
                record.args()
            );
            println!("{}", log_message);

            let config_reader = get_configuration();
            let config = config_reader.inner.lock().unwrap();
            let log_path = config.log_path.to_string();
            drop(config);

            // fixme remove unwrap
            let mut file = OpenOptions::new()
                .append(true)
                .create(true)
                .open(&log_path)
                .unwrap();

            file.write_all(log_message.as_bytes());
        }
    }

    fn flush(&self) {}
}


static LOGGER: SimpleLogger = SimpleLogger;

pub fn init() -> Result<(), SetLoggerError> {
    log::set_logger(&LOGGER)
        .map(|()| log::set_max_level(LevelFilter::Debug))
}