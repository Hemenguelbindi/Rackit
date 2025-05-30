use crate::cli::args::{Commands, OutputFormat, ExampleTemplate};
use crate::config_engine::{load_config, create_executor, create_example_config};
use crate::config_engine::types::StepType;
use crate::error::Result;

/// Выполняет CLI команду
pub fn execute_command(command: Commands, verbose: u8, quiet: bool) -> Result<()> {
    match command {
        Commands::Run { 
            config, device, scenario, dry_run, parallel, max_parallel, ignore_errors 
        } => {
            cmd_run(config, device, scenario, dry_run, parallel, max_parallel, ignore_errors, verbose, quiet)
        }
        Commands::Validate { config, strict } => {
            cmd_validate(config, strict, verbose, quiet)
        }
        Commands::Plan { config, device, scenario, detailed } => {
            cmd_plan(config, device, scenario, detailed, verbose, quiet)
        }
        Commands::List { config, devices, scenarios, format } => {
            cmd_list(config, devices, scenarios, format, verbose, quiet)
        }
        Commands::Example { output, template, force } => {
            cmd_example(output, template, force, verbose, quiet)
        }
        Commands::Check { config, device, ping_only, timeout } => {
            cmd_check(config, device, ping_only, timeout, verbose, quiet)
        }
        Commands::Shell { config, device, command } => {
            cmd_shell(config, device, command, verbose, quiet)
        }
    }
}

/// rackit run config.toml
fn cmd_run(
    config_path: std::path::PathBuf,
    device_filter: Option<String>,
    scenario_filter: Option<String>,
    dry_run: bool,
    parallel: bool,
    _max_parallel: usize,
    ignore_errors: bool,
    verbose: u8,
    quiet: bool,
) -> Result<()> {
    if !quiet {
        println!("🚀 Rackit - выполнение конфигурации");
        if dry_run {
            println!("🔍 Режим: сухой запуск (dry-run)");
        }
        if parallel {
            println!("⚡ Режим: параллельное выполнение");
        }
        println!();
    }

    // Загружаем конфигурацию
    let executor = create_executor(&config_path)?;
    let config = executor.config();

    if verbose > 0 {
        println!("📖 Загружена конфигурация из: {}", config_path.display());
        println!("   Устройств: {}", config.devices.len());
        println!("   Сценариев: {}", config.scenarios.len());
        println!();
    }

    if dry_run {
        println!("📋 План выполнения (dry-run):");
        return cmd_plan(config_path, device_filter, scenario_filter, true, verbose, quiet);
    }

    // Фильтруем устройства
    let devices_to_run: Vec<&String> = if let Some(device_id) = &device_filter {
        if config.devices.contains_key(device_id) {
            vec![device_id]
        } else {
            eprintln!("❌ Устройство '{}' не найдено в конфигурации", device_id);
            return Ok(());
        }
    } else {
        config.devices.keys().collect()
    };

    if !quiet {
        println!("🎯 Выполнение для {} устройств:", devices_to_run.len());
        for device_id in &devices_to_run {
            let device_config = &config.devices[*device_id];
            println!("  📱 {} - {} ({})", 
                device_id, 
                device_config.device_info.name,
                device_config.device_info.vendor
            );
        }
        println!();
    }

    // Выполняем команды
    let mut success_count = 0;
    let mut error_count = 0;

    for device_id in devices_to_run {
        if verbose > 0 {
            println!("🔧 Обработка устройства: {}", device_id);
        }

        match executor.execute_device(device_id) {
            Ok(result) => {
                if result.is_success() {
                    success_count += 1;
                    if !quiet {
                        println!("✅ {} - выполнено успешно ({} команд)", 
                            device_id, result.success_count());
                    }
                } else {
                    error_count += 1;
                    println!("⚠️ {} - выполнено с ошибками ({} успешных, {} ошибок)", 
                        device_id, result.success_count(), result.error_count());
                    
                    if verbose > 0 {
                        for error in &result.failed_commands {
                            println!("   ❌ {}: {}", error.command_name, error.error_message);
                        }
                    }
                }
            }
            Err(e) => {
                error_count += 1;
                println!("❌ {} - ошибка подключения: {}", device_id, e);
                
                if !ignore_errors {
                    println!("💡 Используйте --ignore-errors для продолжения при ошибках");
                    break;
                }
            }
        }
    }

    if !quiet {
        println!();
        println!("📊 Итоги выполнения:");
        println!("   ✅ Успешно: {}", success_count);
        println!("   ❌ С ошибками: {}", error_count);
        println!("   📈 Общий результат: {}", 
            if error_count == 0 { "✅ Успех" } else { "⚠️ Есть ошибки" }
        );
    }

    Ok(())
}

