use anyhow::Result;
use dialoguer::{theme::ColorfulTheme, Select, Confirm};
use serde_json::Value;
use std::fs;
use std::path::Path;

fn main() -> Result<()> {
    let switcher_dir = dirs::home_dir()
        .ok_or_else(|| anyhow::anyhow!("Could not find home directory"))?
        .join(".claude_switcher");

    if !switcher_dir.exists() {
        println!("Claude switcher directory not found at: {:?}", switcher_dir);
        println!("Please create the directory and add provider settings files.");
        return Ok(());
    }

    display_current_claude_settings();

    let providers = discover_providers(&switcher_dir)?;

    if providers.is_empty() {
        println!("No provider settings found in: {:?}", switcher_dir);
        println!("Please add settings files with format: settings_<provider_name>.json");
        return Ok(());
    }

    //println!("Available Providers:");
    //for (i, provider) in providers.iter().enumerate() {
    //    println!("{}. {}", i + 1, provider);
    //}

    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Please choose the provider")
        .default(0)
        .items(&providers)
        .interact()?;

    let selected_provider = &providers[selection];
    let source_file = switcher_dir.join(format!("settings_{}.json", selected_provider));
    let target_dir = dirs::home_dir()
        .ok_or_else(|| anyhow::anyhow!("Could not find home directory"))?
        .join(".claude");
    let target_file = target_dir.join("settings.json");

    if target_file.exists() {
        let overwrite = Confirm::with_theme(&ColorfulTheme::default())
            .with_prompt("The .claude/settings file already exists. Are you sure you want to overwrite it?")
            .default(true)
            .interact()?;

        if !overwrite {
            println!("Operation cancelled.");
            return Ok(());
        }
    }

    if !target_dir.exists() {
        fs::create_dir_all(&target_dir)?;
    }

    fs::copy(&source_file, &target_file)?;
    println!("Switch successful! Current provider: {}", selected_provider);

    Ok(())
}

fn discover_providers(switcher_dir: &Path) -> Result<Vec<String>> {
    let mut providers = Vec::new();

    if let Ok(entries) = fs::read_dir(switcher_dir) {
        for entry in entries {
            if let Ok(entry) = entry {
                let path = entry.path();
                if path.is_file() {
                    if let Some(file_name) = path.file_name().and_then(|n| n.to_str()) {
                        if file_name.starts_with("settings_") && file_name.ends_with(".json") {
                            let provider_name = file_name
                                .trim_start_matches("settings_")
                                .trim_end_matches(".json");
                            providers.push(provider_name.to_string());
                        }
                    }
                }
            }
        }
    }

    providers.sort();
    Ok(providers)
}

fn display_current_claude_settings() {
    let claude_dir = match dirs::home_dir() {
        Some(home) => home.join(".claude"),
        None => {
            println!("Could not find home directory");
            return;
        }
    };

    let settings_file = claude_dir.join("settings.json");

    let mut base_url = "None".to_string();
    let mut model_name = "None".to_string();

    if settings_file.exists() {
        match fs::read_to_string(&settings_file) {
            Ok(content) => {
                match serde_json::from_str::<Value>(&content) {
                    Ok(json) => {
                        if let Some(env) = json.get("env") {
                            if let Some(url) = env.get("ANTHROPIC_BASE_URL") {
                                if let Some(url_str) = url.as_str() {
                                    base_url = url_str.to_string();
                                }
                            }
                            if let Some(model) = env.get("ANTHROPIC_MODEL") {
                                if let Some(model_str) = model.as_str() {
                                    model_name = model_str.to_string();
                                }
                            }
                        }
                    }
                    Err(_) => {
                        // JSON parsing failed, keep default "None" values
                    }
                }
            }
            Err(_) => {
                // File read failed, keep default "None" values
            }
        }
    }

    println!("Current Claude Settings:");
    println!("- ANTHROPIC_BASE_URL: {}", base_url);
    println!("- ANTHROPIC_MODEL: {}", model_name);
    println!("");
}
