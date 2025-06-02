use crate::config_engine::types::{ConfigFile, DeviceConfig, StepType, ErrorAction, TransportType};
use crate::transport::{Transport, SerialTransport};
use crate::device::{DeviceCommands, EltexEsr200};
use crate::error::{Error, Result};
use std::time::Duration;
use std::thread;

/// Исполнительный движок для выполнения команд из конфигурации
pub struct ConfigExecutor {
    config: ConfigFile,
}

impl ConfigExecutor {
    /// Создает новый экземпляр движка с загруженной конфигурацией
    pub fn new(config: ConfigFile) -> Self {
        Self { config }
    }

    /// Выполняет команды для конкретного устройства
    pub fn execute_device(&self, device_id: &str) -> Result<ExecutionResult> {
        let device_config = self.config.devices.get(device_id)
            .ok_or_else(|| Error::ConfigValidation(format!("Устройство '{}' не найдено", device_id)))?;

        println!("🚀 Выполнение команд для устройства: {}", device_config.device_info.name);
        
        let mut result = ExecutionResult::new(device_id.to_string());
        
        // Создаем транспорт (заглушка, реальное подключение в create_device)
        let transport = self.create_transport(device_config)?;
        
        // Создаем устройство (здесь происходит реальное подключение)
        let mut device = self.create_device(transport, device_config)?;
        
        // Выполняем последовательность команд
        for step in &device_config.command_sequence {
            match self.execute_step(&mut device, step, device_config) {
                Ok(output) => {
                    result.add_success(step.name.clone(), output);
                    
                    // Пауза между командами
                    thread::sleep(Duration::from_millis(self.config.global_settings.command_delay_ms));
                }
                Err(e) => {
                    result.add_error(step.name.clone(), e.to_string());
                    
                    // Обрабатываем ошибку согласно настройкам
                    match &step.on_error {
                        ErrorAction::Stop => {
                            println!("❌ Остановка выполнения из-за ошибки в шаге '{}'", step.name);
                            break;
                        }
                        ErrorAction::Continue => {
                            println!("⚠️ Продолжение выполнения несмотря на ошибку в шаге '{}'", step.name);
                            continue;
                        }
                        ErrorAction::Retry => {
                            println!("🔄 Повтор шага '{}' из-за ошибки", step.name);
                            // TODO: Реализовать логику повтора
                            continue;
                        }
                        ErrorAction::GotoStep(_target) => {
                            println!("↗️ Переход к другому шагу из-за ошибки (не реализовано)");
                            continue;
                        }
                    }
                }
            }
        }
        
        // Закрываем соединение
        if let Err(e) = device.disconnect() {
            println!("⚠️ Ошибка при отключении: {}", e);
        }

        Ok(result)
    }

    /// Выполняет один шаг команды
    fn execute_step(
        &self, 
        device: &mut Box<dyn DeviceCommands>, 
        step: &crate::config_engine::types::CommandStep,
        device_config: &DeviceConfig
    ) -> Result<String> {
        println!("  🔧 Выполнение шага: {}", step.name);
        
        match &step.step_type {
            StepType::Login => {
                // Выполняем вход в систему
                self.execute_login(device.as_mut(), device_config)
            }
            StepType::Logout => {
                // Выполняем выход
                device.execute_command("exit")
            }
            StepType::Command { command, expected_prompt: _ } => {
                // Выполняем команду
                device.execute_command(command)
            }
            StepType::DeviceOperation { operation, parameters } => {
                // Выполняем высокоуровневую операцию устройства
                device.execute_device_operation(operation, parameters)
            }
            StepType::Delay { milliseconds } => {
                // Пауза
                println!("    ⏱️ Пауза {} мс", milliseconds);
                thread::sleep(Duration::from_millis(*milliseconds));
                Ok("Pause completed".to_string())
            }
            StepType::WaitPrompt { prompt: _ } => {
                // TODO: Реализовать ожидание промпта
                println!("    ⏳ Ожидание промпта (не реализовано)");
                Ok("Prompt wait completed".to_string())
            }
            StepType::CheckResponse { contains: _, fail_if_not_found: _ } => {
                // TODO: Реализовать проверку ответа
                println!("    ✅ Проверка ответа (не реализовано)");
                Ok("Response check completed".to_string())
            }
        }
    }

    /// Выполняет вход в систему
    fn execute_login(&self, device: &mut dyn DeviceCommands, device_config: &DeviceConfig) -> Result<String> {
        // Для простоты пока используем базовую логику
        // В реальности нужно будет адаптировать под разные типы устройств
        
        let username = &device_config.credentials.username;
        let password = &device_config.credentials.password;
        
        println!("    🔐 Вход в систему как '{}'", username);
        
        // Отправляем имя пользователя
        let _response1 = device.execute_command(username)?;
        
        // Отправляем пароль
        let response2 = device.execute_command(password)?;
        
        Ok(response2)
    }

