pub mod connection;
pub mod queries;

// Re-export connection function
pub use connection::establish_connection;

// Re-export all query functions
pub use queries::{
    get_all_patients,
    get_patient_by_id,
    get_patient_by_personal_id,
    create_patient,
    update_patient,
    delete_patient,
    get_flagged_patients,
};