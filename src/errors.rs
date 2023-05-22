use std::{sync::atomic::AtomicBool, fmt};

pub static HAD_ERROR: AtomicBool = AtomicBool::new(false);

pub(crate) struct CompileError {
    pub line: usize,
    pub location: String,
    pub message: String,
}

impl CompileError {
    pub(crate) fn new(line: usize, location: String, message: String) -> Self {
        CompileError {
            line,
            location,
            message,
        }
    }
}

pub(crate) trait WithError {
    fn error(error: CompileError) {
        println!("{}", error);
    }

    fn set_error(value: bool) {
        HAD_ERROR.store(value, std::sync::atomic::Ordering::Relaxed);
    }

    fn has_error() -> bool {
        HAD_ERROR.load(std::sync::atomic::Ordering::Relaxed)
    }
}

impl fmt::Display for CompileError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Customize so only `x` and `y` are denoted.
        write!(f, "[line {}] Error {}: {}", self.line, self.location, self.message)
    }
}