use std::env;
use url::{self, Url};
use anyhow::Result;
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
    let args: Vec<String> = env::args().collect();

    //eprintln!("{:?}", args);

    if args.len() > 1 {
        // see if the first argument is a URL
        let url_attempt = Url::parse(&args[1]);

        if args.len() > 1 && args.len() < 4 && url_attempt.is_ok() {
            let opt_code = if args.len() == 3 {
                Some(args[2].as_str()) // there is a code
            } else {
                None // no code
            };
            download::download(url_attempt.unwrap(), opt_code)?;
        } else { // assume they're all filenames
            upload::upload(&args[1..])?;
        }
    } else {
        eprintln!("bup version {}

{}

{}

Usage: bup URL                      # download and decrypt
       bup SHORT_URL TELEPHONE_CODE # download and decrypt
       bup FILENAME [FILENAMES...]  # encrypt and upload", 
            env!("CARGO_PKG_VERSION"),
            env!("CARGO_PKG_DESCRIPTION"),
            env!("CARGO_PKG_HOMEPAGE"),
        );
    }

    Ok(())
}
