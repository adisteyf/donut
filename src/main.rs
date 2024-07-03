use std::fs;
use std::fs::File;
use std::io;
use std::io::{Read, Write};
use std::process::Command;

fn vernam_cipher(key: &[u8], data: &[u8]) -> Vec<u8> {
    key.iter()
        .zip(data.iter())
        .map(|(key_byte, data_byte)| key_byte ^ data_byte)
        .collect()
}

fn main() {
    let mut input = String::new();
    println!("type password:");

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    let key = input.as_bytes();
    let input_file_path = "test.jar";
    let output_file_path = ".running.jar";

    let mut input_file = File::open(input_file_path).expect("Unable to open input file");
    let mut input_data = Vec::new();
    input_file
        .read_to_end(&mut input_data)
        .expect("Unable to read input file");

    let encrypted_data = vernam_cipher(key, &input_data);

    let mut output_file = File::create(output_file_path).expect("Unable to create output file");
    output_file
        .write_all(&encrypted_data)
        .expect("Unable to write to output file");

    let _output = Command::new("java")
        .arg("-jar")
        .arg(output_file_path)
        .output()
        .expect("Failed to execute command");

    let _ = fs::remove_file(output_file_path);

    println!("File encrypted successfully!");
}
