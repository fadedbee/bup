use std::{fs::File, io::{BufReader, Read, ErrorKind}};
use anyhow::Result;
//use qr2term;
//use libc::size_t;

use crate::ttbytes;

const BLOCK_SIZE: usize = 1_000_000;

fn encryptAndUploadBlock(buf: &[u8], n: usize) -> Result<()> {
    Ok(())
}

fn uploadFile(filename: &str) -> Result<()> {
    println!("uploading {filename}");
    //qr2term::print_qr("https://rust-lang.org/");
    let mut file = File::open(filename)?;

    let mut num_bytes = 0usize;
    let mut reader = BufReader::new(file);
    let mut buf = [0u8; BLOCK_SIZE];
    loop {
        match reader.read(&mut buf) {
            Ok(0) => break,
            Ok(n) => {
                encryptAndUploadBlock(&buf, n);
                num_bytes += n;
            }
            Err(ref e) if e.kind() == ErrorKind::Interrupted => continue,
            Err(e) => panic!("{:?}", e),
        };
    }

    Ok(())
}

pub fn upload(filenames: &[String]) -> Result<()> {
    for filename in filenames {
        uploadFile(filename)?;
    }

    Ok(())
}