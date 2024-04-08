use dotenvy::var;

pub fn get_env_string_or_default(key: &str, default: &str) -> String {
    var(key).unwrap_or_else(|_| default.into())
}