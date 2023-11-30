mod client;
mod cmd;

pub use cmd::Cmd;

pub type Result<T = (), E = anyhow::Error> = anyhow::Result<T, E>;
