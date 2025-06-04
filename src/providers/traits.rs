use std::collections::HashMap;
use std::fmt::Debug;
use serde::{Deserialize, Serialize};


pub type ProviderResult<T> = Result<T, Box<dyn std::error::Error>>;


pub trait Provider: Debug + Send + Sync {

    /// Тип конфигурации машины
    type MachineConfig: Debug + Clone + Serialize + for<'de> Deserialize<'de>;
    /// Тип идентификатора машины
    type MashineId: Debug + Clone + Serialize + for<'de> Deserialize<'de> + Send + Sync + 'static;

    type MachineInfo: Debug + Clone + Serialize + for<'de> Deserialize<'de> + Send + Sync + 'static;

    fn provider_name(&self) -> &str;


    fn is_available(&self) -> bool;


    fn list_machines(&self) -> ProviderResult<Vec<Self::MachineId>>;

    fn get_machine_info(&self, machine_id: &Self::MachineId) -> ProviderResult<Self::MachineInfo>;

    fn create_machine(&self, machine_id: &Self::MachineId, config: &Self::MachineConfig) -> ProviderResult<()>{
        Err("Machine creation is not supported by this provider".into())
    };

    
}
