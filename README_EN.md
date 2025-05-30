# 🚀 Rackit

*Read this in other languages: [English](README_EN.md), [Русский](README.md)*

![Rust](https://img.shields.io/badge/rust-1.70+-orange.svg)
![License](https://img.shields.io/badge/license-MIT-blue.svg)
![Version](https://img.shields.io/badge/version-0.1.0-green.svg)

**Rackit** is a universal network device configuration engine written in Rust. The project allows automation of work with various types of devices (routers, switches, servers) through a unified interface similar to Ansible or Terraform.

## ✨ Features

- 🔧 **Universality**: Support for various device types and protocols
- 🗂️ **TOML Configuration**: Simple and readable configuration files
- 🔌 **Multiple Transports**: Serial, Telnet, SSH, HTTP/HTTPS
- 📋 **Professional CLI**: Commands in Ansible/Terraform style
- ⚡ **Parallel Execution**: Simultaneous work with multiple devices
- 🛡️ **Error Handling**: Flexible strategies for failures
- 📊 **Detailed Reporting**: Comprehensive execution logs

## 🎯 Supported Devices

- **Eltex ESR-200** - routers via Serial port
- **Cisco** devices (in development)
- **Linux servers** via SSH (in development)
- Extensible architecture for adding new devices

## 📦 Installation

### From Source Code

```bash
git clone https://github.com/yourusername/rackit.git
cd rackit
cargo build --release
```

Binary file will be available at `target/release/rackit`

### System Requirements

- Rust 1.70 or newer
- Linux/Unix system with access to serial ports
- Access permissions to `/dev/ttyS*` or `/dev/ttyUSB*` devices

## 🚀 Quick Start

### 1. Creating Example Configuration

```bash
# Create basic example
rackit example my-config.toml

# Create example for Eltex devices
rackit example eltex-config.toml --template eltex

# Create minimal example
rackit example minimal.toml --template minimal
```

### 2. Editing Configuration

Edit the created file for your devices:

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

### 3. Validation and Execution

```bash
# Check configuration
rackit validate my-config.toml

# View execution plan
rackit plan my-config.toml --detailed

# Dry run (without executing commands)
rackit run my-config.toml --dry-run

# Execute configuration
rackit run my-config.toml
```

## 📚 CLI Commands

### Main Commands

| Command | Description | Analog |
|---------|-------------|--------|
| `rackit run` | Execute configuration | `ansible-playbook` |
| `rackit plan` | Show execution plan | `terraform plan` |
| `rackit validate` | Validate configuration | `terraform validate` |
| `rackit list` | List devices and scenarios | `ansible-inventory --list` |
| `rackit example` | Create example configuration | - |
| `rackit check` | Check device status | `ansible all -m ping` |
| `rackit shell` | Interactive device access | `ansible -m shell` |

### Options

| Option | Description |
|--------|-------------|
| `-v, --verbose` | Detailed output |
| `-q, --quiet` | Quiet mode |
| `--dry-run` | Dry run |
| `--parallel` | Parallel execution |
| `--ignore-errors` | Continue on errors |

## 📖 Usage Examples

### Working with Single Device

```bash
# Execute only for specific device
rackit run config.toml --device router1

# Detailed output
rackit --verbose run config.toml --device router1
```

### Parallel Execution

```bash
# Parallel execution for all devices
rackit run config.toml --parallel --max-parallel 10

# Continue execution even on errors
rackit run config.toml --parallel --ignore-errors
```

### Debugging

```bash
# Dry run with detailed output
rackit --verbose run config.toml --dry-run

# Check device availability
rackit check config.toml --timeout 30
```

## 🏗️ Architecture

```
src/
├── cli/           # CLI interface (clap)
├── config_engine/ # Configuration engine (TOML)
├── device/        # Device drivers
├── transport/     # Transport protocols
├── error/         # Error handling
└── lib.rs         # Public API
```

### Transports

- **Serial** - COM ports and USB-to-Serial adapters
- **Telnet** - Traditional telnet (in development)
- **SSH** - Secure connections (in development)
- **HTTP/HTTPS** - REST API (in development)

### Device Types

- **Router** - Routers
- **Switch** - Switches
- **Server** - Servers
- **Firewall** - Firewalls
- **Custom** - Custom devices

## 🔧 Development

### Building

```bash
cargo build
cargo test
cargo clippy
```

### Adding Support for New Device

1. Create new module in `src/device/`
2. Implement `DeviceCommands` trait
3. Add support in configuration engine
4. Create tests

### Adding New Transport

1. Create new module in `src/transport/`
2. Implement `Transport` trait
3. Add to configuration types
4. Update documentation

## 📝 Configuration

### TOML File Structure

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
baud_rate = 115200      # for serial
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

### Step Types

- `login` - System login
- `logout` - System logout
- `command` - Command execution
- `delay` - Pause
- `wait_prompt` - Wait for prompt
- `check_response` - Response validation

## 🤝 Contributing

1. Fork the project
2. Create feature branch (`git checkout -b feature/amazing-feature`)
3. Commit changes (`git commit -m 'Add amazing feature'`)
4. Push to branch (`git push origin feature/amazing-feature`)
5. Create Pull Request

## 📄 License

This project is licensed under MIT License - see [LICENSE](LICENSE) file for details.

## 🙏 Acknowledgments

- Rust team for excellent programming language
- Authors of `clap`, `serde`, `serialport` libraries
- Community for inspiration and ideas

## 📞 Support

- 🐛 [Issues](https://github.com/yourusername/rackit/issues)
- 💡 [Discussions](https://github.com/yourusername/rackit/discussions)
- 📧 Email: your.email@example.com

---

⭐ If this project was helpful, please star it on GitHub! 