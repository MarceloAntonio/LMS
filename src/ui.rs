use crate::error::Result;
use dialoguer::{theme::ColorfulTheme, Confirm, Input, Select};

pub fn prompt_mode_selection() -> Result<usize> {
    let mode_options = vec![
        "Web Interface (llama-server - via Browser)",
        "CLI Terminal (llama-cli - direct via Terminal)",
    ];

    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("\nHow would you like to start the AI?")
        .items(&mode_options)
        .default(0)
        .interact()?;
    
    Ok(selection)
}

pub fn prompt_model_selection(model_list: &[String]) -> Result<usize> {
    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("\nSelect the model you want to load:")
        .items(model_list)
        .default(0)
        .interact()?;
        
    Ok(selection)
}

pub fn prompt_curated_model_selection(options: &[&str]) -> Result<usize> {
    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("\nSelect a model to download:")
        .items(options)
        .default(0)
        .interact()?;
        
    Ok(selection)
}

pub fn prompt_custom_model_url() -> Result<String> {
    let url: String = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Enter the direct URL to the .gguf file")
        .interact_text()?;
        
    Ok(url)
}

pub fn prompt_use_gpu() -> Result<bool> {
    let use_gpu = Confirm::with_theme(&ColorfulTheme::default())
        .with_prompt("\nDo you want to use the GPU to accelerate generation? (Recommended if you have a dedicated GPU)")
        .default(true)
        .interact()?;
        
    Ok(use_gpu)
}

pub fn prompt_context_size() -> Result<String> {
    let context_size: String = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("\nEnter the context window size")
        .default("4096".to_string())
        .interact_text()?;
        
    Ok(context_size)
}
