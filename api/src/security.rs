// Libs
use ring::digest::{digest, SHA512};
use tracing::{info, warn};

// Functions
/**
 * A method to get the SHA512 from some data.
*/
pub fn get_sha512(data: &[u8]) -> String {
    // Encrypt the password to SHA512.
    info!("Encrypting the password...");
    let pass = digest(&SHA512, data);
    hex::encode(pass)
}

/**
 * A method to check if some field is valid.
*/
pub fn is_valid_field(field: &str, max_length: u8) -> bool {
    // Check if the username is valid.
    info!("Checking if the field is valid...");
    let expression = format!("^[a-zA-Z0-9!@#$%&*_\\-+.,<>;\\/? ]{{3,{max_length}}}$");
    let re = regex::Regex::new(&expression).unwrap();
    match re.is_match(field) {
        true => {
            info!("The field is valid.");
            true
        }
        false => {
            warn!("The field is invalid.");
            false
        }
    }
}
