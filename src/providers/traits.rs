use std::collections::HashMap;
use std::fmt::Debug;
use serde::{Deserialize, Serialize};


pub type ProviderResult<T> = Result<T, Box<dyn std::error::Error>>;


pub trait Provider: Debug + Send + Sync {

    /// Тип конфигурации машины
    type MachineConfig: Debug + Clone + Serialize + for<'de> Deserialize<'de>;
    /// Тип идентификатора машины
    type MashineId: Debug + Clone + Serialize + for<'de> Deserialize<'de> + Send + Sync + 'static;

}
