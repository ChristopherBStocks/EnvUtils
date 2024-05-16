use thiserror::Error;

#[derive(Debug, Error, PartialEq)]
pub enum EnvUtilsError {
	#[error("Environment variable not found: {0}")]
	VarNotFound(String),
	#[error("Environment file unable to loadA: {0}")]
	FileLoadError(String),
	#[error("Environment variable unable to loadA: {0}")]
	VarLoadError(String),
	#[error("Environment variable was unable to be parsed: {0}")]
	VarParseError(String),
	#[error("No Default Provided: {0}")]
	NoDefaultProvided(String),
	#[error("Provider Fault")]
	ProviderFault(),
}