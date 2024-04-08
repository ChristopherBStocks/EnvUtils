use dotenvy::var;

pub fn get_env_i64_or_default(key: &str, default: i64) -> i64 {
    var(key)
        .unwrap_or_else(|_| default.to_string())
        .parse::<i64>()
        .unwrap_or(default)
}

pub fn get_env_i32_or_default(key: &str, default: i32) -> i32 {
    var(key)
        .unwrap_or_else(|_| default.to_string())
        .parse::<i32>()
        .unwrap_or(default)
}

pub fn get_env_i16_or_default(key: &str, default: i16) -> i16 {
    var(key)
        .unwrap_or_else(|_| default.to_string())
        .parse::<i16>()
        .unwrap_or(default)
}

pub fn get_env_i8_or_default(key: &str, default: i8) -> i8 {
    var(key)
        .unwrap_or_else(|_| default.to_string())
        .parse::<i8>()
        .unwrap_or(default)
}