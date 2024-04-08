use crate::get_env_string_or_default;

#[tokio::test]
async fn string_env_test() {
    let key: &str = "STRING_TEST";
    let value: &str = "string_test";
    let retrieve: String = get_env_string_or_default(key, "");
    assert_eq!(retrieve, value);
}

#[tokio::test]
async fn string_default_test() {
    let key: &str = "STRING_DEFAULT_TEST";
    let value: &str = "string_test";
    let retrieve: String = get_env_string_or_default(key, value);
    assert_eq!(retrieve, value);
}