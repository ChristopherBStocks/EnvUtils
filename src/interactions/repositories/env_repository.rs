#[cfg(test)]
use mockall::*;

use crate::EnvUtilsError;

#[cfg_attr(test, automock)]
pub trait EnvRepository {
	// Load .env - ok if successful, otherwise error
	fn load(&self) -> Result<(), EnvUtilsError>;
	
	// Return the filename that will be loaded
	fn get_filename(&self) -> Result<String, EnvUtilsError>;
	
	// Attempt to load a .env.{filename}
	fn load_with_filename(&self, filename: &str) -> Result<(), EnvUtilsError>;
	
	// Return the value of the key
	fn get(&self, key: &str) -> Result<String, EnvUtilsError>;
}