use std::str::FromStr;

use crate::EnvUtilsError;
use crate::interactions::interfaces::{ILoad, IRetrieve, IRetrieveDefault};
use crate::interactions::repositories::EnvRepository;
use crate::interactions::services::load::Load;
use crate::interactions::services::retrieve::{RetrieveDefault, RetrieveDefaultError, RetrieveError};
use crate::ports::spi::env::{DotenvyRepository, EnvType};

pub struct EnvService {
	repository: Box<dyn EnvRepository>,
}

impl EnvService {
	pub fn new(handler: EnvType) -> Result<Self, EnvUtilsError> {
		match handler {
			#[cfg(feature = "dotenvy")]
			EnvType::Dotenvy => Ok(EnvService {
				repository: Box::new(DotenvyRepository),
			}),
		}
	}
	
	pub fn load(&self) -> bool {
		Load::new(self.repository.as_ref()).load()
	}
	
	pub fn retrieve_or_default<T: FromStr>(&self, key: &str, default: T) -> T {
		RetrieveDefault::new(self.repository.as_ref()).retrieve(key, default)
	}
	
	pub fn retrieve_or_default_with_error<T: FromStr>(&self, key: &str, default: T) -> Result<T, EnvUtilsError> {
		RetrieveDefaultError::new(self.repository.as_ref()).retrieve(key, default)
	}
	
	pub fn retrieve_with_error<T: FromStr>(&self, key: &str) -> Result<T, EnvUtilsError> {
		RetrieveError::new(self.repository.as_ref()).retrieve(key)
	}
}