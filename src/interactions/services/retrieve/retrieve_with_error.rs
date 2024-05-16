use std::str::FromStr;

use crate::EnvUtilsError;
use crate::interactions::interfaces::IRetrieve;
use crate::interactions::repositories::EnvRepository;

pub struct RetrieveError<'a> {
	repository: &'a dyn EnvRepository,
}

impl<'a> RetrieveError<'a> {
	#[allow(dead_code)]
	pub fn new(repository: &'a dyn EnvRepository) -> Self {
		RetrieveError { repository }
	}
}

impl<'a, T: FromStr> IRetrieve<T> for RetrieveError<'a> {
	fn retrieve(&self, key: &str) -> Result<T, EnvUtilsError> {
		match self.repository.get(key) {
			Ok(value) => match value.parse() {
				Ok(value) => Ok(value),
				Err(_) => Err(EnvUtilsError::VarParseError(key.to_string())),
			},
			Err(e) => Err(e)
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
		let service = RetrieveError::new(&repository);
		let data = service.retrieve("KEY");
		
		// Assert
		assert_eq!(data, Ok("VALUE".to_string()));
	}
	
	#[tokio::test]
	pub async fn unable_to_parse() {
		// Arrange
		let mut repository = MockEnvRepository::new();
		repository.expect_get().with(eq("KEY")).times(1).returning(|_| Ok("VALUE".to_string()));
		
		// Act
		let service = RetrieveError::new(&repository);
		let data: Result<i32, EnvUtilsError> = service.retrieve("KEY");
		
		// Assert
		assert_eq!(data, Err(EnvUtilsError::VarParseError("KEY".into())));
	}
	
	#[tokio::test]
	pub async fn other_error() {
		// Arrange
		let mut repository = MockEnvRepository::new();
		repository.expect_get().with(eq("KEY")).times(1).returning(|_| Err(EnvUtilsError::VarNotFound("KEY".into())));
		
		// Act
		let service = RetrieveError::new(&repository);
		let data: Result<i32, EnvUtilsError> = service.retrieve("KEY");
		
		// Assert
		assert_eq!(data, Err(EnvUtilsError::VarNotFound("KEY".into())));
	}
	
	#[tokio::test]
	pub async fn successfully_retrieve_boolean() {
		// Arrange
		let mut repository = MockEnvRepository::new();
		repository.expect_get().with(eq("KEY")).times(1).returning(|_| Ok("true".to_string()));
		
		// Act
		let service = RetrieveError::new(&repository);
		let data = service.retrieve("KEY");
		
		// Assert
		assert_eq!(data, Ok(true));
	}
	
	#[tokio::test]
	pub async fn successfully_retrieve_f64() {
		// Arrange
		let mut repository = MockEnvRepository::new();
		repository.expect_get().with(eq("KEY")).times(1).returning(|_| Ok("1.7976931348623157e+308".to_string()));
		
		// Act
		let service = RetrieveError::new(&repository);
		let data = service.retrieve("KEY");
		
		// Assert
		assert_eq!(data, Ok(1.7976931348623157e+308));
	}
	
	#[tokio::test]
	pub async fn successfully_retrieve_f32() {
		// Arrange
		let mut repository = MockEnvRepository::new();
		repository.expect_get().with(eq("KEY")).times(1).returning(|_| Ok("3.4028235e+38".to_string()));
		
		// Act
		let service = RetrieveError::new(&repository);
		let data = service.retrieve("KEY");
		
		// Assert
		assert_eq!(data, Ok(3.4028235e+38));
	}
	
	#[tokio::test]
	pub async fn successfully_retrieve_i128() {
		// Arrange
		let mut repository = MockEnvRepository::new();
		repository.expect_get().with(eq("KEY")).times(1).returning(|_| Ok("170141183460469231731687303715884105727".to_string()));
		
		// Act
		let service = RetrieveError::new(&repository);
		let data: Result<i128, EnvUtilsError> = service.retrieve("KEY");
		
		// Assert
		assert_eq!(data, Ok(170141183460469231731687303715884105727));
	}
	
	#[tokio::test]
	pub async fn successfully_retrieve_i64() {
		// Arrange
		let mut repository = MockEnvRepository::new();
		repository.expect_get().with(eq("KEY")).times(1).returning(|_| Ok("9223372036854775807".to_string()));
		
		// Act
		let service = RetrieveError::new(&repository);
		let data: Result<i64, EnvUtilsError> = service.retrieve("KEY");
		
		// Assert
		assert_eq!(data, Ok(9223372036854775807));
	}
	
	#[tokio::test]
	pub async fn successfully_retrieve_i32() {
		// Arrange
		let mut repository = MockEnvRepository::new();
		repository.expect_get().with(eq("KEY")).times(1).returning(|_| Ok("2147483647".to_string()));
		
		// Act
		let service = RetrieveError::new(&repository);
		let data: Result<i32, EnvUtilsError> = service.retrieve("KEY");
		
		// Assert
		assert_eq!(data, Ok(2147483647));
	}
	
	#[tokio::test]
	pub async fn successfully_retrieve_i16() {
		// Arrange
		let mut repository = MockEnvRepository::new();
		repository.expect_get().with(eq("KEY")).times(1).returning(|_| Ok("32767".to_string()));
		
		// Act
		let service = RetrieveError::new(&repository);
		let data: Result<i16, EnvUtilsError> = service.retrieve("KEY");
		
		// Assert
		assert_eq!(data, Ok(32767));
	}
	
	#[tokio::test]
	pub async fn successfully_retrieve_i8() {
		// Arrange
		let mut repository = MockEnvRepository::new();
		repository.expect_get().with(eq("KEY")).times(1).returning(|_| Ok("127".to_string()));
		
		// Act
		let service = RetrieveError::new(&repository);
		let data: Result<i8, EnvUtilsError> = service.retrieve("KEY");
		
		// Assert
		assert_eq!(data, Ok(127));
	}
	
	#[tokio::test]
	pub async fn successfully_retrieve_string() {
		// Arrange
		let mut repository = MockEnvRepository::new();
		repository.expect_get().with(eq("KEY")).times(1).returning(|_| Ok("VALUE".to_string()));
		
		// Act
		let service = RetrieveError::new(&repository);
		let data = service.retrieve("KEY");
		
		// Assert
		assert_eq!(data, Ok("VALUE".to_string()));
	}
	
	#[tokio::test]
	pub async fn successfully_retrieve_u128() {
		// Arrange
		let mut repository = MockEnvRepository::new();
		repository.expect_get().with(eq("KEY")).times(1).returning(|_| Ok("340282366920938463463374607431768211455".to_string()));
		
		// Act
		let service = RetrieveError::new(&repository);
		let data: Result<u128, EnvUtilsError> = service.retrieve("KEY");
		
		// Assert
		assert_eq!(data, Ok(340282366920938463463374607431768211455));
	}
	
	#[tokio::test]
	pub async fn successfully_retrieve_u64() {
		// Arrange
		let mut repository = MockEnvRepository::new();
		repository.expect_get().with(eq("KEY")).times(1).returning(|_| Ok("18446744073709551615".to_string()));
		
		// Act
		let service = RetrieveError::new(&repository);
		let data: Result<u64, EnvUtilsError> = service.retrieve("KEY");
		
		// Assert
		assert_eq!(data, Ok(18446744073709551615));
	}
	
	#[tokio::test]
	pub async fn successfully_retrieve_u32() {
		// Arrange
		let mut repository = MockEnvRepository::new();
		repository.expect_get().with(eq("KEY")).times(1).returning(|_| Ok("4294967295".to_string()));
		
		// Act
		let service = RetrieveError::new(&repository);
		let data: Result<u32, EnvUtilsError> = service.retrieve("KEY");
		
		// Assert
		assert_eq!(data, Ok(4294967295));
	}
	
	#[tokio::test]
	pub async fn successfully_retrieve_u16() {
		// Arrange
		let mut repository = MockEnvRepository::new();
		repository.expect_get().with(eq("KEY")).times(1).returning(|_| Ok("65535".to_string()));
		
		// Act
		let service = RetrieveError::new(&repository);
		let data: Result<u16, EnvUtilsError> = service.retrieve("KEY");
		
		// Assert
		assert_eq!(data, Ok(65535));
	}
	
	#[tokio::test]
	pub async fn successfully_retrieve_u8() {
		// Arrange
		let mut repository = MockEnvRepository::new();
		repository.expect_get().with(eq("KEY")).times(1).returning(|_| Ok("255".to_string()));
		
		// Act
		let service = RetrieveError::new(&repository);
		let data: Result<u8, EnvUtilsError> = service.retrieve("KEY");
		
		// Assert
		assert_eq!(data, Ok(255));
	}
}