use super::rate::Rate;

pub trait Provider {
    async fn fetch(&self) -> std::result::Result<Vec<Rate>, ProviderError>;
}

#[derive(Debug, thiserror::Error)]
pub enum ProviderError {
    #[error("Network error")]
    Network,

    #[error("Invalid response")]
    InvalidResponse,
    
    #[error("Timeout")]
    Timeout,
}
