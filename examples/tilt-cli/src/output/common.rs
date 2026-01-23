pub fn format_count(count: usize, singular: &str, plural: &str) -> String {
    if count == 1 {
        format!("(1 {})", singular)
    } else {
        format!("({} {})", count, plural)
    }
}

pub fn format_date(timestamp: &str) -> String {
    timestamp.split('T').next().unwrap_or("-").to_string()
}

pub fn format_bytes(bytes: u64) -> String {
    format!("{:.2}", bytes as f64 / 1_000_000_000.0)
}

pub fn format_opt(opt: Option<String>) -> String {
    opt.unwrap_or_else(|| "-".to_string())
}

pub fn format_opt_ref(opt: &Option<String>) -> String {
    opt.as_ref().unwrap_or(&String::from("-")).clone()
}
