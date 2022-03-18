/// Sabaton Update Manager Interface
/// 
use someip_derive::{*};
use someip::{*};
use thiserror::Error;
use serde::{Serialize, Deserialize};
use async_trait::async_trait;




#[service(name("dev.sabaton.ExampleInterface")
    fields([1]status:ExampleStatus)
)]
#[async_trait]
pub trait Example {
    /// Get the list of Software Clusters
    async fn echo(&self, data: String) -> Result<EchoResponse, ExampleError>;
    
    
}


#[derive(Serialize, Deserialize,Clone, Default)]
pub struct EchoResponse  {
    pub echo : String,
}


#[derive(Serialize, Deserialize,Clone)]
pub enum ExampleStatus  {
    Starting,
    Ready,
}

impl Default for ExampleStatus {
    fn default() -> ExampleStatus {
        Self::Starting
    }
}

#[derive(Error, Debug, Serialize, Deserialize)]
pub enum ExampleError {
    #[error("IoError")]
    IoError,
    #[error("Out of Memory")]
    OutOfMemory,
    #[error("Unknown")]
    Unknown,
}


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
