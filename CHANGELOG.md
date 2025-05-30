# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.1.0] - 2024-01-XX

### Added
- Initial release of Rackit
- Universal configuration engine for network devices
- Support for TOML configuration files
- CLI interface with commands: run, plan, validate, list, example, check, shell
- Eltex ESR-200 device support via Serial transport
- Serial port communication transport
- Error handling with flexible strategies
- Example configurations and templates
- Comprehensive documentation in Russian and English
- MIT license

### Features
- **CLI Commands**: 
  - `rackit run` - Execute configuration (like ansible-playbook)
  - `rackit plan` - Show execution plan (like terraform plan)
  - `rackit validate` - Validate configuration (like terraform validate)
  - `rackit list` - List devices and scenarios
  - `rackit example` - Create example configurations
  - `rackit check` - Check device status
  - `rackit shell` - Interactive device access

- **Device Support**:
  - Eltex ESR-200 routers
  - Extensible architecture for new devices

- **Transport Support**:
  - Serial/COM ports
  - USB-to-Serial adapters
  - Configurable baud rates and timeouts

- **Configuration Features**:
  - TOML-based configuration files
  - Multiple devices in single config
  - Command sequences with error handling
  - Global settings and device-specific overrides
  - Step types: login, logout, command, delay, wait_prompt, check_response

### Documentation
- Comprehensive README in Russian and English
- Configuration examples
- API documentation
- Installation and usage guides

### Development
- Modular Rust architecture
- Error handling with thiserror
- CLI with clap derive
- Serialization with serde
- Serial communication with serialport crate 