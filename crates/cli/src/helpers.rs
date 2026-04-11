// ─────────────────────────────────────────────────────────────────────────────
// Helper functions
// ─────────────────────────────────────────────────────────────────────────────

pub fn json_print<T: serde::Serialize>(data: &T) {
    println!(
        "{}",
        serde_json::to_string_pretty(data).unwrap_or_else(|_| "Erro ao serializar".into())
    );
}

pub fn print_ok(msg: &str) {
    println!("✅ {}", msg);
}

pub fn print_err(msg: &str) {
    eprintln!("❌ {}", msg);
}

pub fn parse_decimal(v: f64) -> rust_decimal::Decimal {
    rust_decimal::Decimal::from_f64_retain(v).unwrap_or(rust_decimal_macros::dec!(0))
}
