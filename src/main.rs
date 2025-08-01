use getrandom::getrandom;
use std::fs::File;
use std::io::Error;
use std::io::{self, Write};

fn generate_random_key() -> Result<Vec<u8>, Error> {
    print!("What is the key length? ");
    io::stdout().flush()?; // Make sure the prompt is shown

    let mut input = String::new();
    io::stdin().read_line(&mut input)?;

    let length: usize = match input.trim().parse() {
        Ok(n) => n,
        Err(_) => {
            eprintln!("Enter a valid number.");
            return Err(Error::new(io::ErrorKind::InvalidInput, "Invalid number"));
        }
    };

    println!("Generating a {}-byte key", length);

    let mut key = vec![0u8; length];
    getrandom(&mut key);

    Ok(key)
}

fn main() -> Result<(), Error> {
    let key = generate_random_key()?;

    let mut file = File::create("key.bin")?;
    file.write_all(&key)?;
    println!("Generated key: key.bin");

    Ok(())
}
