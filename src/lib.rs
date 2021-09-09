#[cfg(not(feature = "no-entrypoint"))]
mod entrypoint;

/// ENSURE to register the module upon definition!!!
pub mod error;
pub mod instruction;
pub mod processor;
pub mod state;
