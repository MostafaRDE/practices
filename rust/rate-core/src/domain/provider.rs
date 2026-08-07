use super::rate::Rate;

pub trait Provider {
    async fn fetch(&self) -> std::io::Result<Vec<Rate>>;
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
