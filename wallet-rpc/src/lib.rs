pub mod basic;
pub mod client;
pub mod error;

pub use client::HandshakeWalletRpcClient;
pub use error::Error;

pub type Result<T> = std::result::Result<T, Error>;
