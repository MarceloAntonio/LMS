use std::io;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum LmsError {
    #[error("I/O Error: {0}")]
    Io(#[from] io::Error),
    
    #[error("Network Error: {0}")]
    Network(#[from] ureq::Error),

    #[error("Dialoguer Error: {0}")]
    Dialoguer(#[from] dialoguer::Error),
    
    #[error("Zip Error: {0}")]
    Zip(#[from] zip::result::ZipError),

    #[error("JSON Error: {0}")]
    Json(#[from] serde_json::Error),
    
    #[error("Model not found: {0}")]
    ModelNotFound(String),
    
    #[error("Llama.cpp not found. Setup was aborted.")]
    SetupAborted,
    
    #[error("Failed to spawn process")]
    ProcessError,

    #[error("No compatible asset found in the latest release")]
    AssetNotFound,
}

pub type Result<T> = std::result::Result<T, LmsError>;
