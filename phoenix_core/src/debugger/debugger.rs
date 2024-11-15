use colored::{Colorize, ColoredString};
use std::fs::OpenOptions;
use std::io::Write;
use chrono::Local;

#[derive(Debug)]
pub enum PhoenixResult<T, E> {
  Data(T),
  Error(E), 
}

#[derive(PartialEq)]
pub enum DebuggerOutputMode {
  Terminal,
  File,
}

#[derive(PartialEq)]
pub enum DebuggerRunningMode {
  Off,
  Instant(DebuggerOutputMode),
  Accumulate(DebuggerOutputMode),
}

pub struct PhoenixDebugger<T, E> {
  mode: DebuggerRunningMode,
  stack: Option<Vec<PhoenixResult<T, E>>>,
  log_file_path: Option<String>,
}

impl<T, E> PhoenixDebugger<T, E>
where
  T: std::fmt::Debug,
  E: std::fmt::Debug,
{
  pub fn new(mode: DebuggerRunningMode) -> PhoenixDebugger<T, E> {
    let log_file_path = match mode {
      DebuggerRunningMode::Instant(DebuggerOutputMode::File)
      | DebuggerRunningMode::Accumulate(DebuggerOutputMode::File) => {
        Some(Self::create_log_file())
      }
      _ => None,
    };

    let stack = if let DebuggerRunningMode::Accumulate(_) = mode {
      Some(Vec::new())
    } else {
      None
    };

    PhoenixDebugger {
      mode,
      stack,
      log_file_path,
    }
  }

  pub fn get_mode(&mut self) -> &DebuggerRunningMode {
    &self.mode
  }

  pub fn set_mode(&mut self, mode: DebuggerRunningMode) {
    self.mode = mode;
  }

  fn log(&mut self, result: PhoenixResult<T, E>, level: ColoredString) {
    let message = format!("[{}]: {:?}", level, result);

    match self.mode {
      DebuggerRunningMode::Off => {}
      DebuggerRunningMode::Instant(DebuggerOutputMode::Terminal) => {
        println!("{}", message);
      }
      DebuggerRunningMode::Instant(DebuggerOutputMode::File) => {
        self.write_to_file(&message);
      }
      DebuggerRunningMode::Accumulate(DebuggerOutputMode::Terminal) => {
        if let Some(ref mut stack) = self.stack {
          stack.push(result);
        }
      }
      DebuggerRunningMode::Accumulate(DebuggerOutputMode::File) => {
        if let Some(ref mut stack) = self.stack {
          stack.push(result);
        }
      }
    }
  }

  pub fn error(&mut self, error: E) {
    self.log(PhoenixResult::Error(error), "ERROR".blue());
  }

  pub fn warning(&mut self, warning: T) {
    self.log(PhoenixResult::Data(warning), "WARNING".magenta());
  }

  pub fn info<S: std::fmt::Debug>(&mut self, info: S) {
    self.log(PhoenixResult::Data(info), "INFO".green());
  }

  pub fn trace(&mut self, trace: T) {
    self.log(PhoenixResult::Data(trace), "TRACE".blue());
  }

  pub fn flush(&mut self) {
    if let Some(ref stack) = self.stack {
      for entry in stack {
        let message = format!("{:?}", entry);
        match self.mode {
          DebuggerRunningMode::Accumulate(DebuggerOutputMode::Terminal) => {
            println!("{}", message);
          }
          DebuggerRunningMode::Accumulate(DebuggerOutputMode::File) => {
            self.write_to_file(&message);
          }
          _ => {}
        }
      }
    }
    self.stack = Some(Vec::new());
  }

  fn create_log_file() -> String {
    let dir = "./logs";
    std::fs::create_dir_all(dir).expect("Failed to create logs directory");

    let now = Local::now();
    let filename = format!("{}/log_{}.txt", dir, now.format("%Y-%m-%d_%H-%M-%S"));
    filename
  }

  fn write_to_file(&self, message: &str) {
    if let Some(ref path) = self.log_file_path {
      let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(path)
        .expect("Failed to open log file");
      writeln!(file, "{}", message).expect("Failed to write to log file");
    }
  }
}
