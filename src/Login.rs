use std::net::{TcpStream};
use std::io::{Read, Write};
use std::process;


// TODO probably want to return result here and deal with possible errors

// Function to send a command to the IMAP server
pub fn login(stream: &mut TcpStream, command_id: &mut String, username: &str, password: &str, folder: &str, command_number: &mut u32){
    let mut response = String::new();

    // logging in 
    let full_command = format!("{} LOGIN {} {}", &command_id, &username, &password);
    stream.write_all(full_command.as_bytes());
    stream.write_all(b"\r\n");
    if let Err(_) = stream.read_to_string(&mut response) {
        println!("Login failure\n");
        process::exit(3);
    }
    stream.read_to_string(&mut response);
    println!("Response: {}", response);

    // command of logging is executed, so increment
    *command_number += 1;
    *command_id = format!("A{:03}", *command_number);

    // selecting the folder

    // todo: if no folder is provided, read from inbox
    let full_command = format!("{} SELECT {} ", &command_id, folder);
    response.clear();
    stream.write_all(full_command.as_bytes());
    stream.write_all(b"\r\n");
    if let Err(_) = stream.read_to_string(&mut response) {
        println!("Folder not found\n");
        process::exit(3);
    }
    stream.read_to_string(&mut response);
    println!("Response: {}", response);

    *command_number += 1;
    *command_id = format!("A{:03}", *command_number);

}