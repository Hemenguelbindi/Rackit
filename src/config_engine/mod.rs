pub mod config_reader;
pub mod types;
pub mod toml_reader;
pub mod executor;

pub use config_reader::ConfigReader;
pub use types::*;
pub use toml_reader::TomlReader;
pub use executor::{ConfigExecutor, ExecutionResult};

// Удобная функция для автоматического определения формата
use std::path::Path;
use crate::error::{Error, Result};

/// Автоматически определяет формат файла и загружает конфигурацию
pub fn load_config<P: AsRef<Path>>(path: P) -> Result<ConfigFile> {
    let path = path.as_ref();
    
    match path.extension().and_then(|ext| ext.to_str()) {
        Some("toml") => TomlReader::read_from_file(path),
        Some(ext) => Err(Error::UnsupportedFormat(ext.to_string())),
        None => Err(Error::UnsupportedFormat("no extension".to_string())),
    }
}

/// Создает движок выполнения из файла конфигурации
pub fn create_executor<P: AsRef<Path>>(config_path: P) -> Result<ConfigExecutor> {
    let config = load_config(config_path)?;
    Ok(ConfigExecutor::new(config))
}

/// Создает пример конфигурационного файла (только для демонстрации)
pub fn create_example_config<P: AsRef<Path>>(path: P) -> Result<()> {
    let example_content = TomlReader::create_example_config();
    std::fs::write(path, example_content)?;
    Ok(())
}




