use thiserror::Error;


#[derive(Error, Debug)]
pub enum Error {
    #[error("Ошибка соединения: {0}")]
    Connection(String),

    #[error("Ошибка ввода/вывода: {0}")]
    Io(#[from] std::io::Error),

    #[error("Ошибка Serial порта: {0}")]
    Serial(#[from] serialport::Error),

    #[error("Таймаут операции")]
    Timeout,
    
    #[error("Устройство не подключено")]
    NotConnected,

    #[error("Ошибка Парсинга конфигурации: {0}")]
    ConfigParse(String),

    #[error("Ошибка валидации конфигурации: {0}")]
    ConfigValidation(String),

    #[error("Ошибка чтения конфигурации: {0}")]
    ConfigRead(String),

    #[error("Ошибка Toml: {0}")]
    Toml(#[from] toml::de::Error),


    #[error("Неподерживаемый Формат конфигурации: {0}")]
    UnsupportedFormat(String),

    #[error("Конфигурационный файл не найден: {0}")]
    ConfigNotFound(String),

}

pub type Result<T> = std::result::Result<T, Error>;

