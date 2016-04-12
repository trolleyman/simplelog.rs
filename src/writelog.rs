// Copyright 2016 Victor Brekenfeld
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

//! Module providing the WriteLogger Implementation

use log::{LogLevel, LogLevelFilter, LogMetadata, LogRecord, SetLoggerError, set_logger, Log};
use time;
use std::io::Write;
use std::marker::Send;
use std::sync::Mutex;
use super::SharedLogger;

/// The WriteLogger struct. Provides a Write trait based Logger implementation
pub struct WriteLogger {
    level: LogLevelFilter,
    writer: Mutex<Box<Write + Send>>,
}

impl WriteLogger {
    /// init function. Globally initializes the WriteLogger as the one and only used log facility.
    ///
    /// Takes the desired `LogLevel` and boxed `Write` object as arguments. They cannot be changed later on.
    /// Fails if another Logger was already initialized.
    ///
    /// # Examples
    /// ```
    /// # extern crate simplelog;
    /// # use simplelog::*;
    /// # use std::net::TcpStream;
    /// # fn main() {
    /// let _ = WriteLogger::init(LogLevelFilter::Info, Box::new(TcpStream::connect("127.0.0.1:80").unwrap()));
    /// # }
    /// ```
    #[allow(dead_code)]
    pub fn init(log_level: LogLevelFilter, writer: Box<Write + Send>) -> Result<(), SetLoggerError> {
        set_logger(|max_log_level| {
            max_log_level.set(log_level.clone());
            WriteLogger::new(log_level, writer)
        })
    }
    
    /// Allows to create a new logger, that can be independently used, no matter whats globally set.
    ///
    /// No macros are provided for this case and you probably
    /// dont want to use this function unless you want to build a `CombinedLogger`.
    ///
    /// Takes the desired `LogLevel` and boxed `Write` object as arguments. They cannot be changed later on.
    ///
    /// # Examples
    /// ```
    /// # extern crate simplelog;
    /// # use simplelog::*;
    /// # fn main() {
    /// let mut out = Vec::new();
    /// let write_logger = WriteLogger::new(LogLevelFilter::Info, Box::new(&mut out)).unwrap());
    /// # }
    /// ```
    #[allow(dead_code)]
    pub fn new(log_level: LogLevelFilter, writer: Box<Write + Send>) -> Box<WriteLogger> {
        Box::new(WriteLogger { level: log_level, writer: Mutex::new(writer) })
    }
}

impl Log for WriteLogger {

    fn enabled(&self, metadata: &LogMetadata) -> bool {
        metadata.level() <= self.level
    }

    fn log(&self, record: &LogRecord) {
        if self.enabled(record.metadata()) {

            let mut lock = self.writer.lock().unwrap();

            let cur_time = time::now();

            let _ = match record.level() {
                LogLevel::Trace => {
                    writeln!(lock,
                        "{:02}:{:02}:{:02} [{}] {}: [{}:{}] {}",
                            cur_time.tm_hour,
                            cur_time.tm_min,
                            cur_time.tm_sec,
                            record.level(),
                            record.target(),
                            record.location().file(),
                            record.location().line(),
                            record.args()
                    ).unwrap();
                },
                _ => {
                    writeln!(lock,
                        "{:02}:{:02}:{:02} [{}] {}: {}",
                            cur_time.tm_hour,
                            cur_time.tm_min,
                            cur_time.tm_sec,
                            record.level(),
                            record.target(),
                            record.args()
                    ).unwrap();
                },
            };
        }
    }
}

impl SharedLogger for WriteLogger {

    fn level(&self) -> LogLevelFilter {
        self.level
    }

    fn as_log(self: Box<Self>) -> Box<Log> {
        Box::new(*self)
    }

}
