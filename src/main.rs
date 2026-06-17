use dialoguer::{theme::ColorfulTheme, Select, Input};
use indicatif::{ProgressBar, ProgressStyle};
use std::env;
use std::error::Error;
use std::fs::{self, File};
use std::io::{Read, Write};
use std::path::Path;
use std::process::Command;

fn download_model(url: &str, destination: &Path) -> Result<(), Box<dyn Error>> {
    let response = ureq::get(url).call()?;
    
    let total_size = response
        .headers()
        .get("content-length")
        .and_then(|val| val.to_str().ok())
        .and_then(|s| s.parse::<u64>().ok())
        .unwrap_or(0);
        
    let pb = ProgressBar::new(total_size);
    pb.set_style(ProgressStyle::default_bar()
        .template("{spinner:.green} [{elapsed_precise}] [{wide_bar:.cyan/blue}] {bytes}/{total_bytes} ({eta})")
        .unwrap()
        .progress_chars("#>-"));

    let mut source = response.into_body().into_reader();
    let mut dest = File::create(destination)?;

    let mut buffer = [0; 8192];
    loop {
        let bytes_read = source.read(&mut buffer)?;
        if bytes_read == 0 {
            break;
        }
        dest.write_all(&buffer[..bytes_read])?;
        pb.inc(bytes_read as u64);
    }
    pb.finish_with_message("Download complete");
    Ok(())
}

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

    let download_option = "📥 Download new model".to_string();
    model_list.push(download_option.clone());

    let model_selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("\nSelect the model you want to load:")
        .items(&model_list)
        .default(0)
        .interact()?;

    if model_list[model_selection] == download_option {
        let curated_models = vec![
            ("Phi-3 Mini 4K (2.4 GB) - Fast and capable", "https://huggingface.co/microsoft/Phi-3-mini-4k-instruct-gguf/resolve/main/Phi-3-mini-4k-instruct-q4.gguf", "Phi-3-mini-4k-instruct-q4.gguf"),
            ("Llama-3 8B Instruct Q4 (4.9 GB) - Excellent performance", "https://huggingface.co/QuantFactory/Meta-Llama-3-8B-Instruct-GGUF/resolve/main/Meta-Llama-3-8B-Instruct.Q4_K_M.gguf", "Meta-Llama-3-8B-Instruct.Q4_K_M.gguf"),
            ("Type a custom URL", "", ""),
        ];

        let curated_options: Vec<&str> = curated_models.iter().map(|(name, _, _)| *name).collect();
        let download_selection = Select::with_theme(&ColorfulTheme::default())
            .with_prompt("\nSelect a model to download:")
            .items(&curated_options)
            .default(0)
            .interact()?;

        let (url, filename) = if download_selection == 2 {
            let url: String = Input::with_theme(&ColorfulTheme::default())
                .with_prompt("Enter the direct URL to the .gguf file")
                .interact_text()?;
            
            let filename = url.split('/').last().unwrap_or("custom_model.gguf").to_string();
            (url, filename)
        } else {
            let (_, url, filename) = curated_models[download_selection];
            (url.to_string(), filename.to_string())
        };

        fs::create_dir_all(&models_path)?;
        let dest_path = Path::new(&models_path).join(&filename);
        
        println!("\nDownloading {}...", filename);
        if let Err(e) = download_model(&url, &dest_path) {
            eprintln!("Error downloading model: {}", e);
            return Ok(());
        }

        println!("\nModel downloaded successfully! Please restart LMS to use it.");
        return Ok(());
    }

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