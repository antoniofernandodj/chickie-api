use chrono::Utc;

// utils.rs
pub fn agora() -> String {
    Utc::now().format("%Y-%m-%dT%H:%M:%S%.fZ").to_string()
}