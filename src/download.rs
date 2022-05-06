use std::{str, io::{Read, Write}, path::Path, fs::{File, create_dir_all}};
use url::Url;
use reqwest::{self, StatusCode};
use anyhow::{anyhow, Result};
use crate::{URL, ttbytes::TTBytes, IndexEntry};
use serde_json;

const SLASH_HTML: &[u8] = b"</html>\n";
const TAIL_LENGTH: usize = 32;

fn download_block(url: Url) -> Result<Vec<u8>> {
    //eprintln!("downloading {url}");
    let client = reqwest::blocking::Client::new();
    let mut res = client.get(url)
        .send()?;

    // TODO: Could this be better?
    match res.status() {
        StatusCode::OK => Ok(()),
        StatusCode::NOT_FOUND => Err(anyhow!("404 file not found.  Incorrect code or expired link.")),
        _ => Err(anyhow!("Unexpected HTTP code: {}", res.status().as_u16()))
    }?;

    let mut padded_block: Vec<u8> = Vec::new();
    res.read_to_end(&mut padded_block)?;

    // these buffers are prefixed with some error HTML and suffixed with a 32 byte checksum
    'outer:
    for i in 0..(padded_block.len() - SLASH_HTML.len()) {
        for j in 0..SLASH_HTML.len() {
            if padded_block[i+j] != SLASH_HTML[j] {
                continue 'outer; // not found it yet
            }
        }
        // found it
        //eprintln!("i: {i}");
        let trimmed = &padded_block[(i + SLASH_HTML.len())..(padded_block.len() - TAIL_LENGTH)];
        // TODO: check sha256sum
        //eprintln!("trimmed.len(): {}", trimmed.len());
        return Ok(trimmed.to_vec());
    }

    Err(anyhow!("</html>\\n not found in download"))
}


pub fn download(url: Url) -> Result<()> {
    // download index
    let fragment = url.fragment().unwrap_or("");
    //eprintln!("{fragment}");
    // TODO: check fragment length
    println!("downloading and decrypting index from {url}");

    let index_key = TTBytes::from_base62(fragment);
    let index_block_id = index_key.hash().upper_base62();
    let index_url = Url::parse(&format!("{URL}/block/{index_block_id}"))?;
    let index_block = download_block(index_url)?;
    let index_plaintext = index_key.decrypt(&index_block);
    let index_json = str::from_utf8(&index_plaintext)?;

    //eprintln!("index_json: {}", index_json);

    let index: Vec<IndexEntry> = serde_json::from_str(index_json).unwrap();

    let dir_name = index_block_id.to_lowercase(); // for ease of typing
    let path = Path::new(".").join(dir_name);
    create_dir_all(&path)?;
    for index_entry in &index {
        let file_path = path.join(&index_entry.name);
        println!("downloading and decrypting {} into {}", &index_entry.name, &file_path.display());
        let mut file = File::create(&file_path)?;
        for i in 0..index_entry.keys.len() {
            println!("block {i}");
            let key = TTBytes::from_base62(&index_entry.keys[i]);
            let block_id = key.hash().upper_base62();
            let url = Url::parse(&format!("{URL}/block/{block_id}"))?;
            let block = download_block(url)?;
            let plaintext = key.decrypt(&block);
            // TODO: check block length is BLOCK_SIZE if not final block
            file.write_all(&plaintext)?;
        }
    }
    println!("download complete");
    Ok(())
}