use crate::error::types::Result;
use std::time::Duration;

pub trait Transport {
    // Connect to device 
    fn connect(&mut self) -> Result<()>;
    // Disconnect from device           
    fn disconnect(&mut self) -> Result<()>;
    // Send data to device
    fn send(&mut self, data: &[u8]) -> Result<()>;
    // Get data on delimiter (Example: "\n")
    fn receive_until(&mut self, delimiter: u8) -> Result<Vec<u8>>;
    // Set timeout for device
    fn set_timeout(&mut self, timeout: Duration) -> Result<()>;
    
    fn send_string(&mut self, data: &str) -> Result<()> {
        let mut buffer = data.as_bytes().to_vec();
        buffer.extend_from_slice(b"\r\n");
        self.send(&buffer)
    }
    
    fn receive_string(&mut self) -> Result<String> {
        let buffer = self.receive_until(b'\n')?;
        Ok(String::from_utf8_lossy(&buffer).to_string())
    }
    
    fn send_command(&mut self, command: &str) -> Result<String> {
        self.send_string(command)?;
        self.receive_string()
    }
    
    fn receive_data(&mut self) -> Result<String> {
        self.receive_string()
    }

    // Новый метод для чтения до промпта
    fn receive_until_prompt(&mut self, prompt: &str) -> Result<String>;
}