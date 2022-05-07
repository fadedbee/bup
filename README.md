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
```

Check the usage:
```
fadedbee@box:~$ bup
bup version 0.1.5

A command-line client for Blind Upload.

https://github.com/fadedbee/bup

Usage: bup URL                                    # download and decrypt
       bup --split-key URL TELEPHONE_CODE         # download and decrypt
       bup [--split-key] FILENAME [FILENAMES...]  # encrypt and upload
```

Encrypt and upload some files:
```
fadedbee@box:~$ bup myfile.mp3 myotherfile.txt 
processing myfile.mp3...
encrypting and uploading block 0
encrypting and uploading block 1
encrypting and uploading block 2
encrypting and uploading block 3
encrypting and uploading block 4
processing myotherfile.txt...
encrypting and uploading block 0
encrypting and uploading block 1
encrypting and uploading block 2
encrypting and uploading block 3
processing index...
encrypting and uploading block 0
processing complete

                                         
   ▄▄▄▄▄  ▄ ▄▄   ▄▄▄▄ ▄     ▄ ▄  ▄▄▄▄▄   
          ▄  ▄▄ ▄▄▄▄▄  ▄   ▄▄ ▄          
    ▄▄▄    ▄ ▄  ▄▄   ▄▄▄▄▄▄▄ ▄    ▄▄▄    
  ▄▄▄▄▄▄▄   ▄ ▄▄▄ ▄ ▄▄▄▄     ▄  ▄▄▄▄▄▄▄  
    ▄▄▄ ▄▄▄  ▄ ▄ ▄▄ ▄▄▄ ▄  ▄  ▄ ▄  ▄▄▄▄  
   ▄▄ ▄▄▄▄   ▄▄▄  ▄   ▄▄ ▄▄ ▄ ▄ ▄  ▄ ▄   
   ▄ ▄▄▄▄▄▄ ▄▄▄▄ ▄▄ ▄▄▄▄▄  ▄  ▄ ▄ ▄  ▄▄  
  ▄ ▄   ▄▄▄▄  ▄     ▄  ▄  ▄   ▄▄▄  ▄ ▄   
    ▄   ▄▄▄  ▄▄ ▄ ▄ ▄ ▄▄▄ ▄ ▄▄▄▄▄▄▄ ▄ ▄  
  ▄     ▄  ▄▄ ▄▄  ▄▄  ▄▄ ▄   ▄  ▄ ▄  ▄   
  ▄▄▄▄▄▄▄ ▄ ▄▄▄ ▄▄  ▄▄▄  ▄▄▄ ▄▄ ▄ ▄  ▄   
   ▄    ▄▄ ▄▄▄▄▄ ▄▄ ▄▄  ▄ ▄  ▄ ▄▄  ▄ ▄   
    ▄▄▄▄▄▄  ▄▄   ▄▄ ▄ ▄▄  ▄   ▄   ▄▄ ▄▄  
     ▄▄ ▄▄▄▄▄▄ ▄  ▄▄  ▄▄ ▄▄     ▄▄▄ ▄▄   
  ▄▄▄▄ ▄▄ ▄ ▄ ▄  ▄▄ ▄ ▄▄ ▄▄ ▄▄ ▄▄▄   ▄▄  
   ▄▄▄▄▄  ▄ ▄▄▄▄ ▄  ▄ ▄ ▄▄      ▄  ▄▄▄▄  
          ▄▄ ▄ ▄  ▄▄▄    ▄▄    ▄  ▄▄ ▄   
    ▄▄▄     ▄ ▄ ▄ ▄▄ ▄ ▄▄ ▄ ▄ ▄ ▄ ▄▄ ▄   
  ▄▄▄▄▄▄▄ ▄▄▄    ▄▄  ▄ ▄ ▄▄  ▄   ▄ ▄▄▄▄  
                                         

https://www.blindupload.org/#4g0JhuB2JRkP6U4hUKI6wo6fRXGLdCuEXyFBNqRBiQR

```

Download and decrypt some files:
```
fadedbee@box:~$ bup https://www.blindupload.org/#4g0JhuB2JRkP6U4hUKI6wo6fRXGLdCuEXyFBNqRBiQR
downloading and decrypting index from https://www.blindupload.org/#4g0JhuB2JRkP6U4hUKI6wo6fRXGLdCuEXyFBNqRBiQR
downloading and decrypting myfile.mp3 into ./jisvwnrzvnpsdugtdc5baz/myfile.mp3
block 0
block 1
block 2
block 3
block 4
downloading and decrypting myotherfile.txt into ./jisvwnrzvnpsdugtdc5baz/myotherfile.txt
block 0
block 1
block 2
block 3
download complete

```