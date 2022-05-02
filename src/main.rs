use std::env;
use url::{self, Url};
use anyhow::Result;

mod upload;
mod download;
mod ttbytes;
    
fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();

    println!("{:?}", args);

    if args.len() > 1 {
        // see if the first argument is a URL
        let url_attempt = Url::parse(&args[1]);

        if args.len() == 2 && url_attempt.is_ok() {
            download::download(url_attempt.unwrap());
        } else { // assume they're all filenames
            upload::upload(&args[1..])?;
        }
    } else { // no arguments
        eprintln!("
Usage: bup URL                      # download and decrypt
       bup FILENAME [FILENAMES...]  # encrypt and upload");
    }

    Ok(())
}
