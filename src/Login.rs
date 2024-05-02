use std::net::TcpStream;
use std::io::Write;
use std::process;
use std::io::{BufRead, BufReader};
use crate::sendAndReceive::read_response;
use crate::sendAndReceive::send_command;

// TODO probably want to return result here and deal with possible errors
// actually, spec says to do this in main
// TODO put in general stream writing and reading behaviour in a seperate function -- done!


pub fn login(stream: &mut TcpStream, command_id: &mut String, username: &str, password: &str, folder: &str, command_number: &mut u32){

    let mut reader = BufReader::new(stream.try_clone().expect("error cloning stream"));
    let mut response = String::new();

    // -------------------------------- logging in --------------------------------

    // read the initial response to the connection
    read_response(& mut reader, &mut response);

    // Write login command to server
    let full_command = format!("{} LOGIN {} {} \r\n", &command_id, &username, &password);
    println!("command being writen: {}", full_command);
    send_command(stream, full_command);

    // Read server response until end of line
    read_response(& mut reader, &mut response);

    // command of logging is executed, so increment
    *command_number += 1;
    *command_id = format!("A{}", *command_number);

    // ------------------------- selecting the folder ----------------------------


    // write select folder command to server
    let full_command = format!("{} SELECT {} \r\n", command_id, folder);
    println!("command being written: {}", full_command);
    send_command(stream, full_command);

    // Read server response to selecting folder
    read_response(& mut reader, &mut response);

    *command_number += 1;
    *command_id = format!("A{}", *command_number);

    // ---------------------------------------------------------------------------

}