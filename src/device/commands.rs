use crate::error::types::Result;

pub trait DeviceCommands {
    fn execute_command(&mut self, command: &str) -> Result<String>;
    fn disconnect(&mut self) -> Result<()>;
}



