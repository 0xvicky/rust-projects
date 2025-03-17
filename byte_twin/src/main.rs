use crypto_hash::{Algorithm, Hasher};
use hex::encode;
use std::collections::HashMap;
use std::fs;
use std::io::{self, BufReader, Read, Write};
use std::path::{Path, PathBuf};
const DUMMY_DIR: &str = "E:\\Code\\Rust\\Rust-10\\byte_twin\\dummy_dir";

fn hash(filepath: PathBuf) -> io::Result<Vec<u8>> {
    let file = fs::File::open(filepath)?; //fetch the file path and open it
    let mut reader = BufReader::new(file);
    let mut hasher = Hasher::new(Algorithm::SHA256); //initialise the hasher
    let mut buffer = [0; 1024]; //buffer to read the file

    loop {
        let bytes_read = reader.read(&mut buffer)?;
        if bytes_read == 0 {
            break;
        }
        hasher.write(&buffer[..bytes_read])?;
    }
    // println!("{:?}", hasher.finish());
    Ok(hasher.finish().to_vec())
}
fn main() {
    println!("Byte Twin : A duplicate finder");
    let dummy_dir = Path::new(DUMMY_DIR);
    let mut mpp: HashMap<String, Vec<String>> = HashMap::new();
    //Scans the provided directory and returns files names which have same content
    //1. Start looping over entries in the directory
    match fs::read_dir(dummy_dir) {
        Ok(entries) => {
            for entry in entries.flatten() {
                let path = entry.path();
                if let Ok(hash) = hash(path) {
                    let hash_str = encode(hash);
                    let file_name = entry.file_name().to_string_lossy().to_string();
                    if let Some(vec) = mpp.get_mut(&hash_str) {
                        vec.push(file_name);
                    } else {
                        mpp.insert(hash_str, vec![file_name]);
                    }

                    //mpp.entry(hash_str).or_insert_with(Vec::new).push(file_name);
                }
                // println!("{:?}", path);
            }
        }
        Err(e) => {
            println!("Error : {}", e);
        }
    }
    let mut idx = 1;
    let mut duplicates_found = false;
    //iterate over the map
    //print the file groups where vector size is greater than 1
    for (key, value) in &mpp {
        if value.len() > 1 {
            duplicates_found = true;
            println!(
                "\n================Group - {} Start===================\n",
                idx
            );
            for file in value {
                println!("{}", file);
            }

            idx += 1;
        }
    }
    if !duplicates_found {
        println!("No duplicate files found");
    }
}

//is this buffer a vector? -> No it is an array of 1024 bytes, so 1 kb of chunk from file is read at a time
