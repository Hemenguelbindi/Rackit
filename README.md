# 🚀 Rackit

*Read this in other languages: [English](README_EN.md), [Русский](README.md)*

![Rust](https://img.shields.io/badge/rust-1.70+-orange.svg)
![License](https://img.shields.io/badge/license-MIT-blue.svg)
![Version](https://img.shields.io/badge/version-0.1.0-green.svg)

**Rackit** - это универсальный движок конфигурации для сетевого оборудования, написанный на Rust. Проект позволяет автоматизировать работу с различными типами устройств (роутеры, коммутаторы, серверы) через единый интерфейс, подобный Ansible или Terraform.

## ✨ Особенности

- 🔧 **Универсальность**: Поддержка различных типов устройств и протоколов
- 🗂️ **Конфигурация через TOML**: Простые и читаемые файлы конфигурации
- 🔌 **Множественные транспорты**: Serial, Telnet, SSH, HTTP/HTTPS
- 📋 **CLI как у профессиональных инструментов**: Команды в стиле Ansible/Terraform
- ⚡ **Параллельное выполнение**: Одновременная работа с несколькими устройствами
- 🛡️ **Обработка ошибок**: Гибкие стратегии при сбоях
- 📊 **Подробная отчетность**: Детальные логи выполнения

## 🎯 Поддерживаемые устройства

- **Eltex ESR-200** - роутеры через Serial-порт
- **Cisco** устройства (в разработке)
- **Linux серверы** через SSH (в разработке)
- Extensible архитектура для добавления новых устройств

## 📦 Установка

### Из исходного кода

```bash
git clone https://github.com/yourusername/rackit.git
cd rackit
cargo build --release
```

Бинарный файл будет доступен в `target/release/rackit`

### Системные требования

- Rust 1.70 или новее
- Linux/Unix система с доступом к последовательным портам
- Права доступа к устройствам `/dev/ttyS*` или `/dev/ttyUSB*`

## 🚀 Быстрый старт

### 1. Создание примера конфигурации

```bash
# Создать базовый пример
rackit example my-config.toml

# Создать пример для Eltex устройств
rackit example eltex-config.toml --template eltex

# Создать минимальный пример
rackit example minimal.toml --template minimal
```

### 2. Редактирование конфигурации

Отредактируйте созданный файл под ваши устройства:

```toml
[global_settings]
max_retries = 3
command_delay_ms = 200
default_timeout_seconds = 30

[devices.my_router]
[devices.my_router.device_info]
name = "Eltex ESR-200"
device_type = "router"
vendor = "Eltex"

[devices.my_router.connection]
transport = "serial"
host = "/dev/ttyS0"
baud_rate = 115200

[devices.my_router.credentials]
username = "admin"
password = "your_password"

[[devices.my_router.command_sequence]]
name = "login"
step_type = { type = "login" }
on_error = "stop"

[[devices.my_router.command_sequence]]
name = "show_system"
step_type = { type = "command", data = { command = "show system" } }
on_error = "continue"
```

### 3. Валидация и выполнение

```bash
# Проверить конфигурацию
rackit validate my-config.toml

# Посмотреть план выполнения
rackit plan my-config.toml --detailed

# Сухой запуск (без выполнения команд)
rackit run my-config.toml --dry-run

# Выполнить конфигурацию
rackit run my-config.toml
```

## 📚 Команды CLI

### Основные команды

| Команда | Описание | Аналог |
|---------|----------|--------|
| `rackit run` | Выполнить конфигурацию | `ansible-playbook` |
| `rackit plan` | Показать план выполнения | `terraform plan` |
| `rackit validate` | Валидировать конфигурацию | `terraform validate` |
| `rackit list` | Список устройств и сценариев | `ansible-inventory --list` |
| `rackit example` | Создать пример конфигурации | - |
| `rackit check` | Проверить состояние устройств | `ansible all -m ping` |
| `rackit shell` | Интерактивная работа | `ansible -m shell` |

### Опции

| Опция | Описание |
|-------|----------|
| `-v, --verbose` | Детальный вывод |
| `-q, --quiet` | Тихий режим |
| `--dry-run` | Сухой запуск |
| `--parallel` | Параллельное выполнение |
| `--ignore-errors` | Продолжить при ошибках |

## 📖 Примеры использования

### Работа с одним устройством

```bash
# Выполнить только для конкретного устройства
rackit run config.toml --device router1

# Детальный вывод
rackit --verbose run config.toml --device router1
```

### Параллельная работа

```bash
# Параллельное выполнение для всех устройств
rackit run config.toml --parallel --max-parallel 10

# Продолжить выполнение даже при ошибках
rackit run config.toml --parallel --ignore-errors
```

### Отладка

```bash
# Сухой запуск с детальным выводом
rackit --verbose run config.toml --dry-run

# Проверка доступности устройств
rackit check config.toml --timeout 30
```

## 🏗️ Архитектура

```
src/
├── cli/           # CLI интерфейс (clap)
├── config_engine/ # Движок конфигурации (TOML)
├── device/        # Драйверы устройств
├── transport/     # Транспортные протоколы
├── error/         # Обработка ошибок
└── lib.rs         # Публичный API
```

### Транспорты

- **Serial** - COM-порты и USB-to-Serial адаптеры
- **Telnet** - Традиционный telnet (в разработке)
- **SSH** - Безопасные соединения (в разработке)
- **HTTP/HTTPS** - REST API (в разработке)

### Типы устройств

- **Router** - Маршрутизаторы
- **Switch** - Коммутаторы
- **Server** - Серверы
- **Firewall** - Межсетевые экраны
- **Custom** - Пользовательские устройства

## 🔧 Разработка

### Сборка

```bash
cargo build
cargo test
cargo clippy
```

### Добавление поддержки нового устройства

1. Создайте новый модуль в `src/device/`
2. Реализуйте трейт `DeviceCommands`
3. Добавьте поддержку в конфигурационный движок
4. Создайте тесты

### Добавление нового транспорта

1. Создайте новый модуль в `src/transport/`
2. Реализуйте трейт `Transport`
3. Добавьте в конфигурационные типы
4. Обновите документацию

## 📝 Конфигурация

### Структура TOML файла

```toml
[global_settings]
max_retries = 3
command_delay_ms = 100
default_timeout_seconds = 30

[devices.device_id]
[devices.device_id.device_info]
name = "Device Name"
device_type = "router"  # router|switch|server|firewall|custom
vendor = "Vendor Name"
model = "Model"

[devices.device_id.connection]
transport = "serial"    # serial|telnet|ssh|http|https
host = "/dev/ttyS0"
baud_rate = 115200      # для serial
timeout_seconds = 10

[devices.device_id.credentials]
username = "admin"
password = "password"

[[devices.device_id.command_sequence]]
name = "step_name"
step_type = { type = "login" }
on_error = "stop"       # stop|continue|retry|goto_step

[[devices.device_id.command_sequence]]
name = "command_step"
step_type = { type = "command", data = { command = "show version" } }
on_error = "continue"
timeout_seconds = 5
```

### Типы шагов

- `login` - Вход в систему
- `logout` - Выход из системы
- `command` - Выполнение команды
- `delay` - Пауза
- `wait_prompt` - Ожидание приглашения
- `check_response` - Проверка ответа

## 🤝 Вклад в проект

1. Fork проекта
2. Создайте feature branch (`git checkout -b feature/amazing-feature`)
3. Commit изменения (`git commit -m 'Add amazing feature'`)
4. Push в branch (`git push origin feature/amazing-feature`)
5. Создайте Pull Request

## 📄 Лицензия

Этот проект лицензирован под MIT License - см. файл [LICENSE](LICENSE) для деталей.

## 🙏 Благодарности

- Команде Rust за отличный язык программирования
- Авторам библиотек `clap`, `serde`, `serialport`
- Сообществу за вдохновение и идеи

## 📞 Поддержка

- 🐛 [Issues](https://github.com/Hemenguelbindi/rackit/issues)
- 💡 [Discussions](https://github.com/Hemenguelbindi/rackit/discussions)
- 📧 Email: nilafe4@gmail.com

---

⭐ Если проект был полезен, поставьте звездочку на GitHub! 