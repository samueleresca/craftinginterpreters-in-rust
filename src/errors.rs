use std::sync::atomic::AtomicBool;

pub static HAD_ERROR: AtomicBool = AtomicBool::new(false);

pub trait WithError {
    fn error(line: usize, message: &str) {
        Self::report(line, "", message);
    }

    fn report(line: usize, location: &str, message: &str) {
        println!("[line {}] Error {}: {}", line, location, message);
    }

    fn set_error(value: bool) {
        HAD_ERROR.store(value, std::sync::atomic::Ordering::Relaxed);
    }

    fn has_error() -> bool {
        HAD_ERROR.load(std::sync::atomic::Ordering::Relaxed)
    }
}
