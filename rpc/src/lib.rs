pub mod client;
pub mod error;
pub mod responses;

pub use client::HandshakeRpcClient;
pub use error::Error;

pub type Result<T> = std::result::Result<T, Error>;
