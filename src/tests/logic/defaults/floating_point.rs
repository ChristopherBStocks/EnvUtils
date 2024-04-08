use crate::defaults::{get_env_f32_or_default, get_env_f64_or_default};

#[tokio::test]
async fn f64_env_test() {
    let key: &str = "F64_TEST";
    let value: f64 = f64::MAX;
    let retrieve: f64 = get_env_f64_or_default(key, 0.0);
    assert_eq!(retrieve, value);
}

#[tokio::test]
async fn f64_default_test() {
    let key: &str = "F64_DEFAULT_TEST";
    let value: f64 = f64::MAX;
    let retrieve: f64 = get_env_f64_or_default(key, value);
    assert_eq!(retrieve, value);
}

#[tokio::test]
async fn f32_env_test() {
    let key: &str = "F32_TEST";
    let value: f32 = f32::MAX;
    let retrieve: f32 = get_env_f32_or_default(key, 0.0);
    assert_eq!(retrieve, value);
}

#[tokio::test]
async fn f32_default_test() {
    let key: &str = "F32_DEFAULT_TEST";
    let value: f32 = f32::MAX;
    let retrieve: f32 = get_env_f32_or_default(key, value);
    assert_eq!(retrieve, value);
}