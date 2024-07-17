use thiserror::Error;

#[derive(Error, Clone, Debug)]
pub enum SnipeError {
    #[error("placeholder")]
    NoSubmission(String),
}
