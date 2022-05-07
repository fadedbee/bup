# bup
A command-line client for [Blind Upload](https://www.blindupload.org/about.html).
```
Usage: bup URL                                    # download and decrypt
       bup --split-key URL TELEPHONE_CODE         # download and decrypt
       bup [--split-key] FILENAME [FILENAMES...]  # encrypt and upload
```
[Download pre-compiled linux_x86-64 executable.](https://github.com/fadedbee/bup/blob/main/linux_x86-64_executable/bup)

Or compile it yourself from source, once you've [installed Rust](https://www.rust-lang.org/tools/install):
```
fadedbee@box:~$ cargo install bup
    Updating crates.io index
  Installing bup v0.1.5
   Compiling autocfg v1.1.0
           ...
   Compiling reqwest v0.11.10
   Compiling bup v0.1.5
    Finished release [optimized] target(s) in 1m 17s
  Installing /home/fadedbee/.cargo/bin/bup
   Installed package `bup v0.1.5` (executable `bup`)

fadedbee@box:~$ bup
bup version 0.1.5

A command-line client for Blind Upload.

https://github.com/fadedbee/bup

Usage: bup URL                                    # download and decrypt
       bup --split-key URL TELEPHONE_CODE         # download and decrypt
       bup [--split-key] FILENAME [FILENAMES...]  # encrypt and upload
```
