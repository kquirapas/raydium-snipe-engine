use thiserror::Error;

#[derive(Error, Clone, Debug)]
pub enum SnipeError {
    #[error("Failed to write keypair to file")]
    FailedToWriteKeypair,

    #[error("Failed to read keypair from file")]
    FailedToReadKeypair,
}
