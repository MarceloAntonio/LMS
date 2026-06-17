use crate::config::AppConfig;
use crate::error::Result;
use crate::model::download_file;
use serde::Deserialize;
use std::env;
use std::fs::{self, File};

#[derive(Deserialize, Debug)]
struct Asset {
    name: String,
    browser_download_url: String,
}

#[derive(Deserialize, Debug)]
struct Release {
    assets: Vec<Asset>,
}

pub fn setup_llama_cpp(config: &AppConfig) -> Result<()> {
    let llama_dir = &config.llama_dir;
    println!("\nFetching latest release info from GitHub...");
    let response = ureq::get("https://api.github.com/repos/ggml-org/llama.cpp/releases/latest")
        .header("User-Agent", "lms-app")
        .call()?;

    let release: Release = serde_json::from_reader(response.into_body().into_reader())?;

    let is_windows = env::consts::OS == "windows";
    let target_str = if is_windows { "win-vulkan-x64.zip" } else { "ubuntu-x64.tar.gz" };

    let asset = release.assets.into_iter().find(|a| a.name.contains(target_str));
    
    if let Some(asset) = asset {
        fs::create_dir_all(llama_dir)?;
        let archive_path = llama_dir.join(&asset.name);

        println!("\nDownloading {}...", asset.name);
        download_file(&asset.browser_download_url, &archive_path)?;

        println!("\nExtracting...");
        let file = File::open(&archive_path)?;

        if is_windows {
            let mut archive = zip::ZipArchive::new(file)?;
            for i in 0..archive.len() {
                let mut file = archive.by_index(i)?;
                let Some(enclosed_name) = file.enclosed_name() else { continue; };
                let Some(file_name) = enclosed_name.file_name() else { continue; };

                if (*file.name()).ends_with('/') {
                    continue;
                }

                let outpath = llama_dir.join(file_name);
                let mut outfile = File::create(&outpath)?;
                std::io::copy(&mut file, &mut outfile)?;
            }
        } else {
            let tar = flate2::read::GzDecoder::new(file);
            let mut archive = tar::Archive::new(tar);
            archive.unpack(llama_dir)?;

            if let Ok(entries) = fs::read_dir(llama_dir) {
                for entry in entries.flatten() {
                    let path = entry.path();
                    if path.is_dir() {
                        if let Ok(inner_entries) = fs::read_dir(&path) {
                            for inner_entry in inner_entries.flatten() {
                                let inner_path = inner_entry.path();
                                if let Some(file_name) = inner_path.file_name() {
                                    let dest = llama_dir.join(file_name);
                                    let _ = fs::rename(&inner_path, &dest);
                                }
                            }
                        }
                        let _ = fs::remove_dir_all(&path);
                    }
                }
            }
        }

        // Cleanup archive
        let _ = fs::remove_file(archive_path);

        // Make executable on linux
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            for bin in &["llama-server", "llama-cli"] {
                let bin_path = llama_dir.join(bin);
                if bin_path.exists() {
                    if let Ok(metadata) = fs::metadata(&bin_path) {
                        let mut perms = metadata.permissions();
                        perms.set_mode(0o755);
                        let _ = fs::set_permissions(&bin_path, perms);
                    }
                }
            }
        }

        println!("Setup complete!");
        Ok(())
    } else {
        Err(crate::error::LmsError::AssetNotFound)
    }
}
