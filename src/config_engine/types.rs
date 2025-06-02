use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Корневая конфигурация - может содержать множество устройств
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ConfigFile {
    /// Глобальные настройки
    #[serde(default)]
    pub global_settings: GlobalSettings,
    
    /// Список всех устройств
    pub devices: HashMap<String, DeviceConfig>,
    
    /// Сценарии выполнения
    #[serde(default)]
    pub scenarios: HashMap<String, Scenario>,
}

/// Глобальные настройки для всех устройств
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GlobalSettings {
    #[serde(default = "default_max_retries")]
    pub max_retries: u32,
    #[serde(default = "default_command_delay")]
    pub command_delay_ms: u64,
    #[serde(default = "default_timeout")]
    pub default_timeout_seconds: u64,
    #[serde(default)]
    pub log_level: LogLevel,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "lowercase")]
pub enum LogLevel {
    Error,
    Warn,
    Info,
    Debug,
}

impl Default for LogLevel {
    fn default() -> Self { LogLevel::Info }
}

impl Default for GlobalSettings {
    fn default() -> Self {
        Self {
            max_retries: default_max_retries(),
            command_delay_ms: default_command_delay(),
            default_timeout_seconds: default_timeout(),
            log_level: LogLevel::Info,
        }
    }
}

/// Конфигурация одного устройства
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DeviceConfig {
    pub device_info: DeviceInfo,
    pub connection: ConnectionConfig,
    pub credentials: CredentialsConfig,
    
    /// Последовательность команд для выполнения
    pub command_sequence: Vec<CommandStep>,
    
    /// Специфичные настройки устройства
    #[serde(default)]
    pub device_settings: DeviceSettings,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DeviceInfo {
    pub name: String,
    pub device_type: DeviceType,
    pub model: String,
    pub description: Option<String>,
    pub vendor: String,
}

/// Типы поддерживаемых устройств
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "lowercase")]
pub enum DeviceType {
    Router,     // Маршрутизаторы (Eltex, Cisco, etc.)
    Switch,     // Коммутаторы  
    Server,     // Серверы (Linux, Windows)
    Firewall,   // Межсетевые экраны
    Custom,     // Пользовательский тип
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ConnectionConfig {
    pub transport: TransportType,
    pub host: String,  // может быть IP, hostname, или device path
    pub port: Option<u16>,
    pub baud_rate: Option<u32>,
    pub timeout_seconds: Option<u64>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "lowercase")]
pub enum TransportType {
    Serial,
    Telnet,
    Ssh,
    Http,
    Https,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CredentialsConfig {
    pub username: String,
    pub password: String,
    pub enable_password: Option<String>,
    pub ssh_key_path: Option<String>,
}

/// Один шаг выполнения команды
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CommandStep {
    pub name: String,
    pub step_type: StepType,
    pub timeout_seconds: Option<u64>,
    pub retry_count: Option<u32>,
    pub on_error: ErrorAction,
    pub description: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(tag = "type", content = "data")]
pub enum StepType {
    /// Выполнить команду и ожидать ответ
    #[serde(rename = "command")]
    Command { 
        command: String,
        expected_prompt: Option<String>,
    },
    
    /// Ожидать определенный промпт
    #[serde(rename = "wait_prompt")]
    WaitPrompt { 
        prompt: String 
    },
    
    /// Пауза
    #[serde(rename = "delay")]
    Delay { 
        milliseconds: u64 
    },
    
    /// Проверить ответ (содержит ли определенный текст)
    #[serde(rename = "check_response")]
    CheckResponse { 
        contains: String,
        fail_if_not_found: bool,
    },
    
    /// Войти в систему
    #[serde(rename = "login")]
    Login,
    
    /// Выйти из системы
    #[serde(rename = "logout")]
    Logout,
    
    /// Высокоуровневая операция устройства
    #[serde(rename = "device_operation")]
    DeviceOperation {
        operation: String,
        parameters: HashMap<String, String>,
    },
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "lowercase")]
pub enum ErrorAction {
    /// Остановить выполнение
    Stop,
    /// Продолжить выполнение
    Continue,
    /// Попробовать еще раз
    Retry,
    /// Перейти к определенному шагу
    GotoStep(String),
}

impl Default for ErrorAction {
    fn default() -> Self { ErrorAction::Stop }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DeviceSettings {
    /// Специфичные промпты для устройства
    pub prompts: HashMap<String, String>,
    
    /// Дополнительные параметры
    #[serde(default)]
    pub extra_params: HashMap<String, String>,
    
    /// Команды инициализации
    #[serde(default)]
    pub init_commands: Vec<String>,
}

impl Default for DeviceSettings {
    fn default() -> Self {
        Self {
            prompts: HashMap::new(),
            extra_params: HashMap::new(),
            init_commands: Vec::new(),
        }
    }
}

/// Сценарий выполнения (группа команд для нескольких устройств)
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Scenario {
    pub name: String,
    pub description: Option<String>,
    
    /// Устройства для выполнения (по именам или фильтрам)
    pub target_devices: TargetDevices,
    
    /// Дополнительные команды сценария
    pub commands: Vec<CommandStep>,
    
    /// Выполнять параллельно или последовательно
    #[serde(default)]
    pub execution_mode: ExecutionMode,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(tag = "type")]
pub enum TargetDevices {
    /// Конкретные устройства по именам
    #[serde(rename = "specific")]
    Specific { devices: Vec<String> },
    
    /// Фильтр по типу устройства  
    #[serde(rename = "by_type")]
    ByType { device_type: DeviceType },
    
    /// Фильтр по производителю
    #[serde(rename = "by_vendor")]
    ByVendor { vendor: String },
    
    /// Все устройства
    #[serde(rename = "all")]
    All,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "lowercase")]
pub enum ExecutionMode {
    Sequential,  // Последовательно
    Parallel,    // Параллельно
}

impl Default for ExecutionMode {
    fn default() -> Self { ExecutionMode::Sequential }
}

// Вспомогательные функции для defaults
fn default_max_retries() -> u32 { 3 }
fn default_command_delay() -> u64 { 100 }
fn default_timeout() -> u64 { 30 }
