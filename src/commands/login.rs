// our function imports
use crate::helpers::exiting::exit_parsing;
use crate::helpers::sanitisation::sanitise_string_to_literal;
use crate::helpers::send_and_receive::read_response_object;
use super::send_and_receive::send_command;

// rust std imports
use std::net::TcpStream;
use std::io::BufReader;
use std::process;



pub fn login_command(stream: &mut TcpStream, username: &str, password: &str, folder: &str, command_number: &mut u32){

    let mut reader = BufReader::new(stream.try_clone().expect("error cloning stream"));
    let mut response = String::new();

    // -------------------------------- logging in --------------------------------

    // read the initial response to the connection
    let _ = read_response_object(& mut reader, &mut response, "*".to_string());
    response.clear();

    let command_id = format!("A{}", *command_number);
    // Write login command to server
    let full_command = format!("{} LOGIN {} {} \r\n", command_id, sanitise_string_to_literal(&username), sanitise_string_to_literal(&password));
    send_command(stream, full_command);

    // Read server response until end of line
    let _ = read_response_object(& mut reader, &mut response, command_id.clone());
    let Some(response_done) = response.rsplit("\r\n").skip(1).next() else {exit_parsing()};
    // check if login is invalid 
    let err_no_folder: String = format!("{} NO", command_id);
    if response_done.starts_with(&err_no_folder) {
        println!("Login failure");
        process::exit(3);
    }
    response.clear();

    // command of logging is executed, so increment
    *command_number += 1;
    let command_id_2 = format!("A{}", *command_number);

    // ------------------------- selecting the folder ----------------------------
    // write select folder command to server
    let full_command = format!("{} SELECT {}\r\n", command_id_2, sanitise_string_to_literal(folder));
    send_command(stream, full_command);

    // Read server response to selecting folder
    let _ = read_response_object(& mut reader, &mut response, command_id_2.clone());

    // check if folder doesn't exist
    let Some(response_done) = response.rsplit("\r\n").skip(1).next() else {exit_parsing()};
    let err_no_folder: String = format!("{} NO", command_id_2);
    if response_done.starts_with(&err_no_folder) {
        println!("Folder not found");
        process::exit(3);
        
    }
    response.clear();

    *command_number += 1;

    // ---------------------------------------------------------------------------

}
