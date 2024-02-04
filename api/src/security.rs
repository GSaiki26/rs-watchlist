// Libs
use ring::digest::{digest, SHA512};
use tracing::info;

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
