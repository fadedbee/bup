use std::{fs::File, io::{BufReader, Read}, path::Path};
use anyhow::{anyhow, Result};
use rand::{rngs::OsRng, RngCore};
use reqwest;
use crate::{BLOCK_SIZE, WWW_URL, CACHE_URL, ttbytes::TTBytes, IndexEntry};
use serde_json;

fn encrypt_and_upload_block(buf: &[u8], block_num: usize) -> Result<TTBytes> {
    println!("encrypting and uploading block {}", block_num);
    // TODO: is thread_rng() good enough?
    let mut rng_buf = [0u8; 32];
    OsRng.fill_bytes(&mut rng_buf);
    let key = TTBytes::from_bytes_be(&rng_buf);
    let ciphertext = key.encrypt(buf);

    let client = reqwest::blocking::Client::new();
    let block_id = key.hash().upper_base62();
    let url = format!("{CACHE_URL}/upload/block/{block_id}");
    let res = client.put(url)
        .body(ciphertext)
        .send()?;

    //eprintln!("response: {:?}", res);
    let status = res.status().as_u16();
    if status >= 200 && status < 300 {
        Ok(key)
    } else {
        // TODO: bail!(MyError::MyVariant { actual: 0, expected: 1 })
        Err(anyhow!("Encryption/upload failed.  Status code: {}.", status))
    }
}

fn upload_file(filename: &str) -> Result<IndexEntry> {
    println!("processing {filename}...");
    let file = File::open(filename)?;

    let len = file.metadata().unwrap().len();
    let mut reader = BufReader::new(file);
    let mut buf = [0u8; BLOCK_SIZE];
    let mut final_buf = vec![0u8; len as usize % BLOCK_SIZE];
    let mut keys: Vec<TTBytes> = Vec::new();
    let num_blocks = ((len - 1)/BLOCK_SIZE as u64) + 1; // FIXME: zero-length files

    for i in 0..num_blocks {
        if i < num_blocks - 1 { // all blocks except last
            match reader.read_exact(&mut buf) {
                Ok(()) => {
                    keys.push(encrypt_and_upload_block(&buf, i as usize)?);
                }
                // FIXME: Err(ref e) if e.kind() == ErrorKind::Interrupted => continue,
                Err(e) => panic!("{:?}", e),
            };
        } else {
            match reader.read_exact(&mut final_buf) { // final block (it could be BLOCK_SIZE too)
                Ok(()) => {
                    keys.push(encrypt_and_upload_block(&final_buf, i as usize)?);
                }
                // FIXME: Err(ref e) if e.kind() == ErrorKind::Interrupted => continue,
                Err(e) => panic!("{:?}", e),
            };
        }
    }

    let name = Path::new(filename).file_name().unwrap().to_string_lossy().to_string();

    Ok (
        IndexEntry {
            name,
            size: len,
            keys: keys.iter().map(|x| x.base62()).collect()
        }
    )
}

pub fn upload(filenames: &[String]) -> Result<()> {
    let mut index: Vec<IndexEntry> = Vec::new();

    for filename in filenames {
        let entry = upload_file(filename)?;
        index.push(entry);
    }

    println!("processing index...");
    let index_json = serde_json::to_string(&index).unwrap();
    //eprintln!("index: {:?}", index_json);

    // TODO: upload JSON index 
    let index_key = encrypt_and_upload_block(index_json.as_bytes(), 0)?;
    println!("processing complete");

    // single key
    let download_url = format!("{WWW_URL}/#{}", index_key.base62());
    println!("");
    println!("{download_url}");
    println!("");
    qr2term::print_qr(download_url)?;

    // split keys
    let split_url = format!("{WWW_URL}/#{}", index_key.upper_base62());
    let code = index_key.lower_dashed_base33();
    println!("");
    println!("Split-keys (advanced):");
    println!("");
    println!("{split_url}");
    println!("");
    println!("Telephone Code: {code}");
    println!("");

    Ok(())
}