use std::{fs::File, io::{BufReader, Read, ErrorKind}};
use anyhow::{anyhow, Result};
//use qr2term;
//use libc::size_t;
use rand::Rng;
use reqwest;
use serde::{Deserialize, Serialize};
use crate::ttbytes::TTBytes;
use serde_json;

const BLOCK_SIZE: usize = 1_000_000;
const URL: &str = "http://localhost:3000";


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IndexEntry {
    name: String,
    size: u64,
    keys: Vec<String>
}

fn encrypt_and_upload_block(buf: &[u8], block_num: usize) -> Result<TTBytes> {
    println!("encrypting and uploading block {}", block_num);
    // TODO: is thread_rng() good enough?
    let key = TTBytes::from_bytes_be(&rand::thread_rng().gen::<[u8; 32]>());
    let ciphertext = key.encrypt(buf, block_num);

    let client = reqwest::blocking::Client::new();
    let block_id = key.hash().upper_base62();
    let url = format!("{}/upload/block/{}", URL, block_id);
    let res = client.put(url)
        .body(ciphertext)
        .send()?;

    //eprintln!("response: {:?}", res);
    let status = res.status().as_u16();
    if status >= 200 && status < 300 {
        Ok(key)
    } else {
        // TODO: bail!(MyError::MyVariant { actual: 0, expected: 1 })
        Err(anyhow!("Upload failed.  Status code: {}.", status))
    }
}

fn upload_file(filename: &str) -> Result<IndexEntry> {
    println!("uploading {filename}");
    //qr2term::print_qr("https://rust-lang.org/");
    let file = File::open(filename)?;

    let len = file.metadata().unwrap().len();
    let mut reader = BufReader::new(file);
    let mut buf = [0u8; BLOCK_SIZE];
    let mut final_buf = vec![0u8; len as usize % BLOCK_SIZE];
    let mut keys: Vec<TTBytes> = Vec::new();

    for i in 0..(len/BLOCK_SIZE as u64) { // FIXME: do files < 1MB crash this?
        if i < (len/BLOCK_SIZE as u64) - 1 { // all blocks except last
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

    Ok (
        IndexEntry {
            name: filename.to_string(),
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

    let index_json = serde_json::to_string(&index).unwrap();
    eprintln!("index: {:?}", index_json);

    // TODO: upload JSON index 
    let index_key = encrypt_and_upload_block(index_json.as_bytes(), 0)?;
    println!("{}/#{}", URL, index_key.base62());

    Ok(())
}