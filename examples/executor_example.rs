use rackit::{create_executor, create_example_config, Result};

fn main() -> Result<()> {
    println!("🚀 Демонстрация исполнительного движка конфигурации");
    
    // Создаем пример конфигурации (только для демонстрации)
    println!("\n📝 Создаем пример конфигурации...");
    create_example_config("demo_config.toml")?;
    println!("✅ Файл 'demo_config.toml' создан");
    
    // Создаем движок выполнения
    println!("\n🔧 Создаем движок выполнения...");
    let executor = create_executor("demo_config.toml")?;
    println!("✅ Движок создан и конфигурация загружена");
    
    // Показываем доступные устройства
    println!("\n📋 Доступные устройства:");
    for (device_id, device_config) in &executor.config().devices {
        println!("  📱 {} - {} ({})", 
            device_id, 
            device_config.device_info.name,
            device_config.device_info.vendor
        );
    }
    
    // Выполняем команды только для Eltex устройства (остальные пока не реализованы)
    println!("\n🎯 Выполнение команд для Eltex устройства...");
    
    // Находим Eltex устройство
    let eltex_device_id = executor.config().devices
        .iter()
        .find(|(_, config)| config.device_info.vendor == "Eltex")
        .map(|(id, _)| id.as_str());
    
    match eltex_device_id {
        Some(device_id) => {
            println!("🔍 Найдено Eltex устройство: {}", device_id);
            
            // Пытаемся выполнить команды
            match executor.execute_device(device_id) {
                Ok(result) => {
                    println!("\n✅ Выполнение завершено!");
                    println!("   📊 Статистика:");
                    println!("      - Успешных команд: {}", result.success_count());
                    println!("      - Неудачных команд: {}", result.error_count());
                    println!("      - Общий статус: {}", 
                        if result.is_success() { "✅ Успех" } else { "❌ Есть ошибки" }
                    );
                    
                    // Показываем результаты успешных команд
                    if !result.successful_commands.is_empty() {
                        println!("\n   🎉 Успешные команды:");
                        for cmd_result in &result.successful_commands {
                            println!("      • {}: {}", 
                                cmd_result.command_name,
                                cmd_result.output.lines().next().unwrap_or("").chars().take(50).collect::<String>()
                            );
                        }
                    }
                    
                    // Показываем ошибки
                    if !result.failed_commands.is_empty() {
                        println!("\n   ❌ Ошибки:");
                        for cmd_error in &result.failed_commands {
                            println!("      • {}: {}", cmd_error.command_name, cmd_error.error_message);
                        }
                    }
                }
                Err(e) => {
                    println!("❌ Ошибка выполнения: {}", e);
                    println!("💡 Возможные причины:");
                    println!("   • Устройство не подключено к /dev/ttyS0");
                    println!("   • Неправильные настройки порта");
                    println!("   • Устройство не отвечает");
                    println!("   • Недостаточно прав доступа к порту");
                }
            }
        }
        None => {
            println!("⚠️ Eltex устройство не найдено в конфигурации");
            
            // Показываем что можно попробовать выполнить для всех устройств
            println!("\n🔄 Попытка выполнения для всех устройств (многие завершатся ошибкой)...");
            
            match executor.execute_all_devices() {
                Ok(results) => {
                    println!("📊 Результаты для {} устройств:", results.len());
                    for result in results {
                        println!("  {} - {} ({})", 
                            if result.is_success() { "✅" } else { "❌" },
                            result.device_id,
                            if result.is_success() { 
                                format!("{} команд", result.success_count()) 
                            } else { 
                                result.failed_commands.first()
                                    .map(|e| e.error_message.clone())
                                    .unwrap_or_else(|| "Неизвестная ошибка".to_string())
                            }
                        );
                    }
                }
                Err(e) => {
                    println!("❌ Ошибка массового выполнения: {}", e);
                }
            }
        }
    }
    
    println!("\n🎉 Демонстрация завершена!");
    println!("\n💡 Что продемонстрировано:");
    println!("   ✅ Чтение конфигурации из TOML файла");
    println!("   ✅ Создание движка выполнения");
    println!("   ✅ Применение настроек из конфигурации");
    println!("   ✅ Выполнение команд согласно последовательности");
    println!("   ✅ Обработка ошибок согласно настройкам");
    println!("   ✅ Использование таймаутов и пауз из конфигурации");
    
    Ok(())
} 