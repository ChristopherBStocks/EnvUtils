use crate::{get_env_f32_or_default, get_env_f64_or_default, get_env_i16_or_default, get_env_i32_or_default, get_env_i64_or_default, get_env_i8_or_default, get_env_string_or_default, get_env_u16_or_default, get_env_u32_or_default, get_env_u64_or_default, get_env_u8_or_default};

#[tokio::test]
async fn string_found_test() {
    let key: &str = "STRING_TEST";
    let value: &str = "string_test";
    let retrieve: String = get_env_string_or_default(key, "");
    assert_eq!(retrieve, value);
}

#[tokio::test]
async fn string_not_found_test() {
    let key: &str = "NOT_STRING_TEST";
    let value: &str = "string_test";
    let retrieve: String = get_env_string_or_default(key, value);
    assert_eq!(retrieve, value);
}

#[tokio::test]
async fn u64_found_test() {
    let key: &str = "U64_TEST";
    let value: u64 = u64::MAX;
    let retrieve: u64 = get_env_u64_or_default(key, 0);
    assert_eq!(retrieve, value);
}

#[tokio::test]
async fn u64_not_found_test() {
    let key: &str = "NOT_U64_TEST";
    let value: u64 = u64::MAX;
    let retrieve: u64 = get_env_u64_or_default(key, value);
    assert_eq!(retrieve, value);
}

#[tokio::test]
async fn i64_found_test() {
    let key: &str = "I64_TEST";
    let value: i64 = i64::MAX;
    let retrieve: i64 = get_env_i64_or_default(key, 0);
    assert_eq!(retrieve, value);
}

#[tokio::test]
async fn i64_not_found_test() {
    let key: &str = "NOT_I64_TEST";
    let value: i64 = i64::MAX;
    let retrieve: i64 = get_env_i64_or_default(key, value);
    assert_eq!(retrieve, value);
}

#[tokio::test]
async fn u32_found_test() {
    let key: &str = "U32_TEST";
    let value: u32 = u32::MAX;
    let retrieve: u32 = get_env_u32_or_default(key, 0);
    assert_eq!(retrieve, value);
}

#[tokio::test]
async fn u32_not_found_test() {
    let key: &str = "NOT_U32_TEST";
    let value: u32 = u32::MAX;
    let retrieve: u32 = get_env_u32_or_default(key, value);
    assert_eq!(retrieve, value);
}

#[tokio::test]
async fn i32_found_test() {
    let key: &str = "I32_TEST";
    let value: i32 = i32::MAX;
    let retrieve: i32 = get_env_i32_or_default(key, 0);
    assert_eq!(retrieve, value);
}

#[tokio::test]
async fn i32_not_found_test() {
    let key: &str = "NOT_I32_TEST";
    let value: i32 = i32::MAX;
    let retrieve: i32 = get_env_i32_or_default(key, value);
    assert_eq!(retrieve, value);
}

#[tokio::test]
async fn u16_found_test() {
    let key: &str = "U16_TEST";
    let value: u16 = u16::MAX;
    let retrieve: u16 = get_env_u16_or_default(key, 0);
    assert_eq!(retrieve, value);
}

#[tokio::test]
async fn u16_not_found_test() {
    let key: &str = "NOT_U16_TEST";
    let value: u16 = u16::MAX;
    let retrieve: u16 = get_env_u16_or_default(key, value);
    assert_eq!(retrieve, value);
}

#[tokio::test]
async fn i16_found_test() {
    let key: &str = "I16_TEST";
    let value: i16 = i16::MAX;
    let retrieve: i16 = get_env_i16_or_default(key, 0);
    assert_eq!(retrieve, value);
}

#[tokio::test]
async fn i16_not_found_test() {
    let key: &str = "NOT_I16_TEST";
    let value: i16 = i16::MAX;
    let retrieve: i16 = get_env_i16_or_default(key, value);
    assert_eq!(retrieve, value);
}

#[tokio::test]
async fn u8_found_test() {
    let key: &str = "U8_TEST";
    let value: u8 = u8::MAX;
    let retrieve: u8 = get_env_u8_or_default(key, 0);
    assert_eq!(retrieve, value);
}

#[tokio::test]
async fn u8_not_found_test() {
    let key: &str = "NOT_U8_TEST";
    let value: u8 = u8::MAX;
    let retrieve: u8 = get_env_u8_or_default(key, value);
    assert_eq!(retrieve, value);
}

#[tokio::test]
async fn i8_found_test() {
    let key: &str = "I8_TEST";
    let value: i8 = i8::MAX;
    let retrieve: i8 = get_env_i8_or_default(key, 0);
    assert_eq!(retrieve, value);
}

#[tokio::test]
async fn i8_not_found_test() {
    let key: &str = "NOT_I8_TEST";
    let value: i8 = i8::MAX;
    let retrieve: i8 = get_env_i8_or_default(key, value);
    assert_eq!(retrieve, value);
}

#[tokio::test]
async fn f64_found_test() {
    let key: &str = "F64_TEST";
    let value: f64 = f64::MAX;
    let retrieve: f64 = get_env_f64_or_default(key, 0.0);
    assert_eq!(retrieve, value);
}

#[tokio::test]
async fn f64_not_found_test() {
    let key: &str = "NOT_F64_TEST";
    let value: f64 = f64::MAX;
    let retrieve: f64 = get_env_f64_or_default(key, value);
    assert_eq!(retrieve, value);
}

#[tokio::test]
async fn f32_found_test() {
    let key: &str = "F32_TEST";
    let value: f32 = f32::MAX;
    let retrieve: f32 = get_env_f32_or_default(key, 0.0);
    assert_eq!(retrieve, value);
}

#[tokio::test]
async fn f32_not_found_test() {
    let key: &str = "NOT_F32_TEST";
    let value: f32 = f32::MAX;
    let retrieve: f32 = get_env_f32_or_default(key, value);
    assert_eq!(retrieve, value);
}