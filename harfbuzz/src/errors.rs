use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("No Memory")]
    NoMemory,
}
