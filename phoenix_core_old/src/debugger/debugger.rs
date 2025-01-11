#[derive(Debug)]
pub enum PhoenixError {
  EngineCreation(String),

  Io(String),

  GetAsset(String),
  LoadAsset(String),
  UnloadAsset(String),
}

pub struct Debugger;

impl Debugger {
  // Clears OpenGL errors
  pub fn clear_errors() {
    while unsafe { gl::GetError() } != gl::NO_ERROR {}
  }

  // Checks for OpenGL errors and logs them
  pub fn check_errors(file: &str, line: u32) {
    let mut error = unsafe { gl::GetError() };
    while error != gl::NO_ERROR {
      eprintln!("OpenGL Error (0x{:X}) at {}:{}", error, file, line);
      error = unsafe { gl::GetError() };
    }
  }
}

/// Macro for debugging OpenGL-specific functioncalls
#[macro_export]
macro_rules! gl_call {
  ($func:expr) => {{
    Debugger::clear_errors();
    let result = $func;
    Debugger::check_errors(file!(), line!());
    result
  }};
}
