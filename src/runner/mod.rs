pub mod cli;
pub mod server;

use crate::config::AppConfig;
use crate::error::Result;
use std::path::Path;

pub trait LlamaRunner {
    fn run(&self, config: &AppConfig, model_path: &Path, use_gpu: bool, context_size: &str) -> Result<()>;
}
