use crate::colors;

pub struct Logger {}

impl Logger {
    pub fn new() -> Logger {
        Logger {}
    }

    pub fn info(&self, msg: &str) {
        println!(
            "{}{} INFO {}{}{} {} {}",
            colors::BG_BLUE,
            colors::BLACK,
            colors::RESET,
            colors::CYAN,
            colors::BOLD,
            msg,
            colors::RESET
        );
    }

    pub fn warn(&self, msg: &str) {
        println!(
            "{}{} WARN {}{}{} {} {}",
            colors::BG_YELLOW,
            colors::BLACK,
            colors::RESET,
            colors::YELLOW,
            colors::BOLD,
            msg,
            colors::RESET
        );
    }

    pub fn error(&self, msg: &str) {
        println!(
            "{}{} ERROR {}{}{} {} {}",
            colors::BG_RED,
            colors::BLACK,
            colors::RESET,
            colors::RED,
            colors::BOLD,
            msg,
            colors::RESET
        );
    }
}
