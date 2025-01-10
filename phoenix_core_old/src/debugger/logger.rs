use std::sync::mpsc::{self, Sender};
use std::thread;
use chrono;

#[derive(Debug)]
pub enum LogLevel {
  Trace,
  Info,
  Warning,
  Error
}

pub struct LogMessage {
  level: LogLevel,
  timestamp: String,
  message: String,
  file: &'static str,
  line: u32,
}

pub struct Logger {
  sender: Sender<LogMessage>,
}

impl Logger {
  pub fn new() -> Logger {
    let (sender, receiver) = std::sync::mpsc::channel::<LogMessage>();
    std::thread::spawn(move || {
      while let Ok(log) = receiver.recv() {
        // Process messages
        println!("[{}][{:?}] {}", log.timestamp, log.level, log.message);
      }
    });

    Logger {
      sender
    }
  }

  pub fn log(&self, level: LogLevel, message: &str, file: &'static str, line: u32) {
    let timestamp = chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
    let log_message = LogMessage {
      level,
      timestamp,
      message: message.to_string(),
      file,
      line
    };
    let _ = self.sender.send(log_message);
  }
}

macro_rules! log {
  ($logger:expr, $level:expr, $msg:expr) => {
    $logger.log($level, $msg, file!(), line!());
  };
}

#[macro_export]
macro_rules! trace {
  ($logger:expr, $msg:expr) => {
    log!($logger, LogLevel::Trace, $msg);
  };
}

#[macro_export]
macro_rules! info {
  ($logger:expr, $msg:expr) => {
    log!($logger, LogLevel::Info, $msg);
  };
}

#[macro_export]
macro_rules! warn {
  ($logger:expr, $msg:expr) => {
    log!($logger, LogLevel::Warning, $msg);
  };
}

#[macro_export]
macro_rules! error {
  ($logger:expr, $msg:expr) => {
    log!($logger, LogLevel::Error, $msg);
  };
}
