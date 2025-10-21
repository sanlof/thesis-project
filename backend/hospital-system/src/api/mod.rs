pub mod patients;
pub mod shared;

// Re-export configuration functions
pub use patients::configure_patients;
pub use shared::configure_shared;