/// rackit validate config.toml
fn cmd_validate(
    config_path: std::path::PathBuf,
    strict: bool,
    verbose: u8,
    quiet: bool,
) -> Result<()> {
    if !quiet {
        println!("🔍 Rackit - валидация конфигурации");
        if strict {
            println!("🔒 Режим: строгая валидация");
        }
        println!();
    }

    // Проверяем существование файла
    if !config_path.exists() {
        eprintln!("❌ Файл конфигурации не найден: {}", config_path.display());
        return Ok(());
    }

    // Загружаем и валидируем конфигурацию
    match load_config(&config_path) {
        Ok(config) => {
            if !quiet {
                println!("✅ Синтаксис конфигурации корректен");
                println!("   📱 Устройств: {}", config.devices.len());
                println!("   🎭 Сценариев: {}", config.scenarios.len());
            }

            if verbose > 0 {
                println!("\n📋 Детали конфигурации:");
                for (device_id, device_config) in &config.devices {
                    println!("  📱 {}: {} команд", device_id, device_config.command_sequence.len());
                }
            }

            if strict {
                println!("\n🔒 Строгая валидация (проверка доступности устройств):");
                println!("⚠️ Строгая валидация пока не реализована");
            }

            if !quiet {
                println!("\n🎉 Конфигурация валидна!");
            }
        }
        Err(e) => {
            eprintln!("❌ Ошибка валидации: {}", e);
        }
    }

    Ok(())
}

/// rackit plan config.toml
fn cmd_plan(
    config_path: std::path::PathBuf,
    device_filter: Option<String>,
    _scenario_filter: Option<String>,
    detailed: bool,
    verbose: u8,
    quiet: bool,
) -> Result<()> {
    if !quiet {
        println!("📋 Rackit - план выполнения");
        println!();
    }

    let config = load_config(&config_path)?;

    // Фильтруем устройства
    let devices_to_show: Vec<&String> = if let Some(device_id) = &device_filter {
        if config.devices.contains_key(device_id) {
            vec![device_id]
        } else {
            eprintln!("❌ Устройство '{}' не найдено", device_id);
            return Ok(());
        }
    } else {
        config.devices.keys().collect()
    };

    println!("🎯 План выполнения для {} устройств:", devices_to_show.len());
    println!();

    for device_id in devices_to_show {
        let device_config = &config.devices[device_id];
        
        println!("📱 Устройство: {} ({})", device_id, device_config.device_info.name);
        println!("   🏭 Производитель: {}", device_config.device_info.vendor);
        println!("   🔗 Транспорт: {:?} -> {}", 
            device_config.connection.transport, 
            device_config.connection.host
        );
        println!("   📝 Команд к выполнению: {}", device_config.command_sequence.len());

        if detailed || verbose > 0 {
            println!("   📋 Последовательность команд:");
            for (i, step) in device_config.command_sequence.iter().enumerate() {
                let step_desc = match &step.step_type {
                    StepType::Login => "🔐 Вход в систему".to_string(),
                    StepType::Logout => "🚪 Выход из системы".to_string(),
                    StepType::Command { command, .. } => format!("💻 Команда: '{}'", command),
                    StepType::Delay { milliseconds } => format!("⏱️ Пауза: {} мс", milliseconds),
                    StepType::WaitPrompt { prompt } => format!("⏳ Ожидание: '{}'", prompt),
                    StepType::CheckResponse { contains, .. } => format!("✅ Проверка: '{}'", contains),
                };
                println!("      {}. {} - {}", i + 1, step.name, step_desc);
            }
        }
        println!();
    }

    if !quiet {
        println!("💡 Используйте 'rackit run' для выполнения плана");
    }

    Ok(())
}

