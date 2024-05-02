use std::net::{TcpStream};
use std::io::{Read, Write};
use std::process;
use std::io::{BufRead, BufReader};


// TODO probably want to return result here and deal with possible errors
// actually, spec says to do this in mean
// TODO put in general stream writing and reading behaviour in a seperate function


pub fn login(stream: &mut TcpStream, command_id: &mut String, username: &str, password: &str, folder: &str, command_number: &mut u32){
    let mut response = String::new();

    // -------------------------------- logging in --------------------------------

    let full_command = format!("{} LOGIN {} {} \r\n", &command_id, &username, &password);
    println!("command being writen: {}", full_command);

    let mut reader = BufReader::new(stream.try_clone().expect("error cloning stream"));
    match reader.read_line(&mut response) {
        Ok(_) => println!("Server response to login: {}", response),
        Err(err) => {
            eprintln!("Error reading from stream: {}", err);
            process::exit(1);
        }
    }

    response.clear();

    // Write command to server
    match stream.write_all(full_command.as_bytes()) {
        Ok(_) => println!("Successfully written login command"),
        Err(err) => {
            eprintln!("Error writing to stream: {}", err);
            process::exit(1);
        }
    }

    // clear response or it cries idk
    response.clear();

    // Read server response until end of line
    //let mut reader = BufReader::new(stream.try_clone().expect("error cloning stream"));
    match reader.read_line(&mut response) {
        Ok(_) => println!("Server response to login: {}", response),
        Err(err) => {
            eprintln!("Error reading from stream: {}", err);
            process::exit(1);
        }
    }

    response.clear();

    // command of logging is executed, so increment
    *command_number += 1;
    *command_id = format!("A{}", *command_number);
    println!("{}", command_id);

    // ---------------------------------------------------------------------------


    

    // ------------------------- selecting the folder ----------------------------

    // TODO: if no folder is provided, read from inbox

    let full_command = format!("\n{} SELECT {} \r\n", command_id, folder);
    println!("command being written: {}", full_command);

    match stream.write_all(full_command.as_bytes()) {
        Ok(_) => println!("Successfully written select folder command"),
        Err(err) => {
            eprintln!("Error writing to stream: {}", err);
            process::exit(1);
        }
    }

    //response.clear();

    // Read server response until end of line
    //let mut reader = BufReader::new(stream.try_clone().expect("error cloning stream"));
    match reader.read_line(&mut response) {
        Ok(_) => println!("Server response to folder selection: {}", response),
        Err(err) => {
            eprintln!("Error reading from stream: {}", err);
            process::exit(1);
        }
    }

    *command_number += 1;
    *command_id = format!("A{}", *command_number);

    // ---------------------------------------------------------------------------

}