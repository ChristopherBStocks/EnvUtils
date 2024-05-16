use std::str::FromStr;

use crate::interactions::interfaces::IRetrieveDefault;
use crate::interactions::repositories::EnvRepository;

pub struct RetrieveDefault<'a> {
	repository: &'a dyn EnvRepository,
}

impl<'a> RetrieveDefault<'a> {
	#[allow(dead_code)]
	pub fn new(repository: &'a dyn EnvRepository) -> Self {
		RetrieveDefault { repository }
	}
}

impl<'a, T: FromStr> IRetrieveDefault<T, T> for RetrieveDefault<'a> {
	fn retrieve(&self, key: &str, default: T) -> T {
		match self.repository.get(key) {
			Ok(value) => value.parse().unwrap_or_else(|_| default),
			Err(_) => default,
		}
	}
}

#[cfg(test)]
mod tests {
	use mockall::predicate::eq;
	
	use crate::EnvUtilsError;
	use crate::interactions::repositories::MockEnvRepository;
	
	use super::*;
	
	#[tokio::test]
	pub async fn successfully_retrieve_from_env() {
		// Arrange
		let mut repository = MockEnvRepository::new();
		repository.expect_get().with(eq("KEY")).times(1).returning(|_| Ok("VALUE".to_string()));
		
		// Act
		let service = RetrieveDefault::new(&repository);
		let data = service.retrieve("KEY", "DEFAULT".to_string());
		
		// Assert
		assert_eq!(data, "VALUE");
	}
	
	#[tokio::test]
	pub async fn not_found_and_retrieve_default() {
		// Arrange
		let mut repository = MockEnvRepository::new();
		repository.expect_get().with(eq("KEY")).times(1).returning(|_| Err(EnvUtilsError::VarNotFound("KEY".into())));
		
		// Act
		let service = RetrieveDefault::new(&repository);
		let data = service.retrieve("KEY", "DEFAULT".to_string());
		
		// Assert
		assert_eq!(data, "DEFAULT");
	}
	
	#[tokio::test]
	pub async fn fail_parse_and_retrieve_default() {
		// Arrange
		let mut repository = MockEnvRepository::new();
		repository.expect_get().with(eq("KEY")).times(1).returning(|_| Err(EnvUtilsError::VarParseError("KEY".into())));
		
		// Act
		let service = RetrieveDefault::new(&repository);
		let data = service.retrieve("KEY", "DEFAULT".to_string());
		
		// Assert
		assert_eq!(data, "DEFAULT");
	}
	
	#[tokio::test]
	pub async fn successfully_retrieve_boolean() {
		// Arrange
		let mut repository = MockEnvRepository::new();
		repository.expect_get().with(eq("KEY")).times(1).returning(|_| Ok("true".to_string()));
		
		// Act
		let service = RetrieveDefault::new(&repository);
		let data = service.retrieve("KEY", false);
		
		// Assert
		assert_eq!(data, true);
	}
	
	#[tokio::test]
	pub async fn successfully_retrieve_f64() {
		// Arrange
		let mut repository = MockEnvRepository::new();
		repository.expect_get().with(eq("KEY")).times(1).returning(|_| Ok("1.7976931348623157e+308".to_string()));
		
		// Act
		let service = RetrieveDefault::new(&repository);
		let data = service.retrieve("KEY", 0.0f64);
		
		// Assert
		assert_eq!(data, 1.7976931348623157e+308);
	}
	
	#[tokio::test]
	pub async fn successfully_retrieve_f32() {
		// Arrange
		let mut repository = MockEnvRepository::new();
		repository.expect_get().with(eq("KEY")).times(1).returning(|_| Ok("3.4028235e+38".to_string()));
		
		// Act
		let service = RetrieveDefault::new(&repository);
		let data = service.retrieve("KEY", 0.0f32);
		
		// Assert
		assert_eq!(data, 3.4028235e+38);
	}
	
	#[tokio::test]
	pub async fn successfully_retrieve_i128() {
		// Arrange
		let mut repository = MockEnvRepository::new();
		repository.expect_get().with(eq("KEY")).times(1).returning(|_| Ok("170141183460469231731687303715884105727".to_string()));
		
		// Act
		let service = RetrieveDefault::new(&repository);
		let data = service.retrieve("KEY", 0i128);
		
		// Assert
		assert_eq!(data, 170141183460469231731687303715884105727);
	}
	
