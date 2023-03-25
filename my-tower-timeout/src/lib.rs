pub mod timeout;

pub type BoxError = Box<dyn std::error::Error + Send + Sync>;
