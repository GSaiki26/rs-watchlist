// Libs
use regex::Regex;

// Functions
/**
 * A method to check if the field is valid.
*/
pub fn is_field_valid(field: &str) -> bool {
    Regex::new("^[a-zA-Z0-9!@#$%&*_\\-+.,<>;\\/? ]{3,20}$")
        .unwrap()
        .is_match(field)
}
