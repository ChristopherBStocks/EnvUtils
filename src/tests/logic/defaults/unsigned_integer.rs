use crate::{get_env_u16_or_default, get_env_u32_or_default, get_env_u64_or_default, get_env_u8_or_default};

#[tokio::test]
async fn u64_env_test() {
    let key: &str = "U64_TEST";
    let value: u64 = u64::MAX;
    let retrieve: u64 = get_env_u64_or_default(key, 0);
    assert_eq!(retrieve, value);
}

#[tokio::test]
async fn u64_default_test() {
    let key: &str = "U64_DEFAULT_TEST";
    let value: u64 = u64::MAX;
    let retrieve: u64 = get_env_u64_or_default(key, value);
    assert_eq!(retrieve, value);
}

#[tokio::test]
async fn u32_env_test() {
    let key: &str = "U32_TEST";
    let value: u32 = u32::MAX;
    let retrieve: u32 = get_env_u32_or_default(key, 0);
    assert_eq!(retrieve, value);
}

#[tokio::test]
async fn u32_default_test() {
    let key: &str = "U32_DEFAULT_TEST";
    let value: u32 = u32::MAX;
    let retrieve: u32 = get_env_u32_or_default(key, value);
    assert_eq!(retrieve, value);
}

#[tokio::test]
async fn u16_env_test() {
    let key: &str = "U16_TEST";
    let value: u16 = u16::MAX;
    let retrieve: u16 = get_env_u16_or_default(key, 0);
    assert_eq!(retrieve, value);
}

#[tokio::test]
async fn u16_default_test() {
    let key: &str = "U16_DEFAULT_TEST";
    let value: u16 = u16::MAX;
    let retrieve: u16 = get_env_u16_or_default(key, value);
    assert_eq!(retrieve, value);
}

#[tokio::test]
async fn u8_env_test() {
    let key: &str = "U8_TEST";
    let value: u8 = u8::MAX;
    let retrieve: u8 = get_env_u8_or_default(key, 0);
    assert_eq!(retrieve, value);
}

#[tokio::test]
async fn u8_default_test() {
    let key: &str = "U8_DEFAULT_TEST";
    let value: u8 = u8::MAX;
    let retrieve: u8 = get_env_u8_or_default(key, value);
    assert_eq!(retrieve, value);
}