use dotenvy::var;

pub fn get_env_f64_or_default(key: &str, default: f64) -> f64 {
    var(key)
        .unwrap_or_else(|_| default.to_string())
        .parse::<f64>()
        .unwrap_or(default)
}

pub fn get_env_f32_or_default(key: &str, default: f32) -> f32 {
    var(key)
        .unwrap_or_else(|_| default.to_string())
        .parse::<f32>()
        .unwrap_or(default)
}