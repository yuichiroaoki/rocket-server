use rocket::serde::Deserialize;
use std::error::Error;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;

#[derive(Deserialize, Debug)]
#[serde(crate = "rocket::serde")]
pub struct User {
    fingerprint: String,
    location: String,
}

// pub struct Sample {
//     success: bool,
//     error: String,
//     result: Pointers
// }

// pub struct Pointers {
//     pointers: vec![Pointer]
// }


// pub struct Pointer { 
//     elements: Vec<String>,
//     description: String,
//     markdown: String,
// }


pub fn read_user_from_file<P: AsRef<Path>>(path: P) -> Result<User, Box<dyn Error>> {
    // Open the file in read-only mode with buffer.
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    // Read the JSON contents of the file as an instance of `User`.
    let u = serde_json::from_reader(reader)?;

    // Return the `User`.
    Ok(u)
}
