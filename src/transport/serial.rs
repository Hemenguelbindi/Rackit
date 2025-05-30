use crate::error::types::Result;
use serialport::SerialPort;
use std::time::Duration;
use std::io::{Read, Write};

pub struct SerialTransport {
    port_name: String,
    baud_rate: u32,
    port: Option<Box<dyn SerialPort>>,
}

impl SerialTransport {
    pub fn new(port_name: String, baud_rate: u32) -> Self {
        Self {
            port_name,
            baud_rate,
            port: None,
        }
    }
}

impl super::traits::Transport for SerialTransport {
    fn connect(&mut self) -> Result<()> {
        let port = serialport::new(&self.port_name, self.baud_rate)
            .timeout(Duration::from_millis(3000))
            .open()?;
        
        self.port = Some(port);
        println!("Connected to {}", self.port_name);
        Ok(())
    }

    fn disconnect(&mut self) -> Result<()> {
        if self.port.is_some() {
            self.port = None;
            println!("Disconnected from {}", self.port_name);
        }
        Ok(())
    }

    fn send(&mut self, data: &[u8]) -> Result<()> {
        match &mut self.port {
            Some(port) => {
                port.write_all(data)?;
                port.flush()?;
                Ok(())
            }
            None => Err(crate::error::types::Error::Connection("Port not connected".to_string()))
        }
    }

    fn receive_until(&mut self, delimiter: u8) -> Result<Vec<u8>> {
        match &mut self.port {
            Some(port) => {
                let mut buffer = Vec::new();
                let mut byte = [0u8; 1];
                
                loop {
                    match port.read_exact(&mut byte) {
                        Ok(_) => {
                            buffer.push(byte[0]);
                            if byte[0] == delimiter {
                                break;
                            }
                        }
                        Err(e) => return Err(e.into()),
                    }
                }
                Ok(buffer)
            }
            None => Err(crate::error::types::Error::Connection("Port not connected".to_string()))
        }
    }

    fn set_timeout(&mut self, timeout: Duration) -> Result<()> {
        match &mut self.port {
            Some(port) => {
                port.set_timeout(timeout)?;
                Ok(())
            }
            None => Err(crate::error::types::Error::Connection("Port not connected".to_string()))
        }
    }

    fn receive_until_prompt(&mut self, prompt: &str) -> Result<String> {
        match &mut self.port {
            Some(port) => {
                let mut buffer = Vec::new();
                let mut temp_buffer = [0u8; 1024];
                let start_time = std::time::Instant::now();
                let timeout_duration = Duration::from_secs(10); // 10 секунд таймаут
                
                loop {
                    // Проверяем таймаут
                    if start_time.elapsed() > timeout_duration {
                        break;
                    }
                    
                    match port.read(&mut temp_buffer) {
                        Ok(bytes_read) if bytes_read > 0 => {
                            buffer.extend_from_slice(&temp_buffer[..bytes_read]);
                            
                            // Преобразуем в строку для проверки
                            let text = String::from_utf8_lossy(&buffer);
                            println!("DEBUG: Читаем данные ({} байт): '{}'", buffer.len(), text);
                            
                            // Если нашли промпт - возвращаем результат
                            if text.contains(prompt) {
                                return Ok(text.to_string());
                            }
                        }
                        Ok(_) => {
                            // Нет данных, ждем еще
                            std::thread::sleep(Duration::from_millis(50));
                        }
                        Err(ref e) if e.kind() == std::io::ErrorKind::TimedOut => {
                            // Таймаут чтения - продолжаем ждать
                            std::thread::sleep(Duration::from_millis(50));
                        }
                        Err(e) => return Err(e.into()),
                    }
                }
                
                // Если вышли по таймауту - возвращаем что накопили
                Ok(String::from_utf8_lossy(&buffer).to_string())
            }
            None => Err(crate::error::types::Error::Connection("Port not connected".to_string()))
        }
    }
}


