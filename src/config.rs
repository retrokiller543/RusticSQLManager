use crate::CONFIG_FILE;
use clap::Args;
use config::Config;
use serde::{Deserialize, Serialize};
use std::io::Write;
use std::path::PathBuf;

#[derive(Args, Debug, Clone, Serialize, Deserialize)]
pub struct AppConfig {
    #[arg(short, long, default_value = "root")]
    pub user: String,
    #[arg(short = 'H', long, default_value = "localhost")]
    pub host: String,
}

/// Returns the full path to the configuration file.
///
/// This function constructs the path to the configuration file based on the operating system.
/// On Linux, it joins the HOME environment variable with the CONFIG_FILE constant.
/// On Windows, it joins the USERPROFILE environment variable with the CONFIG_FILE constant.
///
/// # Returns
///
/// * `PathBuf` - The full path to the configuration file.
pub fn full_path() -> PathBuf {
    #[cfg(target_os = "linux")]
        let path = PathBuf::new().join(env!("HOME")).join(CONFIG_FILE);
    #[cfg(target_os = "windows")]
        let path = PathBuf::new().join(env!("USERPROFILE")).join(CONFIG_FILE);
    path
}

/// Returns the application configuration.
///
/// This function retrieves the configuration for the application from a configuration file.
/// The path to the configuration file is obtained by calling the `full_path` function.
///
/// If the configuration file does not exist, it creates a new one by calling the `create_config_file` function.
///
/// The configuration file is expected to be in TOML format and contain the following keys:
/// * `user` - The username to use for the application. Defaults to "root" if not specified.
/// * `host` - The host to use for the application. Defaults to "localhost" if not specified.
///
/// # Returns
///
/// * `Result<Config, anyhow::Error>` - The application configuration wrapped in a `Result`. If an error occurred while reading or parsing the configuration file, it is returned as an `Err`.
pub fn get_config() -> anyhow::Result<Config> {
    let path = full_path();

    if !path.exists() {
        println!("Config file does not exist. Creating it...");
        create_config_file(&path)?;
    }

    let settings = Config::builder()
        .add_source(config::File::with_name(path.to_str().unwrap()))
        .set_default("user", "root")?
        .set_default("host", "localhost")?
        .build()?;
    Ok(settings)
}

/// Writes the application configuration to a file.
///
/// This function writes the provided application configuration to a configuration file.
/// The path to the configuration file is obtained by calling the `full_path` function.
///
/// If the configuration file does not exist, it creates a new one by calling the `create_config_file` function.
///
/// The configuration is written in TOML format.
///
/// # Arguments
///
/// * `app_config` - A reference to the application configuration to write.
///
/// # Returns
///
/// * `Result<(), anyhow::Error>` - Returns `Ok(())` if the configuration was successfully written. If an error occurred while writing the configuration, it is returned as an `Err`.
pub fn write_config(app_config: &AppConfig) -> anyhow::Result<()> {
    let path = full_path();

    if !path.exists() {
        println!("Config file does not exist. Creating it...");
        create_config_file(&path)?;
    }

    let mut file = std::fs::OpenOptions::new().write(true).open(path)?;

    file.write_all(toml::to_string(app_config)?.as_bytes())?;
    Ok(())
}

/// Creates a configuration file at the specified path.
///
/// This function checks if a file exists at the provided path. If not, it creates a new directory and file at that path.
///
/// # Arguments
///
/// * `path` - A reference to the path where the configuration file should be created.
///
/// # Returns
///
/// * `Result<(), anyhow::Error>` - Returns `Ok(())` if the configuration file was successfully created. If an error occurred while creating the directory or file, it is returned as an `Err`.
fn create_config_file(path: &PathBuf) -> anyhow::Result<()> {
    if !path.exists() {
        std::fs::create_dir_all(path.parent().unwrap())?;
        std::fs::File::create(path)?;
    }
    Ok(())
}