use dialoguer::{theme::ColorfulTheme, Select};
use std::env;
use std::error::Error;
use std::fs;
use std::path::Path;
use std::process::Command;

fn main() -> Result<(), Box<dyn Error>> {
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

    let mode_options = vec![
        "Web Interface (llama-server - via Browser)",
        "CLI Terminal (llama-cli - direct via Terminal)"
    ];
    
    let mode_selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("\nHow would you like to start the AI?")
        .items(&mode_options)
        .default(0)
        .interact()?;

    let mut model_list: Vec<String> = Vec::new();

    if let Ok(entries) = fs::read_dir(&models_path) {
        for entry in entries.flatten() {
            let file_path = entry.path();
            if file_path.is_file() {
                if let Some(extension) = file_path.extension() {
                    if extension == "gguf" {
                        if let Some(file_name) = file_path.file_name().and_then(|n| n.to_str()) {
                            model_list.push(file_name.to_string());
                        }
                    }
                }
            }
        }
    }

    if model_list.is_empty() {
        eprintln!("Error: No '.gguf' model found in directory '{}'.", models_path);
        return Ok(());
    }

    let model_selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("\nSelect the model you want to load:")
        .items(&model_list)
        .default(0)
        .interact()?;

    let selected_model = &model_list[model_selection];
    let full_model_path = Path::new(&models_path).join(selected_model);

    let exe_suffix = env::consts::EXE_SUFFIX;
    let server = Path::new(&llama_dir).join(format!("llama-server{}", exe_suffix));
    let cli = Path::new(&llama_dir).join(format!("llama-cli{}", exe_suffix));

    let mut process;

    if mode_selection == 0 {
        println!("\n[>] Starting llama-server with model: {}", selected_model);
        println!("[>] Access http://localhost:8080 in your browser.\n");
        
        process = Command::new(&server);
        process.arg("-m").arg(&full_model_path)
               .arg("-c").arg("4096")
               .arg("-t").arg("4")
               .arg("--port").arg("8080");
    } else {
        println!("\n[>] Starting llama-cli with model: {}", selected_model);
        println!("[>] Waiting for memory allocation...\n");
        
        process = Command::new(&cli);
        process.arg("-m").arg(&full_model_path)
               .arg("-c").arg("4096")
               .arg("-t").arg("4");
    }

    let mut child = process.spawn()?;
    child.wait()?;

    Ok(())
}