use std::path::Path;
use std::fs;
use crate::error::{Error, Result};
use crate::config_engine::{ConfigReader, types::ConfigFile};

pub struct TomlReader;

impl ConfigReader for TomlReader {
    fn read_from_file<P: AsRef<Path>>(path: P) -> Result<ConfigFile> {
        let path = path.as_ref();
        
        // Проверяем существование файла
        if !path.exists() {
            return Err(Error::ConfigNotFound(
                path.display().to_string()
            ));
        }
        
        // Читаем содержимое файла
        let content = fs::read_to_string(path)
            .map_err(|e| Error::Io(e))?;
        
        // Парсим TOML
        Self::read_from_str(&content)
    }
    
    fn read_from_str(content: &str) -> Result<ConfigFile> {
        let config: ConfigFile = toml::from_str(content)
            .map_err(|e| Error::ConfigParse(format!("TOML parse error: {}", e)))?;
        
        // Валидируем конфигурацию
        Self::validate(&config)?;
        
        Ok(config)
    }
    
    fn supported_extensions() -> &'static [&'static str] {
        &["toml"]
    }
}

impl TomlReader {
    /// Создает пример универсальной TOML конфигурации для разных устройств
    pub fn create_example_config() -> String {
        r##"# Универсальная конфигурация для управления различными устройствами

[global_settings]
max_retries = 3
command_delay_ms = 200
default_timeout_seconds = 30
log_level = "info"

# ================================
# УСТРОЙСТВА
# ================================

# Маршрутизатор Eltex ESR-200
[devices.eltex_router]
[devices.eltex_router.device_info]
name = "Eltex ESR-200 Main Router"
device_type = "router"
model = "ESR-200"
vendor = "Eltex"
description = "Основной маршрутизатор офиса"

[devices.eltex_router.connection]
transport = "serial"
host = "/dev/ttyS0"
baud_rate = 115200
timeout_seconds = 10

[devices.eltex_router.credentials]
username = "admin"
password = "password"
enable_password = "enable123"

[[devices.eltex_router.command_sequence]]
name = "login"
description = "Вход в систему"
step_type = { type = "login" }
timeout_seconds = 5
retry_count = 2
on_error = "stop"

[[devices.eltex_router.command_sequence]]
name = "get_system_info"
description = "Получить информацию о системе"
step_type = { type = "command", data = { command = "show system", expected_prompt = "esr-200#" } }
timeout_seconds = 10
retry_count = 1
on_error = "continue"

[[devices.eltex_router.command_sequence]]
name = "get_version"
description = "Получить версию ПО"
step_type = { type = "command", data = { command = "show version", expected_prompt = "esr-200#" } }
timeout_seconds = 10
retry_count = 1
on_error = "continue"

[[devices.eltex_router.command_sequence]]
name = "get_interfaces"
description = "Показать интерфейсы"
step_type = { type = "command", data = { command = "show interfaces", expected_prompt = "esr-200#" } }
timeout_seconds = 15
retry_count = 1
on_error = "continue"

[[devices.eltex_router.command_sequence]]
name = "logout"
description = "Выход из системы"
step_type = { type = "logout" }
timeout_seconds = 5
retry_count = 1
on_error = "continue"

[devices.eltex_router.device_settings]
init_commands = ["terminal length 0"]

[devices.eltex_router.device_settings.prompts]
login = "login:"
password = "Password:"
main = "esr-200#"
enable = "esr-200>"

# ================================
# Коммутатор Cisco
[devices.cisco_switch]
[devices.cisco_switch.device_info]
name = "Cisco Catalyst 2960"
device_type = "switch"
model = "WS-C2960-24TT-L"
vendor = "Cisco"
description = "Коммутатор доступа"

[devices.cisco_switch.connection]
transport = "telnet"
host = "192.168.1.100"
port = 23
timeout_seconds = 15

[devices.cisco_switch.credentials]
username = "admin"
password = "cisco123"
enable_password = "enable456"

[[devices.cisco_switch.command_sequence]]
name = "login"
step_type = { type = "login" }
timeout_seconds = 10
retry_count = 3
on_error = "stop"

[[devices.cisco_switch.command_sequence]]
name = "enable_mode"
step_type = { type = "command", data = { command = "enable", expected_prompt = "#" } }
timeout_seconds = 5
retry_count = 2
on_error = "stop"

[[devices.cisco_switch.command_sequence]]
name = "show_version"
step_type = { type = "command", data = { command = "show version", expected_prompt = "#" } }
timeout_seconds = 10
retry_count = 1
on_error = "continue"

[[devices.cisco_switch.command_sequence]]
name = "show_vlans"
step_type = { type = "command", data = { command = "show vlan brief", expected_prompt = "#" } }
timeout_seconds = 10
retry_count = 1
on_error = "continue"

[[devices.cisco_switch.command_sequence]]
name = "logout"
step_type = { type = "logout" }
timeout_seconds = 5
on_error = "continue"

[devices.cisco_switch.device_settings]
init_commands = ["terminal length 0", "terminal width 0"]

[devices.cisco_switch.device_settings.prompts]
username = "Username:"
password = "Password:"
main = "#"
user_mode = ">"

# ================================
# Linux сервер
[devices.linux_server]
[devices.linux_server.device_info]
name = "Ubuntu Server"
device_type = "server"
model = "VM"
vendor = "Ubuntu"
description = "Веб-сервер на Ubuntu"

[devices.linux_server.connection]
transport = "ssh"
host = "192.168.1.50"
port = 22
timeout_seconds = 20

[devices.linux_server.credentials]
username = "admin"
password = "serverpass"
ssh_key_path = "/home/user/.ssh/id_rsa"

[[devices.linux_server.command_sequence]]
name = "login"
step_type = { type = "login" }
timeout_seconds = 10
retry_count = 3
on_error = "stop"

[[devices.linux_server.command_sequence]]
name = "check_system"
step_type = { type = "command", data = { command = "uname -a", expected_prompt = "$" } }
timeout_seconds = 5
retry_count = 1
on_error = "continue"

[[devices.linux_server.command_sequence]]
name = "check_disk_space"
step_type = { type = "command", data = { command = "df -h", expected_prompt = "$" } }
timeout_seconds = 10
retry_count = 1
on_error = "continue"

[[devices.linux_server.command_sequence]]
name = "check_memory"
step_type = { type = "command", data = { command = "free -m", expected_prompt = "$" } }
timeout_seconds = 5
retry_count = 1
on_error = "continue"

[[devices.linux_server.command_sequence]]
name = "check_services"
step_type = { type = "command", data = { command = "systemctl status apache2", expected_prompt = "$" } }
timeout_seconds = 10
retry_count = 1
on_error = "continue"

[devices.linux_server.device_settings]
[devices.linux_server.device_settings.prompts]
main = "$"
root = "#"

# ================================
# СЦЕНАРИИ
# ================================

[scenarios.daily_check]
name = "Ежедневная проверка"
description = "Проверка состояния всех устройств"
execution_mode = "sequential"

[scenarios.daily_check.target_devices]
type = "all"

[[scenarios.daily_check.commands]]
name = "health_check"
step_type = { type = "command", data = { command = "show status", expected_prompt = "#" } }
timeout_seconds = 30
on_error = "continue"

[scenarios.router_only]
name = "Проверка только маршрутизаторов"
description = "Команды только для роутеров"
execution_mode = "parallel"

[scenarios.router_only.target_devices]
type = "by_type"
device_type = "router"

[[scenarios.router_only.commands]]
name = "check_routing_table"
step_type = { type = "command", data = { command = "show ip route", expected_prompt = "#" } }
timeout_seconds = 15
on_error = "continue"
"##.to_string()
    }
    
