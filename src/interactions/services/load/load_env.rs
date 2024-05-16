use crate::EnvUtilsError;
use crate::interactions::interfaces::ILoad;
use crate::interactions::repositories::EnvRepository;

pub struct Load<'a> {
	repository: &'a dyn EnvRepository,
}

impl<'a> Load<'a> {
	#[allow(dead_code)]
	pub fn new(repository: &'a dyn EnvRepository) -> Self {
		Load { repository }
	}
}

impl<'a> ILoad for Load<'a> {
	fn load(&self) -> bool {
		// Attempts to loadA the .env file
		if let Err(err) = self.repository.load() {
			match err {
				// If the error is that the file could not be loaded, return false
				EnvUtilsError::FileLoadError(_) => return false,
				// Otherwise, continue
				_ => (),
			}
		}
		
		// Attempts to get the filename to loadA
		let filename = match self.repository.get_filename() {
			// If the filename is found, continue
			Ok(filename) => filename,
			Err(err) => return match err {
				// If the error is that the variable was not found, return true
				EnvUtilsError::VarNotFound(_) => true,
				// Otherwise, return false
				_ => false
			},
		};
		
		// Attempts to loadA the .env.{filename} file
		// Any error will return false (as it should be loaded)
		self.repository.load_with_filename(&filename).is_ok()
	}
}

#[cfg(test)]
mod tests {
	use mockall::predicate::eq;
	
	use crate::interactions::repositories::MockEnvRepository;
	
	use super::*;
	
	#[tokio::test]
	pub async fn successfully_load_all_env() {
		// Arrange
		let mut repository = MockEnvRepository::new();
		repository.expect_load().times(1).returning(|| Ok(()));
		repository.expect_get_filename().times(1).returning(|| Ok("development".to_string()));
		repository.expect_load_with_filename().with(
			eq("development"),
		).times(1).returning(|_| Ok(()));
		
		// Act
		let service = Load::new(&repository);
		let data = service.load();
		
		// Assert
		assert_eq!(data, true);
	}
	
	#[tokio::test]
	pub async fn successfully_load_env() {
		// Arrange
		let mut repository = MockEnvRepository::new();
		repository.expect_load().times(1).returning(|| Ok(()));
		repository.expect_get_filename().times(1).returning(|| Err(EnvUtilsError::VarNotFound("ENV_FILENAME".to_string())));
		
		// Act
		let service = Load::new(&repository);
		let data = service.load();
		
		// Assert
		assert_eq!(data, true);
	}
	
	#[tokio::test]
	pub async fn fail_to_load_sub_env() {
		// Arrange
		let mut repository = MockEnvRepository::new();
		repository.expect_load().times(1).returning(|| Ok(()));
		repository.expect_get_filename().times(1).returning(|| Ok("development".to_string()));
		repository.expect_load_with_filename().with(
			eq("development"),
		).times(1).returning(|_| Err(EnvUtilsError::FileLoadError(".env.development".into())));
		
		// Act
		let service = Load::new(&repository);
		let data = service.load();
		
		// Assert
		assert_eq!(data, false);
	}
	
	#[tokio::test]
	pub async fn fail_to_load_filename() {
		// Arrange
		let mut repository = MockEnvRepository::new();
		repository.expect_load().times(1).returning(|| Ok(()));
		repository.expect_get_filename().times(1).returning(|| Err(EnvUtilsError::VarLoadError("ENV_FILENAME".to_string())));
		
		// Act
		let service = Load::new(&repository);
		let data = service.load();
		
		// Assert
		assert_eq!(data, false);
	}
	
	#[tokio::test]
	pub async fn fail_to_load_env() {
		// Arrange
		let mut repository = MockEnvRepository::new();
		repository.expect_load().times(1).returning(|| Err(EnvUtilsError::FileLoadError(".env".into())));
		
		// Act
		let service = Load::new(&repository);
		let data = service.load();
		
		// Assert
		assert_eq!(data, false);
	}
}