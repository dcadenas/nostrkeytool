#![doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/README.md"))]

use std::str::FromStr;

use anyhow::{Context, Result};
use clap::{Arg, Command};
use nostr_sdk::prelude::*;

#[tokio::main]
async fn main() -> Result<()> {
    let no_arguments_provided = std::env::args().len() <= 1;

    if no_arguments_provided {
        println!("No arguments provided. Use --help for usage information.");
        return Ok(());
    }

    let matches = Command::new("nostrkeytool")
        .version("0.1.0")
        .author("Daniel Cadenas")
        .about("Tool for NOSTR key operations")
        .arg(
            Arg::new("sec2npub")
                .long("sec2npub")
                .help("Input: secret, Output: npub"),
        )
        .arg(
            Arg::new("sec2pubkey")
                .long("sec2pubkey")
                .help("Input: secret, Output: pubkey"),
        )
        .arg(
            Arg::new("pubkey2npub")
                .long("pubkey2npub")
                .help("Input: pubkey, Output: npub"),
        )
        .arg(
            Arg::new("npub2pubkey")
                .long("npub2pubkey")
                .help("Input: npub, Output: pubkey"),
        )
        .arg(
            Arg::new("gen")
                .long("gen")
                .action(clap::ArgAction::SetTrue)
                .help("Generate a secret"),
        )
        .get_matches();

    if let Some(secret) = matches.get_one::<String>("sec2npub") {
        let result = process_sec2npub(secret).await?;
        println!("{}", result);
    } else if let Some(secret) = matches.get_one::<String>("sec2pubkey") {
        let result = process_sec2pubkey(secret).await?;
        println!("{}", result);
    } else if let Some(pubkey) = matches.get_one::<String>("pubkey2npub") {
        let result = process_pubkey2npub(pubkey).await?;
        println!("{}", result);
    } else if let Some(npub) = matches.get_one::<String>("npub2pubkey") {
        let result = process_npub2pubkey(npub).await?;
        println!("{}", result);
    } else if matches.contains_id("gen") {
        let result = process_gen().await?;
        println!("{}", result);
    } else {
        println!("No valid arguments provided. Use --help for usage information.");
    }

    Ok(())
}

async fn process_sec2npub(secret: &str) -> Result<String> {
    let keys = Keys::from_sk_str(secret).context("Error creating keys from secret")?;
    let npub = keys
        .public_key()
        .to_bech32()
        .context("Error converting public key to npub")?;
    Ok(npub)
}

async fn process_sec2pubkey(secret: &str) -> Result<String> {
    let keys = Keys::from_sk_str(secret).context("Error creating keys from secret")?;
    let pubkey = keys.public_key().to_string();
    Ok(pubkey)
}

async fn process_pubkey2npub(pubkey: &str) -> Result<String> {
    let pubkey = XOnlyPublicKey::from_str(pubkey);

    let npub = XOnlyPublicKey::to_bech32(&pubkey?)?;
    Ok(npub)
}

async fn process_npub2pubkey(npub: &str) -> Result<String> {
    let pubkey = XOnlyPublicKey::from_bech32(npub)?.to_string();
    Ok(pubkey)
}

async fn process_gen() -> Result<String> {
    let keys = Keys::generate();

    let secret_key = keys.secret_key().context("Error generating secret key")?;
    let secret = secret_key.display_secret().to_string();

    Ok(secret)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_process_gen() {
        let result = process_gen().await.unwrap();
        assert_eq!(result.len(), 64);
    }

    #[tokio::test]
    async fn test_process_sec2npub() {
        let secret = "51ce70ac70753e62f9baf4a8ce5e1334c30360ab14783016775ecb42dc322571";
        let result = process_sec2npub(secret).await.unwrap();

        assert_eq!(
            result,
            "npub1jg44kjf32wtt6cplc7kmyvq3w3jj3pw0yrjq2tk2yp3dpsuh9nps4re52r"
        );
    }

    #[tokio::test]
    async fn test_process_sec2pubkey() {
        let secret = "51ce70ac70753e62f9baf4a8ce5e1334c30360ab14783016775ecb42dc322571";
        let result = process_sec2pubkey(secret).await.unwrap();

        assert_eq!(
            result,
            "922b5b49315396bd603fc7adb2301174652885cf20e4052eca2062d0c3972cc3"
        );
    }

    #[tokio::test]
    async fn test_process_npub2pubkey() {
        let npub = "npub1jg44kjf32wtt6cplc7kmyvq3w3jj3pw0yrjq2tk2yp3dpsuh9nps4re52r";
        let result = process_npub2pubkey(npub).await.unwrap();

        assert_eq!(
            result,
            "922b5b49315396bd603fc7adb2301174652885cf20e4052eca2062d0c3972cc3"
        );
    }

    #[tokio::test]
    async fn test_process_pubkey2npub() {
        let pubkey = "922b5b49315396bd603fc7adb2301174652885cf20e4052eca2062d0c3972cc3";
        let result = process_pubkey2npub(pubkey).await.unwrap();

        assert_eq!(
            result,
            "npub1jg44kjf32wtt6cplc7kmyvq3w3jj3pw0yrjq2tk2yp3dpsuh9nps4re52r"
        );
    }
}
