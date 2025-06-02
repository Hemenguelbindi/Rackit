use crate::error::types::Result;
use std::collections::HashMap;

pub trait DeviceCommands {
    fn execute_command(&mut self, command: &str) -> Result<String>;
    fn disconnect(&mut self) -> Result<()>;
    
    /// ГЛАВНЫЙ МЕТОД: Выполнить высокоуровневую операцию устройства
    /// Этот метод вызывается из конфигурационного движка
    fn execute_device_operation(&mut self, operation: &str, params: &HashMap<String, String>) -> Result<String>;
}



