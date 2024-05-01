use std::net::{TcpStream};
use std::io::{Read, Write};
use std::process;


// TODO probably want to return result here and deal with possible errors

// Function to send a command to the IMAP server
pub fn login(stream: &mut TcpStream, command_id: &mut String, username: &str, password: &str, folder: &str, command_number: &mut u32){
    let mut response = String::new();

    // logging in 
    let full_command = format!("{} LOGIN {} {} \r\n", &command_id, &username, &password);

    // testing
    println!("{}", full_command);
    match stream.write(full_command.as_bytes()) {
        Ok(_) => {},
        Err(err) => {
            eprintln!("Error writing to stream: {}", err);
            process::exit(1);
        }
    }
    // prints up to here:
    println!("successfully written");

    // hangs here:
    if let Err(_) = stream.read_to_string(&mut response) {
        println!("Login failure\n");
        process::exit(3);
    }

    println!("successfully read to response");
    //stream.read_to_string(&mut response);
    println!("Response: {}", response);

    // command of logging is executed, so increment
    *command_number += 1;
    *command_id = format!("A{}", *command_number);

    // selecting the folder

    // todo: if no folder is provided, read from inbox
    let full_command = format!("{} SELECT {} \r\n", &command_id, folder);
    response.clear();
    match stream.write_all(full_command.as_bytes()) {
        Ok(_) => {},
        Err(err) => {
            eprintln!("Error writing to stream: {}", err);
            process::exit(1);
        }
    }
    //stream.read_to_string(&mut response);
    println!("Response: {}", response);

    *command_number += 1;
    *command_id = format!("A{}", *command_number);

}