    /// Создает транспорт согласно конфигурации
    fn create_transport(&self, device_config: &DeviceConfig) -> Result<Box<dyn Transport>> {
        match device_config.connection.transport {
            TransportType::Serial => {
                let baud_rate = device_config.connection.baud_rate.unwrap_or(115200);
                let transport = SerialTransport::new(device_config.connection.host.clone(), baud_rate);
                Ok(Box::new(transport))
            }
            TransportType::Telnet => {
                // TODO: Реализовать Telnet транспорт
                Err(Error::ConfigValidation("Telnet транспорт не реализован".to_string()))
            }
            TransportType::Ssh => {
                // TODO: Реализовать SSH транспорт
                Err(Error::ConfigValidation("SSH транспорт не реализован".to_string()))
            }
            TransportType::Http => {
                // TODO: Реализовать HTTP транспорт
                Err(Error::ConfigValidation("HTTP транспорт не реализован".to_string()))
            }
            TransportType::Https => {
                // TODO: Реализовать HTTPS транспорт
                Err(Error::ConfigValidation("HTTPS транспорт не реализован".to_string()))
            }
        }
    }

    /// Создает устройство согласно конфигурации
    fn create_device(&self, _transport: Box<dyn Transport>, device_config: &DeviceConfig) -> Result<Box<dyn DeviceCommands>> {
        match device_config.device_info.vendor.as_str() {
            "Eltex" => {
                // Для Eltex создаем новый транспорт напрямую
                if let TransportType::Serial = device_config.connection.transport {
                    let baud_rate = device_config.connection.baud_rate.unwrap_or(115200);
                    let mut serial_transport = SerialTransport::new(device_config.connection.host.clone(), baud_rate);
                    
                    // Подключаемся к транспорту
                    serial_transport.connect()?;
                    
                    // Устанавливаем таймаут
                    let timeout = Duration::from_secs(
                        device_config.connection.timeout_seconds
                            .unwrap_or(self.config.global_settings.default_timeout_seconds)
                    );
                    serial_transport.set_timeout(timeout)?;
                    
                    let eltex_device = EltexEsr200::new(serial_transport)?;
                    Ok(Box::new(eltex_device))
                } else {
                    Err(Error::ConfigValidation("Eltex поддерживает только Serial транспорт".to_string()))
                }
            }
            "Cisco" => {
                // TODO: Реализовать поддержку Cisco
                Err(Error::ConfigValidation("Cisco устройства не реализованы".to_string()))
            }
            "Ubuntu" => {
                // TODO: Реализовать поддержку Linux серверов
                Err(Error::ConfigValidation("Linux серверы не реализованы".to_string()))
            }
            vendor => {
                Err(Error::ConfigValidation(format!("Неподдерживаемый производитель: {}", vendor)))
            }
        }
    }

    /// Выполняет команды для всех устройств
    pub fn execute_all_devices(&self) -> Result<Vec<ExecutionResult>> {
        let mut results = Vec::new();
        
        for (device_id, _) in &self.config.devices {
            match self.execute_device(device_id) {
                Ok(result) => results.push(result),
                Err(e) => {
                    let mut error_result = ExecutionResult::new(device_id.clone());
                    error_result.add_error("connection".to_string(), e.to_string());
                    results.push(error_result);
                }
            }
        }
        
        Ok(results)
    }

    /// Возвращает ссылку на конфигурацию
    pub fn config(&self) -> &ConfigFile {
        &self.config
    }
}

/// Результат выполнения команд для одного устройства
#[derive(Debug, Clone)]
pub struct ExecutionResult {
    pub device_id: String,
    pub successful_commands: Vec<CommandResult>,
    pub failed_commands: Vec<CommandError>,
    pub execution_time: std::time::Duration,
}

#[derive(Debug, Clone)]
pub struct CommandResult {
    pub command_name: String,
    pub output: String,
    pub execution_time: std::time::Duration,
}

#[derive(Debug, Clone)]
pub struct CommandError {
    pub command_name: String,
    pub error_message: String,
}

impl ExecutionResult {
    fn new(device_id: String) -> Self {
        Self {
            device_id,
            successful_commands: Vec::new(),
            failed_commands: Vec::new(),
            execution_time: std::time::Duration::default(),
        }
    }

    fn add_success(&mut self, command_name: String, output: String) {
        self.successful_commands.push(CommandResult {
            command_name,
            output,
            execution_time: std::time::Duration::default(), // TODO: Измерять время
        });
    }

    fn add_error(&mut self, command_name: String, error_message: String) {
        self.failed_commands.push(CommandError {
            command_name,
            error_message,
        });
    }

    /// Возвращает true если все команды выполнены успешно
    pub fn is_success(&self) -> bool {
        self.failed_commands.is_empty()
    }

    /// Возвращает количество успешных команд
    pub fn success_count(&self) -> usize {
        self.successful_commands.len()
    }

    /// Возвращает количество неудачных команд
    pub fn error_count(&self) -> usize {
        self.failed_commands.len()
    }
} 