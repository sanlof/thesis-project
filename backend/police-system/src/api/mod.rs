pub mod suspects;
pub mod shared;

// Re-export configuration functions
pub use suspects::configure_suspects;
pub use shared::configure_shared;