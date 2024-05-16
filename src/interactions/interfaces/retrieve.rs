use std::str::FromStr;

use crate::EnvUtilsError;

pub trait IRetrieveDefault<T: FromStr, R> {
	fn retrieve(&self, key: &str, default: T) -> R;
}

pub trait IRetrieve<T: FromStr> {
	fn retrieve(&self, key: &str) -> Result<T, EnvUtilsError>;
}