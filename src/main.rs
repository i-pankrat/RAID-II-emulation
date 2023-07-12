pub mod hamming_encoding;
pub mod raid;

use raid::{FileType, RaidII};
use std::io;

fn main() {
    help();
    invite_to_enter_command();
    let mut raid = RaidII::from_data_capacity(1024);

    loop {
        let mut user_input = String::new();
        let stdin = io::stdin();

        match stdin.read_line(&mut user_input) {
            Ok(_) => {
                let tokens: Vec<&str> = user_input.split_whitespace().collect();

                match tokens.first() {
                    Some(string) => match *string {
                        "write" => {
                            if tokens.len() != 3 {
                                say_error();
                                invite_to_enter_command();
                                continue;
                            }

                            let data = tokens[1].as_bytes().to_vec();
                            let name = tokens[2].to_owned();
                            let result = raid.write_file(&data, &FileType::Text, &name);

                            match result {
                                raid::FileWriteResult::Success => println!(
                                    "File {} with content '{}' has been written!",
                                    name, tokens[1]
                                ),
                                raid::FileWriteResult::NotEnoughSpace => {
                                    println!("Not enough space to store data!")
                                }
                            }
                        }
                        "read" => {
                            if tokens.len() != 2 {
                                say_error();
                                invite_to_enter_command();
                                continue;
                            }

                            let name = tokens[1].to_owned();
                            let result = raid.read_file(&name);

                            match result {
                                raid::FileReadResult::NotFound => println!("File {} does not exist", name),
                                raid::FileReadResult::DisksCorrupted => println!(
                                    "All data is corrupted. Failed to complete you request!"
                                ),
                                raid::FileReadResult::Success(file_type, byte_data) => {
                                    match file_type {
                                        FileType::Text => match String::from_utf8(byte_data) {
                                            Ok(content_string) => {
                                                println!(
                                                    "File: {}\nContent: {}",
                                                    name, content_string
                                                );
                                            }
                                            Err(_) => unreachable!(
                                                "All data stored at RAID has UTF8 format"
                                            ),
                                        },
                                    }
                                }
                            }
                        }
                        "exit" => {
                            break;
                        }
                        _ => {
                            say_error();
                            invite_to_enter_command();
                            continue;
                        }
                    },
                    None => {
                        say_error();
                        invite_to_enter_command();
                        continue;
                    }
                }
            }
            Err(_) => (),
        }
    }
}

fn help() {
    println!(
        "This is a simulation of RAID II operation. Available commands:
        - write str_data file_name
        - read file_name
        - exit"
    );
}

fn invite_to_enter_command() {
    print!("Your command: ");
}

fn say_error() {
    print!("Invalid command. Try again, please!");
}
