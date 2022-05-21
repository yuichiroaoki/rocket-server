use std::error::Error;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;

use rocket::serde::{Deserialize, json::Value};

#[derive(Deserialize, Debug)]
#[serde(crate = "rocket::serde")]
pub struct User {
    fingerprint: String,
    location: String,
    age: u8,
    name: String,
    has_car: bool,
}


pub fn read_user_from_file<P: AsRef<Path>>(path: P) -> Result<User, Box<dyn Error>> {
    // Open the file in read-only mode with buffer.
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    // Read the JSON contents of the file as an instance of `User`.
    let user: User = serde_json::from_reader(reader)?;
    if user.has_car {
        println!("{}", user.name);
    }

    Ok(user)
}

pub fn read_json_from_file<P: AsRef<Path>>(path: P) -> Result<Value, Box<dyn Error>> {
    // Open the file in read-only mode with buffer.
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    // Read the JSON contents of the file as an instance of `User`.
    let u = serde_json::from_reader(reader)?;

    Ok(u)
}
