use crate::config::AppConfig;
use crate::error::Result;
use indicatif::{ProgressBar, ProgressStyle};
use std::fs::{self, File};
use std::io::{Read, Write};
use std::path::Path;

pub fn download_file(url: &str, destination: &Path) -> Result<()> {
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

pub fn get_local_models(config: &AppConfig) -> Vec<String> {
    let mut model_list: Vec<String> = Vec::new();
    if let Ok(entries) = fs::read_dir(&config.models_path) {
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
    model_list
}

pub fn get_curated_models() -> Vec<(&'static str, &'static str, &'static str)> {
    vec![
        ("Phi-3 Mini 4K (2.4 GB) - Fast and capable", "https://huggingface.co/microsoft/Phi-3-mini-4k-instruct-gguf/resolve/main/Phi-3-mini-4k-instruct-q4.gguf", "Phi-3-mini-4k-instruct-q4.gguf"),
        ("Llama-3 8B Instruct Q4 (4.9 GB) - Excellent performance", "https://huggingface.co/QuantFactory/Meta-Llama-3-8B-Instruct-GGUF/resolve/main/Meta-Llama-3-8B-Instruct.Q4_K_M.gguf", "Meta-Llama-3-8B-Instruct.Q4_K_M.gguf"),
        ("Type a custom URL", "", ""),
    ]
}

pub fn download_new_model(config: &AppConfig, url: &str, filename: &str) -> Result<()> {
    fs::create_dir_all(&config.models_path)?;
    let dest_path = config.models_path.join(filename);
    
    println!("\nDownloading {}...", filename);
    download_file(url, &dest_path)?;
    println!("\nModel downloaded successfully! Please restart LMS to use it.");
    Ok(())
}
