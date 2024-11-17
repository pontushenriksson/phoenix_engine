use std::sync::{Arc, Mutex, mpsc};
use std::fs::OpenOptions;
use std::io::Write;
use std::thread;
use chrono::Local;

#[derive(Debug, Clone, PartialEq)]
pub enum DebuggerOutputMode {
  Terminal,
  File,
}

#[derive(Debug, Clone, PartialEq)]
pub enum DebuggerRunningMode {
  Off,
  Instant(DebuggerOutputMode),
  Accumulate(DebuggerOutputMode),
}

#[derive(Debug, Clone)]
pub struct LogMessage {
  pub level: LogLevel,
  pub message: String,
}

#[derive(Debug, Clone)]
pub enum LogLevel {
  Info,
  Warning,
  Error,
  Trace,
}

#[derive(Debug)]
pub struct EngineDebugger {
  mode: DebuggerRunningMode,
  stack: Option<Arc<Mutex<Vec<LogMessage>>>>,
  sender: Option<mpsc::Sender<LogMessage>>,
  log_file_path: Option<String>,
}

impl EngineDebugger {
  pub fn new(mode: DebuggerRunningMode) -> Self {
    let log_file_path = match &mode {
      DebuggerRunningMode::Instant(DebuggerOutputMode::File)
      | DebuggerRunningMode::Accumulate(DebuggerOutputMode::File) => {
          Some(Self::create_log_file())
      }
      _ => None,
    };

    let stack = if let DebuggerRunningMode::Accumulate(_) = mode {
      Some(Arc::new(Mutex::new(Vec::new())))
    } else {
      None
    };

    let (sender, receiver): (Option<mpsc::Sender<LogMessage>>, Option<mpsc::Receiver<LogMessage>>) =
    if mode != DebuggerRunningMode::Off {
      let (s, r) = mpsc::channel::<LogMessage>();
      (Some(s), Some(r))
    } else {
      (None, None)
    };

    if let Some(receiver) = receiver {
      let log_file_path = log_file_path.clone();
      let mode_clone = mode.clone();
      thread::spawn(move || {
        let mut file = log_file_path.map(|path| {
          OpenOptions::new()
            .create(true)
            .append(true)
            .open(path)
            .expect("Failed to open log file")
        });

        for log in receiver {
          match &mode_clone {
            DebuggerRunningMode::Instant(DebuggerOutputMode::Terminal) => {
              println!("[{:?}] {}", log.level, log.message);
            }
            DebuggerRunningMode::Instant(DebuggerOutputMode::File) => {
              if let Some(f) = file.as_mut() {
                writeln!(f, "[{:?}] {}", log.level, log.message)
                  .expect("Failed to write to log file");
              }
            }
            // Accumulate & Off
            _ => {},
          }
        }
      });
    }

    EngineDebugger {
      mode,
      stack,
      sender,
      log_file_path,
    }
  }

  fn create_log_file() -> String {
    const DIR: &str = "./logs";
    std::fs::create_dir_all(DIR).expect("Failed to create logs directory");

    let now = Local::now();
    let filename = format!("{}/log_{}.txt", DIR, now.format("%Y-%m-%d_%H-%M-%S"));
    filename
  }

  pub fn log(&self, level: LogLevel, message: String) {
    if let Some(sender) = &self.sender {
      let log_msg = LogMessage { level, message: message.clone() };
      sender.send(log_msg.clone()).unwrap();

      if let Some(stack) = &self.stack {
        stack.lock().unwrap().push(log_msg);
      }
    }
  }

  /// Flushes and clears accumulated logs.
  pub fn flush(&self) -> Option<Vec<LogMessage>> {
    self.stack.as_ref().map(|stack| {
      let mut locked_stack = stack.lock().unwrap();
      let flushed_logs = locked_stack.clone();
      locked_stack.clear(); // Clear after flushing
      flushed_logs
    })
  }
}
