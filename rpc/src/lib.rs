mod block;
mod chain;
mod mempool;
mod mining;
mod names;
mod network;
mod node;
// mod tx;
pub mod client;
pub mod error;

pub use client::HandshakeRpcClient;
pub use error::Error;

pub type Result<T> = std::result::Result<T, Error>;
