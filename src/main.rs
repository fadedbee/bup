use std::env;
use url::{self, Url};
use anyhow::{ensure, Result};
use serde::{Deserialize, Serialize};

mod upload;
mod download;
mod ttbytes;
    
pub const BLOCK_SIZE: usize = 1_000_000;
pub const CACHE_URL: &str = "https://ca1.blindupload.org";
pub const WWW_URL: &str = "https://www.blindupload.org";
   
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IndexEntry {
    name: String,
    size: u64,
    keys: Vec<String>

}

fn main() -> Result<()> {
    // we can't use "clap" or similar as we want to be able to determine whether to download or
    // upload depeneding on whether the first srgument is a URL (to save the user some typing)
    let mut args: Vec<String> = env::args().collect();

    eprintln!("{:?}", args);

    // are any of the arguments --split-key
    let num_before = args.len();
    args.retain(|x| x != "--split-key");

    // work out whether a --split-key option was present (and has been removed)
    let split_key = args.len() != num_before; 

    if args.len() > 1 {
        // see if the first argument is a URL
        let url_attempt = Url::parse(&args[1]);

        if url_attempt.is_ok() { // it's a URL, we're going to deownload/decrypt
            let opt_code = if split_key {
                ensure!(args.len() == 3, "split key requires a URL and a telephone code");
                Some(args[2].as_str()) // there is a code
            } else {
                ensure!(args.len() == 2, "only a URL argument is required");
                None // no code
            };
            download::download(url_attempt.unwrap(), opt_code)?;
        } else { // assume they're all filenames to encrypt/upload
            upload::upload(&args[1..], split_key)?;
        }
    } else {
        eprintln!("bup version {}

{}

{}

Usage: bup URL                                    # download and decrypt
       bup --split-key URL TELEPHONE_CODE         # download and decrypt
       bup [--split-key] FILENAME [FILENAMES...]  # encrypt and upload", 
            env!("CARGO_PKG_VERSION"),
            env!("CARGO_PKG_DESCRIPTION"),
            env!("CARGO_PKG_HOMEPAGE"),
        );
    }

    Ok(())
}
