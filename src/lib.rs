pub use core::error::EnvUtilsError;
pub use ports::spi::env::EnvType;
pub use systems::env_service::EnvService;

mod core {
	pub mod error;
}

mod interactions {
	pub mod repositories {
		pub use env_repository::*;
		
		mod env_repository;
	}
	
	pub mod services {
		pub mod load {
			pub use load_env::Load;
			
			mod load_env;
		}
		
		pub mod retrieve {
			pub use retrieve_or_default::RetrieveDefault;
			pub use retrieve_or_default_with_error::RetrieveDefaultError;
			pub use retrieve_with_error::RetrieveError;
			
			mod retrieve_or_default;
			mod retrieve_or_default_with_error;
			mod retrieve_with_error;
		}
	}
	
	pub mod interfaces {
		pub use load::ILoad;
		pub use retrieve::*;
		
		mod load;
		mod retrieve;
	}
}

mod ports {
	pub mod spi {
		pub mod env {
			pub use dotenvy::DotenvyRepository;
			pub use env_type::EnvType;
			
			mod dotenvy;
			
			mod env_type;
		}
	}
}

mod systems {
	pub mod env_service;
}

