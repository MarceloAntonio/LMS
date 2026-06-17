use lms::config::AppConfig;
use lms::model::{download_new_model, get_curated_models, get_local_models};
use lms::runner::{cli::CliRunner, server::ServerRunner, LlamaRunner};
use lms::setup::setup_llama_cpp;
use lms::ui::{
    prompt_context_size, prompt_curated_model_selection, prompt_custom_model_url,
    prompt_mode_selection, prompt_model_selection, prompt_use_gpu,
};
use lms::error::Result;

fn main() -> Result<()> {
    // 1. Initialize configuration
    let config = AppConfig::new();

    // 2. Select execution mode (Strategy Pattern setup)
    let mode_selection = prompt_mode_selection()?;
    let runner: Box<dyn LlamaRunner> = if mode_selection == 0 {
        Box::new(ServerRunner)
    } else {
        Box::new(CliRunner)
    };

    // 3. Model management
    let mut model_list = get_local_models(&config);
    let download_option = "📥 Download new model".to_string();
    model_list.push(download_option.clone());

    let model_selection = prompt_model_selection(&model_list)?;

    if model_list[model_selection] == download_option {
        let curated_models = get_curated_models();
        let curated_options: Vec<&str> = curated_models.iter().map(|(name, _, _)| *name).collect();
        
        let download_selection = prompt_curated_model_selection(&curated_options)?;

        let (url, filename) = if download_selection == 2 {
            let custom_url = prompt_custom_model_url()?;
            let filename = custom_url.split('/').last().unwrap_or("custom_model.gguf").to_string();
            (custom_url, filename)
        } else {
            let (_, url, filename) = curated_models[download_selection];
            (url.to_string(), filename.to_string())
        };

        download_new_model(&config, &url, &filename)?;
        return Ok(());
    }

    let selected_model = &model_list[model_selection];
    let full_model_path = config.models_path.join(selected_model);

    // 4. Configuration prompts
    let use_gpu = prompt_use_gpu()?;
    let context_size = prompt_context_size()?;

    // 5. Verify Llama binaries
    let server_bin = config.server_binary();
    let cli_bin = config.cli_binary();

    if !server_bin.exists() || !cli_bin.exists() {
        setup_llama_cpp(&config)?;
    }

    if !server_bin.exists() || !cli_bin.exists() {
        eprintln!("\nError: llama.cpp binaries not found. Cannot proceed.");
        return Ok(());
    }

    // 6. Execute via selected runner strategy
    runner.run(&config, &full_model_path, use_gpu, &context_size)?;

    Ok(())
}