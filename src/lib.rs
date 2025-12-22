pub mod cli;
pub mod core;
pub mod ui;
pub mod error;

pub use crate::error::Result;

// Re-export the ModelSize enum for use in tests
// This is a duplicate of the one in main.rs but needed for library tests
#[derive(Clone, Debug)]
pub enum ModelSize {
    Tiny,
    Base,
    Small,
    Medium,
    Large,
}

impl std::fmt::Display for ModelSize {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ModelSize::Tiny => write!(f, "tiny"),
            ModelSize::Base => write!(f, "base"),
            ModelSize::Small => write!(f, "small"),
            ModelSize::Medium => write!(f, "medium"),
            ModelSize::Large => write!(f, "large"),
        }
    }
}