use dotenvy::var;

pub fn get_env_bool_or_default(key: &str, default: bool) -> bool {
    var(key)
        .unwrap_or_else(|_| default.to_string())
        .parse::<bool>()
        .unwrap_or(default)
}