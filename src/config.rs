use std::env;
use std::path::PathBuf;

#[derive(Debug, Clone)]
pub struct AppConfig {
    pub llama_dir: PathBuf,
    pub models_path: PathBuf,
}

impl AppConfig {
    pub fn new() -> Self {
        let mut llama_dir = String::from("./llama");
        let mut models_path = String::from("models");

        let args: Vec<String> = env::args().collect();
        let mut i = 1;
        
        while i < args.len() {
            match args[i].as_str() {
                "--llama" | "-l" => {
                    if i + 1 < args.len() {
                        llama_dir = args[i + 1].clone();
                        i += 1;
                    }
                }
                "--models" | "-m" => {
                    if i + 1 < args.len() {
                        models_path = args[i + 1].clone();
                        i += 1;
                    }
                }
                _ => {}
            }
            i += 1;
        }

        Self {
            llama_dir: PathBuf::from(llama_dir),
            models_path: PathBuf::from(models_path),
        }
    }

    pub fn server_binary(&self) -> PathBuf {
        let exe_suffix = env::consts::EXE_SUFFIX;
        self.llama_dir.join(format!("llama-server{}", exe_suffix))
    }

    pub fn cli_binary(&self) -> PathBuf {
        let exe_suffix = env::consts::EXE_SUFFIX;
        self.llama_dir.join(format!("llama-cli{}", exe_suffix))
    }
}
