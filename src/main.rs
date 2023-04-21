use std::fs;
use std::path::Path;
use std::time::{self, Instant};
extern crate regex;

use regex::Regex;

// Define the regular expression pattern for a credit card number
const CC_NUM_REGEX: &str = r"\b(?:4[0-9]{12}(?:[0-9]{3})?|5[1-5][0-9]{14}|6(?:011|5[0-9][0-9])[0-9]{12}|3[47][0-9]{13}|3(?:0[0-5]|[68][0-9])[0-9]{11}|(?:2131|1800|35\d{3})\d{11})\b";

// Define the Luhn check function for validating credit card numbers
fn luhn_check(cc_num: &str) -> bool {
    // Convert the credit card number to a vector of digits
    let mut digits: Vec<u32> = cc_num.chars().map(|c| c.to_digit(10).unwrap()).collect();

    // Check if the number of digits is valid
    if digits.len() < 13 || digits.len() > 19 {
        return false;
    }

    // Double every other digit, starting from the right
    for i in (0..digits.len()).rev().skip(1).step_by(2) {
        digits[i] *= 2;

        // If the doubled digit is greater than 9, subtract 9 from it
        if digits[i] > 9 {
            digits[i] -= 9;
        }
    }

    // Sum all the digits and check if it is divisible by 10
    let sum: u32 = digits.iter().sum();
    sum % 10 == 0
}

fn not_major_card(card: &str) -> bool{

    if card.starts_with("4") || card.starts_with("3") || card.starts_with("5") {

        match card.as_bytes()[0]{
            51 => print!("Found possible AmEx credit card number: "), 
            52 => print!("Found possible Visa credit card number: "), 
            53 => print!("Found possible MC credit card number: "), 
                _ => (),
        }
        return false;
    }


    return true;
}

// Define a function for recursively searching a folder for credit card numbers
fn search_folder(path: &Path) {
    // Read the contents of the folder
    let entries = fs::read_dir(path).unwrap();

    // Iterate over the entries in the folder
    for entry in entries {
        // Get the path of the current entry
        let entry_path = entry.unwrap().path();

        // Check if the entry is a file
        if entry_path.is_file() {
            // Read the contents of the file as a string
            // Read the contents of the file as a string
            let contents = match fs::read_to_string(entry_path) {
                Ok(s) => s,
                Err(_e) => {
                    // Print an error message and skip the file
                    continue;
                }
            };

            // Use the regular expression to find all credit card numbers in the file
            let cc_nums: Vec<&str> = Regex::new(CC_NUM_REGEX)
                .unwrap()
                .find_iter(&contents)
                .map(|m| m.as_str())
                .collect();

            // Iterate over the found credit card numbers
            for cc_num in cc_nums {
                if not_major_card(cc_num){
                    continue
                }
                // Check if the credit card number passes the Luhn check
                if luhn_check(cc_num) {
                    println!("{}", cc_num);
                }
                



            }
        } else if entry_path.is_dir() {
            // Recursively search the subfolder
            search_folder(&entry_path);
        }
    }
}

fn main() {
    let now = Instant::now();
    // Define the path to the folder to be searched
    let folder_path = Path::new("./");

    // Check if the folder exists
    if !folder_path.exists() || !folder_path.is_dir() {
        println!("Invalid folder path!");
        return;
    }    

    // Start searching the folder
    search_folder(folder_path);
    println!("{}", now.elapsed().as_secs());
}