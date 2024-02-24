// In loaders/mod.rs
pub mod loader_trait;
pub mod file_loader;
pub mod database_loader;

pub use loader_trait::DataLoader;
pub use file_loader::FileLoader;
pub use database_loader::DatabaseLoader;