/// rackit list config.toml
fn cmd_list(
    config_path: std::path::PathBuf,
    devices_only: bool,
    scenarios_only: bool,
    format: OutputFormat,
    _verbose: u8,
    quiet: bool,
) -> Result<()> {
    let config = load_config(&config_path)?;

    match format {
        OutputFormat::Table => {
            if !scenarios_only {
                if !quiet { println!("📱 Устройства:"); }
                println!("┌─────────────────┬─────────────────────────┬─────────────┬─────────────┬─────────────┐");
                println!("│ ID              │ Название                │ Тип         │ Производ.   │ Команд      │");
                println!("├─────────────────┼─────────────────────────┼─────────────┼─────────────┼─────────────┤");
                
                for (device_id, device_config) in &config.devices {
                    println!("│ {:<15} │ {:<23} │ {:<11} │ {:<11} │ {:<11} │",
                        truncate(device_id, 15),
                        truncate(&device_config.device_info.name, 23),
                        format!("{:?}", device_config.device_info.device_type),
                        truncate(&device_config.device_info.vendor, 11),
                        device_config.command_sequence.len()
                    );
                }
                println!("└─────────────────┴─────────────────────────┴─────────────┴─────────────┴─────────────┘");
                println!();
            }

            if !devices_only && !config.scenarios.is_empty() {
                if !quiet { println!("🎭 Сценарии:"); }
                println!("┌─────────────────┬─────────────────────────┬─────────────────────────┬─────────────┐");
                println!("│ ID              │ Название                │ Целевые устройства      │ Команд      │");
                println!("├─────────────────┼─────────────────────────┼─────────────────────────┼─────────────┤");
                
                for (scenario_id, scenario) in &config.scenarios {
                    let target_desc = match &scenario.target_devices {
                        crate::config_engine::types::TargetDevices::All => "Все".to_string(),
                        crate::config_engine::types::TargetDevices::Specific { devices } => {
                            format!("{} устройств", devices.len())
                        },
                        crate::config_engine::types::TargetDevices::ByType { device_type } => {
                            format!("Тип: {:?}", device_type)
                        },
                        crate::config_engine::types::TargetDevices::ByVendor { vendor } => {
                            format!("Произв.: {}", vendor)
                        },
                    };
                    
                    println!("│ {:<15} │ {:<23} │ {:<23} │ {:<11} │",
                        truncate(scenario_id, 15),
                        truncate(&scenario.name, 23),
                        truncate(&target_desc, 23),
                        scenario.commands.len()
                    );
                }
                println!("└─────────────────┴─────────────────────────┴─────────────────────────┴─────────────┘");
            }
        }
        OutputFormat::List => {
            if !scenarios_only {
                for device_id in config.devices.keys() {
                    println!("{}", device_id);
                }
            }
            if !devices_only {
                for scenario_id in config.scenarios.keys() {
                    println!("{}", scenario_id);
                }
            }
        }
        OutputFormat::Json => {
            println!("JSON формат пока не реализован");
        }
        OutputFormat::Yaml => {
            println!("YAML формат пока не реализован");
        }
    }

    Ok(())
}

/// rackit example output.toml
fn cmd_example(
    output_path: std::path::PathBuf,
    template: ExampleTemplate,
    force: bool,
    _verbose: u8,
    quiet: bool,
) -> Result<()> {
    if !quiet {
        println!("📝 Создание примера конфигурации");
        println!("   Шаблон: {:?}", template);
        println!("   Файл: {}", output_path.display());
        println!();
    }

    // Проверяем существование файла
    if output_path.exists() && !force {
        eprintln!("❌ Файл уже существует: {}", output_path.display());
        eprintln!("💡 Используйте --force для перезаписи");
        return Ok(());
    }

    // Создаем пример в зависимости от шаблона
    match template {
        ExampleTemplate::Full => {
            create_example_config(&output_path)?;
        }
        ExampleTemplate::Minimal => {
            let minimal_config = create_minimal_example();
            std::fs::write(&output_path, minimal_config)?;
        }
        ExampleTemplate::Eltex => {
            let eltex_config = create_eltex_example();
            std::fs::write(&output_path, eltex_config)?;
        }
        ExampleTemplate::Cisco => {
            println!("⚠️ Cisco шаблон пока не реализован, создаем полный пример");
            create_example_config(&output_path)?;
        }
        ExampleTemplate::Linux => {
            println!("⚠️ Linux шаблон пока не реализован, создаем полный пример");
            create_example_config(&output_path)?;
        }
    }

    if !quiet {
        println!("✅ Пример конфигурации создан: {}", output_path.display());
        println!("💡 Отредактируйте файл под ваши устройства и запустите:");
        println!("   rackit validate {}", output_path.display());
        println!("   rackit plan {}", output_path.display());
        println!("   rackit run {}", output_path.display());
    }

    Ok(())
}

