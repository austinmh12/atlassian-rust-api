// Modules
mod error;
mod rest_client;
mod jira;
mod web;

// Flatten
pub use error::{Error, Result};
pub use jira::{Jira, JiraBuilder};

// Public Modules