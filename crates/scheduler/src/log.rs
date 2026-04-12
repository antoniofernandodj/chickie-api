use chrono::Utc;
use std::io::{self, Write};


pub fn log_msg(level: &str, msg: &str) {
    let now = Utc::now().to_rfc3339();
    eprintln!("[{} {}] {}", now, level, msg);
    let _ = io::stderr().flush();
}

pub fn log_info(msg: &str) { log_msg("INFO", msg); }
pub fn log_error(msg: &str) { log_msg("ERROR", msg); }
pub fn log_warn(msg: &str) { log_msg("WARN", msg); }



