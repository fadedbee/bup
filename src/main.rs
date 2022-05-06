use std::env;
use url::{self, Url};
use anyhow::Result;
use serde::{Deserialize, Serialize};

mod upload;
mod download;
mod ttbytes;
    
pub const BLOCK_SIZE: usize = 1_000_000;
pub const URL: &str = "http://localhost:3000";
   
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IndexEntry {
    name: String,
    size: u64,
    keys: Vec<String>

}
fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();

    //eprintln!("{:?}", args);

    if args.len() > 1 {
        // see if the first argument is a URL
        let url_attempt = Url::parse(&args[1]);

        if args.len() == 2 && url_attempt.is_ok() {
            download::download(url_attempt.unwrap())?;
        } else { // assume they're all filenames
            upload::upload(&args[1..])?;
        }
    } else if args.len() == 1 { // no arguments
        eprintln!("bup version {}

{}

{}

Usage: bup URL                      # download and decrypt
       bup FILENAME [FILENAMES...]  # encrypt and upload", 
            env!("CARGO_PKG_VERSION"),
            env!("CARGO_PKG_DESCRIPTION"),
            env!("CARGO_PKG_HOMEPAGE"),
        );
    } else {
        // This will never be executed, but it allows us to keep code which we may need in future
        // without warnings being displayed at compilation time.
        ttbytes::TTBytes::from_bytes_be(&[0u8; 32]).lower_dashed_base33();
    }

    Ok(())
}
