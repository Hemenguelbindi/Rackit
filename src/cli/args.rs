use clap::{Parser, Subcommand};
use std::path::PathBuf;

/// Rackit - универсальный движок конфигурации сетевого оборудования
#[derive(Parser)]
#[command(name = "rackit")]
#[command(about = "Универсальный движок конфигурации для сетевого оборудования")]
#[command(version = "0.1.0")]
#[command(long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
    
    /// Уровень детализации вывода
    #[arg(short, long, action = clap::ArgAction::Count)]
    pub verbose: u8,
    
    /// Тихий режим (только ошибки)
    #[arg(short, long)]
    pub quiet: bool,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Выполнить конфигурацию (как ansible-playbook)
    Run {
        /// Путь к файлу конфигурации
        #[arg(value_name = "CONFIG_FILE")]
        config: PathBuf,
        
        /// Выполнить только для конкретного устройства
        #[arg(short, long)]
        device: Option<String>,
        
        /// Выполнить только конкретный сценарий
        #[arg(short, long)]
        scenario: Option<String>,
        
        /// Сухой запуск (только валидация, не выполнять команды)
        #[arg(long)]
        dry_run: bool,
        
        /// Параллельное выполнение для всех устройств
        #[arg(short, long)]
        parallel: bool,
        
        /// Максимальное количество параллельных соединений
        #[arg(long, default_value = "5")]
        max_parallel: usize,
        
        /// Продолжить выполнение даже при ошибках
        #[arg(long)]
        ignore_errors: bool,
    },
    
    /// Валидировать конфигурацию (как terraform validate)
    Validate {
        /// Путь к файлу конфигурации
        #[arg(value_name = "CONFIG_FILE")]
        config: PathBuf,
        
        /// Строгая валидация (проверка доступности устройств)
        #[arg(short, long)]
        strict: bool,
    },
    
    /// Показать план выполнения (как terraform plan)
    Plan {
        /// Путь к файлу конфигурации
        #[arg(value_name = "CONFIG_FILE")]
        config: PathBuf,
        
        /// Фильтр по устройству
        #[arg(short, long)]
        device: Option<String>,
        
        /// Фильтр по сценарию
        #[arg(short, long)]
        scenario: Option<String>,
        
        /// Показать подробную информацию
        #[arg(long)]
        detailed: bool,
    },
    
    /// Список устройств и сценариев
    List {
        /// Путь к файлу конфигурации
        #[arg(value_name = "CONFIG_FILE")]
        config: PathBuf,
        
        /// Показать только устройства
        #[arg(long)]
        devices: bool,
        
        /// Показать только сценарии
        #[arg(long)]
        scenarios: bool,
        
        /// Формат вывода
        #[arg(long, default_value = "table")]
        format: OutputFormat,
    },
    
    /// Создать пример конфигурации
    Example {
        /// Путь для создания примера
        #[arg(value_name = "OUTPUT_FILE", default_value = "rackit-config.toml")]
        output: PathBuf,
        
        /// Тип примера
        #[arg(short, long, default_value = "full")]
        template: ExampleTemplate,
        
        /// Перезаписать существующий файл
        #[arg(short, long)]
        force: bool,
    },
    
    /// Проверить состояние устройств
    Check {
        /// Путь к файлу конфигурации
        #[arg(value_name = "CONFIG_FILE")]
        config: PathBuf,
        
        /// Фильтр по устройству
        #[arg(short, long)]
        device: Option<String>,
        
        /// Только проверка подключения
        #[arg(long)]
        ping_only: bool,
        
        /// Таймаут для проверки (в секундах)
        #[arg(long, default_value = "10")]
        timeout: u64,
    },
    
    /// Выполнить интерактивную команду на устройстве
    Shell {
        /// Путь к файлу конфигурации
        #[arg(value_name = "CONFIG_FILE")]
        config: PathBuf,
        
        /// ID устройства
        #[arg(short, long)]
        device: String,
        
        /// Команда для выполнения (если не указана - интерактивный режим)
        #[arg(short, long)]
        command: Option<String>,
    },
}

#[derive(Clone, clap::ValueEnum)]
pub enum OutputFormat {
    /// Таблица (по умолчанию)
    Table,
    /// JSON формат
    Json,
    /// YAML формат
    Yaml,
    /// Простой список
    List,
}

#[derive(Clone, Debug, clap::ValueEnum)]
pub enum ExampleTemplate {
    /// Полный пример со всеми типами устройств
    Full,
    /// Минимальный пример
    Minimal,
    /// Только для Eltex устройств
    Eltex,
    /// Только для Cisco устройств
    Cisco,
    /// Только для Linux серверов
    Linux,
} 