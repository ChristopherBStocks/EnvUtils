use crate::{get_env_i16_or_default, get_env_i32_or_default, get_env_i64_or_default, get_env_i8_or_default};

#[tokio::test]
async fn i64_env_test() {
    let key: &str = "I64_TEST";
    let value: i64 = i64::MAX;
    let retrieve: i64 = get_env_i64_or_default(key, 0);
    assert_eq!(retrieve, value);
}

#[tokio::test]
async fn i64_default_test() {
    let key: &str = "I64_DEFAULT_TEST";
    let value: i64 = i64::MAX;
    let retrieve: i64 = get_env_i64_or_default(key, value);
    assert_eq!(retrieve, value);
}

#[tokio::test]
async fn i32_env_test() {
    let key: &str = "I32_TEST";
    let value: i32 = i32::MAX;
    let retrieve: i32 = get_env_i32_or_default(key, 0);
    assert_eq!(retrieve, value);
}

#[tokio::test]
async fn i32_default_test() {
    let key: &str = "I32_DEFAULT_TEST";
    let value: i32 = i32::MAX;
    let retrieve: i32 = get_env_i32_or_default(key, value);
    assert_eq!(retrieve, value);
}

#[tokio::test]
async fn i16_env_test() {
    let key: &str = "I16_TEST";
    let value: i16 = i16::MAX;
    let retrieve: i16 = get_env_i16_or_default(key, 0);
    assert_eq!(retrieve, value);
}

#[tokio::test]
async fn i16_default_test() {
    let key: &str = "I16_DEFAULT_TEST";
    let value: i16 = i16::MAX;
    let retrieve: i16 = get_env_i16_or_default(key, value);
    assert_eq!(retrieve, value);
}

#[tokio::test]
async fn i8_env_test() {
    let key: &str = "I8_TEST";
    let value: i8 = i8::MAX;
    let retrieve: i8 = get_env_i8_or_default(key, 0);
    assert_eq!(retrieve, value);
}

#[tokio::test]
async fn i8_default_test() {
    let key: &str = "I8_DEFAULT_TEST";
    let value: i8 = i8::MAX;
    let retrieve: i8 = get_env_i8_or_default(key, value);
    assert_eq!(retrieve, value);
}