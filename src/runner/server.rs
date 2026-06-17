use super::LlamaRunner;
use crate::config::AppConfig;
use crate::error::{LmsError, Result};
use std::path::Path;
use std::process::Command;

pub struct ServerRunner;

impl LlamaRunner for ServerRunner {
    fn run(&self, config: &AppConfig, model_path: &Path, use_gpu: bool, context_size: &str) -> Result<()> {
        let server_bin = config.server_binary();
        
        let model_name = model_path.file_name().and_then(|n| n.to_str()).unwrap_or("unknown");
        println!("\n[>] Starting llama-server with model: {}", model_name);
        println!("[>] Access http://localhost:8080 in your browser.\n");
        
        let mut process = Command::new(&server_bin);
        process.arg("-m").arg(model_path)
               .arg("-c").arg(context_size)
               .arg("-t").arg("4")
               .arg("--port").arg("8080");
               
        if use_gpu {
            process.arg("-ngl").arg("99");
        }

        let mut child = process.spawn().map_err(|_| LmsError::ProcessError)?;
        child.wait().map_err(|_| LmsError::ProcessError)?;

        Ok(())
    }
}
