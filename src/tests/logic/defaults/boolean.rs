use crate::get_env_bool_or_default;

#[tokio::test]
async fn bool_env_test() {
    let key: &str = "BOOL_TEST";
    let value: bool = true;
    let retrieve: bool = get_env_bool_or_default(key, false);
    assert_eq!(retrieve, value);
}

#[tokio::test]
async fn bool_default_test() {
    let key: &str = "BOOL_DEFAULT_TEST";
    let value: bool = true;
    let retrieve: bool = get_env_bool_or_default(key, value);
    assert_eq!(retrieve, value);
}