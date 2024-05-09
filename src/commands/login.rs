use std::net::TcpStream;
use std::io::BufReader;
use super::send_and_receive::read_response;
use super::send_and_receive::send_command;
use std::process;



pub fn login_command(stream: &mut TcpStream, username: &str, password: &str, folder: &str, command_number: &mut u32){

    let mut reader = BufReader::new(stream.try_clone().expect("error cloning stream"));
    let mut response = String::new();

    // -------------------------------- logging in --------------------------------

    // read the initial response to the connection
    read_response(& mut reader, &mut response, "*".to_string());
    response.clear();

    let command_id = format!("A{}", *command_number);
    // Write login command to server
    let full_command = format!("{} LOGIN {} {} \r\n", command_id, &username, &password);
    send_command(stream, full_command);

    // Read server response until end of line
    read_response(& mut reader, &mut response, command_id.clone());

    // check if login is invalid 
    let err_no_folder: String = format!("{} NO", command_id);
    if response.starts_with(&err_no_folder) {
        println!("Login failure\n");
        process::exit(3);
    }
    response.clear();

    // command of logging is executed, so increment
    *command_number += 1;
    let command_id_2 = format!("A{}", *command_number);

    // ------------------------- selecting the folder ----------------------------


    // write select folder command to server
    let full_command = format!("{} SELECT {} \r\n", command_id_2, folder);
    send_command(stream, full_command);

    // Read server response to selecting folder
    read_response(& mut reader, &mut response, command_id_2.clone());

    // check if folder doesn't exist
    let err_no_folder: String = format!("{} NO", command_id_2);
    if response.starts_with(&err_no_folder) {
        println!("Folder not found\n");
        process::exit(3);
        
    }
    response.clear();

    *command_number += 1;

    // ---------------------------------------------------------------------------

}