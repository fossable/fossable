use clap::{Parser, Subcommand};
use std::error::Error;

pub mod words;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
struct Args {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Create cryptographic signature for artifact
    #[cfg(feature = "verify")]
    Sign {
        /// File path of the artifact to sign
        artifact: String,

        /// File path of signing (private!) key
        key: String,
    },

    /// Verify cryptographic signature for artifact
    #[cfg(feature = "verify")]
    Verify {
        /// File path of the artifact to verify
        artifact: String,
    },
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();
    match &args.command {
        #[cfg(feature = "verify")]
        Commands::Sign {
            artifact: _,
            key: _,
        } => todo!(),
        #[cfg(feature = "verify")]
        Commands::Verify { artifact } => fossable::signing::verify(artifact)?,
        _ => todo!(),
    };
    Ok(())
}
