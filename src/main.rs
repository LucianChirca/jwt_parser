use clap::Parser;
use base64::{Engine as _, engine::{general_purpose}};
use std::str;
use serde_json::Value;

#[derive(Parser)]
struct Cli {
    token: String,
}

fn main() {
    let args = Cli::parse();
    let token_parts: Vec<&str> = args.token.split(".").collect();

    for part in &token_parts[0..2] {
        let fixed_part = fix_base64_padding(part.to_string());
        
        match general_purpose::STANDARD.decode(fixed_part) {
            Ok(decoded_bytes) => {
                let decoded_part = str::from_utf8(&decoded_bytes).unwrap();
                let json: Value = serde_json::from_str(decoded_part).unwrap();

                println!("{}", serde_json::to_string_pretty(&json).unwrap());
            }
            Err(e) => eprintln!("Error decoding Base64 string: {}, {}", e, part),
        };
    }
}

fn fix_base64_padding(mut base64_string: String) -> String {
    while base64_string.len() % 4 != 0 {
        base64_string.push('=');
    }
    base64_string
}