use super::LlamaRunner;
use crate::config::AppConfig;
use crate::error::{LmsError, Result};
use std::path::Path;
use std::process::Command;

pub struct CliRunner;

impl LlamaRunner for CliRunner {
    fn run(&self, config: &AppConfig, model_path: &Path, use_gpu: bool, context_size: &str) -> Result<()> {
        let cli_bin = config.cli_binary();
        
        let model_name = model_path.file_name().and_then(|n| n.to_str()).unwrap_or("unknown");
        println!("\n[>] Starting llama-cli with model: {}", model_name);
        println!("[>] Waiting for memory allocation...\n");
        
        let mut process = Command::new(&cli_bin);
        process.arg("-m").arg(model_path)
               .arg("-c").arg(context_size)
               .arg("-t").arg("4");
               
        if use_gpu {
            process.arg("-ngl").arg("99");
        }

        let mut child = process.spawn().map_err(|_| LmsError::ProcessError)?;
        child.wait().map_err(|_| LmsError::ProcessError)?;

        Ok(())
    }
}
