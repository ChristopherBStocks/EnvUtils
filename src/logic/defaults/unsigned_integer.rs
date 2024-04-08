use dotenvy::var;

pub fn get_env_u64_or_default(key: &str, default: u64) -> u64 {
    var(key)
        .unwrap_or_else(|_| default.to_string())
        .parse::<u64>()
        .unwrap_or(default)
}

pub fn get_env_u32_or_default(key: &str, default: u32) -> u32 {
    var(key)
        .unwrap_or_else(|_| default.to_string())
        .parse::<u32>()
        .unwrap_or(default)
}

pub fn get_env_u16_or_default(key: &str, default: u16) -> u16 {
    var(key)
        .unwrap_or_else(|_| default.to_string())
        .parse::<u16>()
        .unwrap_or(default)
}

pub fn get_env_u8_or_default(key: &str, default: u8) -> u8 {
    var(key)
        .unwrap_or_else(|_| default.to_string())
        .parse::<u8>()
        .unwrap_or(default)
}   