    /// Сохраняет конфигурацию в TOML файл
    pub fn save_to_file<P: AsRef<Path>>(config: &ConfigFile, path: P) -> Result<()> {
        let toml_string = toml::to_string_pretty(config)
            .map_err(|e| Error::ConfigParse(format!("Failed to serialize TOML: {}", e)))?;
        
        fs::write(path, toml_string)
            .map_err(|e| Error::Io(e))?;
        
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::NamedTempFile;
    use std::io::Write;

    #[test]
    fn test_read_toml_config() {
        let toml_content = TomlReader::create_example_config();
        let config = TomlReader::read_from_str(&toml_content).unwrap();
        
        // Проверяем что есть все устройства
        assert!(config.devices.contains_key("eltex_router"));
        assert!(config.devices.contains_key("cisco_switch"));
        assert!(config.devices.contains_key("linux_server"));
        
        // Проверяем Eltex устройство
        let eltex = &config.devices["eltex_router"];
        assert_eq!(eltex.device_info.name, "Eltex ESR-200 Main Router");
        assert_eq!(eltex.connection.host, "/dev/ttyS0");
        assert_eq!(eltex.credentials.username, "admin");
        assert!(!eltex.command_sequence.is_empty());
        
        // Проверяем сценарии
        assert!(config.scenarios.contains_key("daily_check"));
        assert!(config.scenarios.contains_key("router_only"));
    }
    
    #[test]
    fn test_read_from_file() {
        let mut temp_file = NamedTempFile::new().unwrap();
        let toml_content = TomlReader::create_example_config();
        temp_file.write_all(toml_content.as_bytes()).unwrap();
        
        let config = TomlReader::read_from_file(temp_file.path()).unwrap();
        assert_eq!(config.devices.len(), 3);
    }
}