use crate::transport::Transport;
use crate::device::commands::DeviceCommands;
use crate::error::types::Result;
use std::time::Duration;

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
}

// Специфичные методы для Eltex
impl<T: Transport> EltexEsr200<T> {
    pub fn login(&mut self, username: &str, password: &str) -> Result<String> {
        let _response = self.execute_command(username)?;
        let response = self.execute_command(password)?;
        Ok(response)
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