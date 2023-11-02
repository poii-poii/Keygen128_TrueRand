use getrandom::getrandom; // Get random data from low level source (/dev/urandom) 
use std::fs::File;
use std::io::Write;

fn generate_16_byte_random_key() -> Result<[u8; 16], std::io::Error> {
    let mut key = [0u8; 16];
    getrandom(&mut key);
    Ok(key)
}

fn main() -> Result<(), std::io::Error> {
    let key = generate_16_byte_random_key()?;
    
    let mut file = File::create("key.bin")?;
    file.write_all(&key)?;

    println!("Generated 16-byte truly random key and saved it to 'key.bin'.");

    Ok(())
}

