use std::path::Path;

use anyhow::Result;
use pgp::{packet::PublicKey, Deserializable, Message};

pub fn sign() {}

/// Check that the given artifact was signed by the private key.
pub fn verify<P>(path: P) -> Result<()>
where
    P: AsRef<Path>,
{
    let public_key = PublicKey::from_slice(pgp::types::Version::New, include_bytes!("public.key"))?;

    // TODO don't read entirely
    let content = std::fs::read_to_string(path.as_ref())?;
    let (msg, _) = Message::from_string(&content)?;

    msg.verify(&public_key)?;
    Ok(())
}