/// rackit check config.toml
fn cmd_check(
    config_path: std::path::PathBuf,
    device_filter: Option<String>,
    ping_only: bool,
    _timeout: u64,
    _verbose: u8,
    quiet: bool,
) -> Result<()> {
    if !quiet {
        println!("🔍 Rackit - проверка состояния устройств");
        if ping_only {
            println!("📡 Режим: только проверка подключения");
        }
        println!();
    }

    let config = load_config(&config_path)?;

    // Фильтруем устройства
    let devices_to_check: Vec<&String> = if let Some(device_id) = &device_filter {
        if config.devices.contains_key(device_id) {
            vec![device_id]
        } else {
            eprintln!("❌ Устройство '{}' не найдено", device_id);
            return Ok(());
        }
    } else {
        config.devices.keys().collect()
    };

    println!("🎯 Проверка {} устройств:", devices_to_check.len());
    println!();

    for device_id in devices_to_check {
        let device_config = &config.devices[device_id];
        print!("📱 {} ({}) ... ", device_id, device_config.connection.host);
        
        // Пока что просто показываем статус "не реализовано"
        println!("⚠️ Проверка не реализована");
    }

    Ok(())
}

/// rackit shell config.toml --device router1
fn cmd_shell(
    _config_path: std::path::PathBuf,
    device_id: String,
    command: Option<String>,
    _verbose: u8,
    quiet: bool,
) -> Result<()> {
    if !quiet {
        println!("🖥️ Rackit - интерактивная оболочка");
        println!("   Устройство: {}", device_id);
        if let Some(cmd) = &command {
            println!("   Команда: {}", cmd);
        } else {
            println!("   Режим: интерактивный");
        }
        println!();
    }

    println!("⚠️ Интерактивная оболочка пока не реализована");
    
    Ok(())
}

// Вспомогательные функции

fn truncate(s: &str, max_len: usize) -> String {
    if s.len() <= max_len {
        s.to_string()
    } else {
        format!("{}...", &s[..max_len.saturating_sub(3)])
    }
}

fn create_minimal_example() -> String {
    r#"# Минимальный пример конфигурации Rackit

[global_settings]
max_retries = 3
command_delay_ms = 100
default_timeout_seconds = 30

[devices.my_device]
[devices.my_device.device_info]
name = "My Device"
device_type = "router"
model = "Unknown"
vendor = "Unknown"

[devices.my_device.connection]
transport = "serial"
host = "/dev/ttyS0"
baud_rate = 115200

[devices.my_device.credentials]
username = "admin"
password = "password"

[[devices.my_device.command_sequence]]
name = "login"
step_type = { type = "login" }
on_error = "stop"

[[devices.my_device.command_sequence]]
name = "show_info"
step_type = { type = "command", data = { command = "show version" } }
on_error = "continue"
"#.to_string()
}

fn create_eltex_example() -> String {
    r#"# Пример конфигурации для Eltex устройств

[global_settings]
max_retries = 3
command_delay_ms = 200
default_timeout_seconds = 30

[devices.eltex_router]
[devices.eltex_router.device_info]
name = "Eltex ESR-200"
device_type = "router"
model = "ESR-200"
vendor = "Eltex"

[devices.eltex_router.connection]
transport = "serial"
host = "/dev/ttyS0"
baud_rate = 115200
timeout_seconds = 10

[devices.eltex_router.credentials]
username = "admin"
password = "password"

[[devices.eltex_router.command_sequence]]
name = "login"
step_type = { type = "login" }
on_error = "stop"

[[devices.eltex_router.command_sequence]]
name = "show_system"
step_type = { type = "command", data = { command = "show system", expected_prompt = "esr-200#" } }
on_error = "continue"

[[devices.eltex_router.command_sequence]]
name = "show_version"
step_type = { type = "command", data = { command = "show version", expected_prompt = "esr-200#" } }
on_error = "continue"

[[devices.eltex_router.command_sequence]]
name = "logout"
step_type = { type = "logout" }
on_error = "continue"
"#.to_string()
} 