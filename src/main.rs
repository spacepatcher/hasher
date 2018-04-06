extern crate walkdir;
extern crate md5;
extern crate sha1;
extern crate sha2;
extern crate digest;

use walkdir::WalkDir;
use std::io;
use std::fs::File;
use std::io::Seek;
use std::io::SeekFrom;
use digest::Digest;
use md5::Md5;
use sha1::Sha1;
use sha2::Sha256;


fn main() {
    let mut user_input = String::new();

//    struct Info {
//        path: ,
//        md5: ,
//        sha1: ,
//        sha256: ,
//    }

    io::stdin().read_line(&mut user_input).expect("Failed");
    
    let root = user_input.trim();
    
    for entry in WalkDir::new(&root) {
        match entry {
            Ok(entry) => {
                if entry.path().is_file() {
                    let mut file = File::open(&entry.path()).expect("File opening failed");
                    
                    let md5_d = Md5::digest_reader(&mut file).expect("MD5 failed");
                    file.seek(SeekFrom::Start(0)).expect("Seek failed");
                    let sha1_d = Sha1::digest_reader(&mut file).expect("SHA1 failed");
                    file.seek(SeekFrom::Start(0)).expect("Seek failed");
                    let sha256_d = Sha256::digest_reader(&mut file).expect("SHA256 failed");
                    file.seek(SeekFrom::Start(0)).expect("Seek failed");

                    println!("{}", entry.path().display());
                    println!("MD5: {:x}", md5_d);
                    println!("SHA1: {:x}", sha1_d);
                    println!("SHA256: {:x}", sha256_d);
                }
            },
            Err(err) => {
                if let Some(inner) = err.io_error() {
                    match inner.kind() {
                        io::ErrorKind::InvalidData => {
                            println!("Entry contains invalid data: {}", inner)
                        }
                        io::ErrorKind::PermissionDenied => {
                            println!("Missing permission to read entry: {}", inner)
                        }
                        _ => {
                            println!("Unexpected error occurred: {}", inner)
                        }
                    }
                }    
            }
        }
    }
}
