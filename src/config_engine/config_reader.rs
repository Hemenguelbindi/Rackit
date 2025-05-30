use std::path::Path;
use crate::error::Result;
use crate::config_engine::types::ConfigFile;

pub trait ConfigReader {
    /// Читает конфигурацию из файла
    fn read_from_file<P: AsRef<Path>>(path: P) -> Result<ConfigFile>;
    
    /// Читает конфигурацию из строки
    fn read_from_str(content: &str) -> Result<ConfigFile>;
    
    /// Возвращает поддерживаемые расширения файлов
    fn supported_extensions() -> &'static [&'static str];
    
    /// Валидация конфигурации
    fn validate(config: &ConfigFile) -> Result<()> {
        // Базовая валидация
        if config.devices.is_empty() {
            return Err(crate::error::Error::ConfigValidation(
                "Конфигурация должна содержать хотя бы одно устройство".to_string()
            ));
        }
        
        // Валидируем каждое устройство
        for (device_name, device_config) in &config.devices {
            if device_config.device_info.name.is_empty() {
                return Err(crate::error::Error::ConfigValidation(
                    format!("Имя устройства '{}' не может быть пустым", device_name)
                ));
            }
            
            if device_config.connection.host.is_empty() {
                return Err(crate::error::Error::ConfigValidation(
                    format!("Хост для устройства '{}' не может быть пустым", device_name)
                ));
            }
            
            if device_config.credentials.username.is_empty() {
                return Err(crate::error::Error::ConfigValidation(
                    format!("Имя пользователя для устройства '{}' не может быть пустым", device_name)
                ));
            }
            
            if device_config.command_sequence.is_empty() {
                return Err(crate::error::Error::ConfigValidation(
                    format!("Устройство '{}' должно содержать хотя бы одну команду", device_name)
                ));
            }
        }
        
        Ok(())
    }
}