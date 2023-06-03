use base64::{Engine as _, engine::general_purpose};
use sha256::digest;

fn main() {
    let original = "FIND_THIS_STRING";
    let hash = bob_the_builder(original);
    let mut number_string = String::new();
    for i in hash.iter() {
        number_string.push_str(&i.to_string());
    }
    let final_number_string = (number_string.parse::<u128>().unwrap()/8).to_string();
    let mut final_hash = general_purpose::STANDARD_NO_PAD.encode(final_number_string);
    let salt = digest("BCklP09120Dgg56");
    let mut modified_salt = String::new();
    for char in salt.chars() {
        let modified_char = match char {
            'A'..='M' | 'a'..='m' => (char as u8 + 7) as char,
            'N'..='Z' | 'n'..='z' => (char as u8 - 7) as char,
            _ => char,
        };

        modified_salt.push(modified_char);
    }
    final_hash.insert_str(5, &modified_salt);
    final_hash.insert_str(final_hash.len()-3, &salt);
    println!("{}", final_hash);

}

fn bob_the_builder (word: &str) -> Vec<u8>{
    let mut output_vector = Vec::new();
    for char in word.chars(){
        output_vector.push(char as u8 + 3);
    }
    output_vector

}