use std::env::VarError;

use dotenvy::{Error, var};

use crate::EnvUtilsError;
use crate::interactions::repositories::EnvRepository;

pub struct DotenvyRepository;

impl EnvRepository for DotenvyRepository {
	fn load(&self) -> Result<(), EnvUtilsError> {
		dotenvy::dotenv().map_err(|_| EnvUtilsError::FileLoadError(".env".into())).map(|_| ())
	}
	
	fn get_filename(&self) -> Result<String, EnvUtilsError> {
		self.get("ENV")
	}
	
	fn load_with_filename(&self, filename: &str) -> Result<(), EnvUtilsError> {
		dotenvy::from_filename(format!(".env.{0}", filename))
			.map_err(|_| EnvUtilsError::FileLoadError(filename.into()))
			.map(|_| {
				eprintln!("Loaded .env.{} file\n", filename);
				()
			})
	}
	
	fn get(&self, key: &str) -> Result<String, EnvUtilsError> {
		match var(key) {
			Ok(value) => Ok(value),
			Err(e) => match e {
				Error::EnvVar(e) => match e {
					VarError::NotPresent => Err(EnvUtilsError::VarNotFound(key.into())),
					_ => Err(EnvUtilsError::VarLoadError(key.into())),
				},
				_ => Err(EnvUtilsError::VarLoadError(key.into())),
			}
		}
	}
}