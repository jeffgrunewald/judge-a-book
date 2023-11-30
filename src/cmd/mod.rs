use clap::Subcommand;

mod fetch_covers;

pub use fetch_covers::Resolution;

// An extension point for adding additional commands over time
#[derive(Debug, Subcommand)]
pub enum Cmd {
    /// Fetch the cover images for the books in the queried collection
    FetchCovers(fetch_covers::FetchCoversArgs),
}
