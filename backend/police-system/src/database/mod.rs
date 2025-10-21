pub mod connection;
pub mod queries;

// Re-export connection function
pub use connection::establish_connection;

// Re-export all query functions
pub use queries::{
    get_all_suspects,
    get_suspect_by_id,
    get_suspect_by_personal_id,
    create_suspect,
    update_suspect,
    delete_suspect,
    update_flag,
};