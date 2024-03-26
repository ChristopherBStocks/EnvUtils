use dotenvy::var;

pub fn get_env_string_or_default(key: &str, default: &str) -> String {
    var(key).unwrap_or_else(|_| default.into())
}

pub fn get_env_u64_or_default(key: &str, default: u64) -> u64 {
    var(key)
        .unwrap_or_else(|_| default.to_string())
        .parse::<u64>()
        .unwrap_or(default)
}

pub fn get_env_i64_or_default(key: &str, default: i64) -> i64 {
    var(key)
        .unwrap_or_else(|_| default.to_string())
        .parse::<i64>()
        .unwrap_or(default)
}

pub fn get_env_u32_or_default(key: &str, default: u32) -> u32 {
    var(key)
        .unwrap_or_else(|_| default.to_string())
        .parse::<u32>()
        .unwrap_or(default)
}

pub fn get_env_i32_or_default(key: &str, default: i32) -> i32 {
    var(key)
        .unwrap_or_else(|_| default.to_string())
        .parse::<i32>()
        .unwrap_or(default)
}

pub fn get_env_u16_or_default(key: &str, default: u16) -> u16 {
    var(key)
        .unwrap_or_else(|_| default.to_string())
        .parse::<u16>()
        .unwrap_or(default)
}

pub fn get_env_i16_or_default(key: &str, default: i16) -> i16 {
    var(key)
        .unwrap_or_else(|_| default.to_string())
        .parse::<i16>()
        .unwrap_or(default)
}

pub fn get_env_u8_or_default(key: &str, default: u8) -> u8 {
    var(key)
        .unwrap_or_else(|_| default.to_string())
        .parse::<u8>()
        .unwrap_or(default)
}

pub fn get_env_i8_or_default(key: &str, default: i8) -> i8 {
    var(key)
        .unwrap_or_else(|_| default.to_string())
        .parse::<i8>()
        .unwrap_or(default)
}

pub fn get_env_bool_or_default(key: &str, default: bool) -> bool {
    var(key)
        .unwrap_or_else(|_| default.to_string())
        .parse::<bool>()
        .unwrap_or(default)
}

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