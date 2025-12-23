use thiserror::Error;

pub type Result<T> = std::result::Result<T, AudioTranscriptionError>;

#[derive(Error, Debug)]
pub enum AudioTranscriptionError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Audio processing error: {0}")]
    Audio(String),

    #[error("Model error: {0}")]
    Model(String),

    #[error("Network error: {0}")]
    Network(#[from] reqwest::Error),

    #[error("Hugging Face Hub error: {0}")]
    HuggingFaceHub(#[from] hf_hub::api::tokio::ApiError),

    #[error("Serialization error: {0}")]
    Serialization(#[from] serde_json::Error),

    #[error("File browser error: {0}")]
    FileBrowser(String),

    #[error("Unsupported audio format: {0}")]
    UnsupportedFormat(String),

    #[error("Insufficient memory: {0}")]
    InsufficientMemory(String),

    #[error("GPU acceleration unavailable: {0}")]
    GpuUnavailable(String),

    #[error("Configuration error: {0}")]
    Configuration(String),
}