	#[tokio::test]
	pub async fn successfully_retrieve_i64() {
		// Arrange
		let mut repository = MockEnvRepository::new();
		repository.expect_get().with(eq("KEY")).times(1).returning(|_| Ok("9223372036854775807".to_string()));
		
		// Act
		let service = RetrieveDefault::new(&repository);
		let data = service.retrieve("KEY", 0i64);
		
		// Assert
		assert_eq!(data, 9223372036854775807);
	}
	
	#[tokio::test]
	pub async fn successfully_retrieve_i32() {
		// Arrange
		let mut repository = MockEnvRepository::new();
		repository.expect_get().with(eq("KEY")).times(1).returning(|_| Ok("2147483647".to_string()));
		
		// Act
		let service = RetrieveDefault::new(&repository);
		let data = service.retrieve("KEY", 0i32);
		
		// Assert
		assert_eq!(data, 2147483647);
	}
	
	#[tokio::test]
	pub async fn successfully_retrieve_i16() {
		// Arrange
		let mut repository = MockEnvRepository::new();
		repository.expect_get().with(eq("KEY")).times(1).returning(|_| Ok("32767".to_string()));
		
		// Act
		let service = RetrieveDefault::new(&repository);
		let data = service.retrieve("KEY", 0i16);
		
		// Assert
		assert_eq!(data, 32767);
	}
	
	#[tokio::test]
	pub async fn successfully_retrieve_i8() {
		// Arrange
		let mut repository = MockEnvRepository::new();
		repository.expect_get().with(eq("KEY")).times(1).returning(|_| Ok("127".to_string()));
		
		// Act
		let service = RetrieveDefault::new(&repository);
		let data = service.retrieve("KEY", 0i8);
		
		// Assert
		assert_eq!(data, 127);
	}
	
	#[tokio::test]
	pub async fn successfully_retrieve_string() {
		// Arrange
		let mut repository = MockEnvRepository::new();
		repository.expect_get().with(eq("KEY")).times(1).returning(|_| Ok("VALUE".to_string()));
		
		// Act
		let service = RetrieveDefault::new(&repository);
		let data = service.retrieve("KEY", "DEFAULT".to_string());
		
		// Assert
		assert_eq!(data, "VALUE");
	}
	
	#[tokio::test]
	pub async fn successfully_retrieve_u128() {
		// Arrange
		let mut repository = MockEnvRepository::new();
		repository.expect_get().with(eq("KEY")).times(1).returning(|_| Ok("340282366920938463463374607431768211455".to_string()));
		
		// Act
		let service = RetrieveDefault::new(&repository);
		let data = service.retrieve("KEY", 0u128);
		
		// Assert
		assert_eq!(data, 340282366920938463463374607431768211455);
	}
	
	#[tokio::test]
	pub async fn successfully_retrieve_u64() {
		// Arrange
		let mut repository = MockEnvRepository::new();
		repository.expect_get().with(eq("KEY")).times(1).returning(|_| Ok("18446744073709551615".to_string()));
		
		// Act
		let service = RetrieveDefault::new(&repository);
		let data = service.retrieve("KEY", 0u64);
		
		// Assert
		assert_eq!(data, 18446744073709551615);
	}
	
	#[tokio::test]
	pub async fn successfully_retrieve_u32() {
		// Arrange
		let mut repository = MockEnvRepository::new();
		repository.expect_get().with(eq("KEY")).times(1).returning(|_| Ok("4294967295".to_string()));
		
		// Act
		let service = RetrieveDefault::new(&repository);
		let data = service.retrieve("KEY", 0u32);
		
		// Assert
		assert_eq!(data, 4294967295);
	}
	
	#[tokio::test]
	pub async fn successfully_retrieve_u16() {
		// Arrange
		let mut repository = MockEnvRepository::new();
		repository.expect_get().with(eq("KEY")).times(1).returning(|_| Ok("65535".to_string()));
		
		// Act
		let service = RetrieveDefault::new(&repository);
		let data = service.retrieve("KEY", 0u16);
		
		// Assert
		assert_eq!(data, 65535);
	}
	
	#[tokio::test]
	pub async fn successfully_retrieve_u8() {
		// Arrange
		let mut repository = MockEnvRepository::new();
		repository.expect_get().with(eq("KEY")).times(1).returning(|_| Ok("255".to_string()));
		
		// Act
		let service = RetrieveDefault::new(&repository);
		let data = service.retrieve("KEY", 0u8);
		
		// Assert
		assert_eq!(data, 255);
	}
}