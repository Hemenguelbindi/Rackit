use rackit::{load_config, create_example_config, Result};
use rackit::config_engine::types::{StepType, ErrorAction};

fn main() -> Result<()> {
    println!("🚀 Демонстрация универсального движка конфигурации");
    
    // Создаем пример конфигурации
    println!("\n📝 Создаем пример конфигурации...");
    create_example_config("universal_config.toml")?;
    println!("✅ Файл 'universal_config.toml' создан");
    
    // Загружаем конфигурацию
    println!("\n📖 Загружаем конфигурацию...");
    let config = load_config("universal_config.toml")?;
    
    println!("✅ Конфигурация загружена успешно!");
    println!("   📊 Глобальные настройки:");
    println!("      - Максимум повторов: {}", config.global_settings.max_retries);
    println!("      - Задержка между командами: {} мс", config.global_settings.command_delay_ms);
    println!("      - Таймаут по умолчанию: {} сек", config.global_settings.default_timeout_seconds);
    
    // Показываем информацию об устройствах
    println!("\n🖥️  Обнаружено устройств: {}", config.devices.len());
    for (device_id, device_config) in &config.devices {
        println!("\n  📱 Устройство: {}", device_id);
        println!("     Название: {}", device_config.device_info.name);
        println!("     Тип: {:?}", device_config.device_info.device_type);
        println!("     Производитель: {}", device_config.device_info.vendor);
        println!("     Модель: {}", device_config.device_info.model);
        println!("     Транспорт: {:?}", device_config.connection.transport);
        println!("     Хост: {}", device_config.connection.host);
        println!("     Команд в последовательности: {}", device_config.command_sequence.len());
        
        // Показываем команды
        for (i, command) in device_config.command_sequence.iter().enumerate() {
            match &command.step_type {
                StepType::Command { command: cmd, expected_prompt } => {
                    println!("       {}. {} -> команда: '{}'", i+1, command.name, cmd);
                    if let Some(prompt) = expected_prompt {
                        println!("          ожидаемый промпт: '{}'", prompt);
                    }
                }
                StepType::Login => {
                    println!("       {}. {} -> логин", i+1, command.name);
                }
                StepType::Logout => {
                    println!("       {}. {} -> выход", i+1, command.name);
                }
                StepType::Delay { milliseconds } => {
                    println!("       {}. {} -> пауза {} мс", i+1, command.name, milliseconds);
                }
                StepType::WaitPrompt { prompt } => {
                    println!("       {}. {} -> ожидание промпта '{}'", i+1, command.name, prompt);
                }
                StepType::CheckResponse { contains, fail_if_not_found } => {
                    println!("       {}. {} -> проверка наличия '{}' (критично: {})", 
                        i+1, command.name, contains, fail_if_not_found);
                }
            }
        }
    }
    
    // Показываем сценарии
    if !config.scenarios.is_empty() {
        println!("\n🎭 Сценарии выполнения: {}", config.scenarios.len());
        for (scenario_name, scenario) in &config.scenarios {
            println!("\n  🎯 Сценарий: {}", scenario_name);
            println!("     Название: {}", scenario.name);
            if let Some(desc) = &scenario.description {
                println!("     Описание: {}", desc);
            }
            println!("     Режим выполнения: {:?}", scenario.execution_mode);
            println!("     Целевые устройства: {:?}", scenario.target_devices);
            println!("     Дополнительных команд: {}", scenario.commands.len());
        }
    }
    
    println!("\n🎉 Демонстрация завершена!");
    println!("\n💡 Теперь вы можете:");
    println!("   • Редактировать universal_config.toml для своих устройств");
    println!("   • Добавлять новые устройства разных типов (роутеры, коммутаторы, серверы)");
    println!("   • Создавать сценарии для групп устройств");
    println!("   • Выполнять команды из конфигурации программно");
    
    Ok(())
} 