use crate::transport::Transport;
use crate::device::commands::DeviceCommands;
use crate::error::types::Result;
use std::time::Duration;
use std::collections::HashMap;

// Структура с generic параметром
pub struct EltexEsr200<T: Transport> {
    transport: T,
}

impl<T: Transport> EltexEsr200<T> {
    pub fn new(mut transport: T) -> Result<Self> {
        // Отправляем Enter для получения приглашения
        transport.send(b"\r\n")?;
        std::thread::sleep(Duration::from_secs(1));
        
        // Читаем приветствие до промпта
        let welcome = transport.receive_until_prompt("esr-200#")?;
        println!("DEBUG: Приветствие:\n{}", welcome);
        
        Ok(Self { transport })
    }
}

// Реализация трейта DeviceCommands
impl<T: Transport> DeviceCommands for EltexEsr200<T> {
    fn execute_command(&mut self, command: &str) -> Result<String> {
        println!("DEBUG: Отправляем команду: '{}'", command);
        
        self.transport.send_string(command)?;
        
        // Читаем ВСЕ данные до промпта
        let response = self.transport.receive_until_prompt("esr-200#")?;
        
        println!("DEBUG: Полный ответ:\n'{}'", response);
        Ok(response)
    }

    fn disconnect(&mut self) -> Result<()> {
        // Сначала выходим из системы
        let _ = self.logout();
        
        // Потом разрываем соединение
        self.transport.disconnect()
    }

    fn execute_device_operation(&mut self, operation: &str, params: &HashMap<String, String>) -> Result<String> {
        match operation {
            "start_settings" => {
                let username = params.get("username")
                    .ok_or_else(|| crate::error::types::Error::ConfigValidation("Missing username parameter".to_string()))?;
                let password = params.get("password")
                    .ok_or_else(|| crate::error::types::Error::ConfigValidation("Missing password parameter".to_string()))?;
                let new_password = params.get("new_password")
                    .ok_or_else(|| crate::error::types::Error::ConfigValidation("Missing new_password parameter".to_string()))?;
                self.start_settings(username, password, new_password)
            }
            "show_system" => self.show_system(),
            _ => Err(crate::error::types::Error::ConfigValidation(
                format!("Unknown operation '{}' for Eltex ESR-200", operation)
            ))
        }
    }
}

// Специфичные методы для Eltex
impl<T: Transport> EltexEsr200<T> {
    pub fn start_settings(&mut self, username: &str, password: &str, new_password: &str) -> Result<String> {
        // Логинимся с дефолтными данными
        let _login_response = self.login(username, password)?;
        
        // Входим в режим конфигурации
        self.execute_command("configure")?;
        
        // Меняем пароль пользователя
        self.execute_command(&format!("username {}", username))?;
        
        // Выходим из режима конфигурации
        self.execute_command(&format!("password {}", new_password))?;
        
        self.execute_command("exit")?;
        self.execute_command("exit")?;
        // Сохраняем конфигурацию
        let _save_response = self.save_config()?;
        
        // Выходим из системы
        let _logout_response = self.logout()?;
        
        Ok("Initial setup completed: password changed and configuration saved".to_string())
    }

    pub fn login(&mut self, username: &str, password: &str) -> Result<String> {
        let _response = self.execute_command(username)?;
        let response = self.execute_command(password)?;
        Ok(response)
    }

    pub fn configure_interface(&mut self, interface: &str, ip: &str, mask: &str) -> Result<String> {
        self.execute_command("configure")?;
        let result = self.execute_command(&format!("interface {} ip address {}/{}", interface, ip, mask))?;
        self.execute_command("end")?;
        Ok(result)
    }

    fn save_config(&mut self) -> Result<String> {
        self.execute_command("write")
    }

    pub fn show_system(&mut self) -> Result<String> {
        let response = self.execute_command("show system")?;
        
        let cleaned = response
            .lines()
            .skip_while(|line| line.trim().is_empty() || line.contains("show system"))
            .take_while(|line| !line.contains("esr-200#"))
            .collect::<Vec<_>>()
            .join("\n");
        
        Ok(cleaned.trim().to_string())
    }

    // Новые методы для выхода
    pub fn logout(&mut self) -> Result<String> {
        self.execute_command("exit")
    }
}