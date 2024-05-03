use std::net::TcpStream;
use std::io::BufReader;
use crate::send_and_receive::read_response;
use crate::send_and_receive::send_command;




pub fn login(stream: &mut TcpStream, command_id: &mut String, username: &str, password: &str, folder: &str, command_number: &mut u32){

    let mut reader = BufReader::new(stream.try_clone().expect("error cloning stream"));
    let mut response = String::new();

    // -------------------------------- logging in --------------------------------

    // read the initial response to the connection
    read_response(& mut reader, &mut response);
    response.clear();

    // Write login command to server
    let full_command = format!("{} LOGIN {} {} \r\n", &command_id, &username, &password);
    send_command(stream, full_command);

    // Read server response until end of line
    read_response(& mut reader, &mut response);
    response.clear();

    // command of logging is executed, so increment
    *command_number += 1;
    *command_id = format!("A{}", *command_number);

    // ------------------------- selecting the folder ----------------------------

    // write select folder command to server
    let full_command = format!("{} SELECT {} \r\n", command_id, folder);
    send_command(stream, full_command);

    // Read server response to selecting folder
    read_response(& mut reader, &mut response);
    response.clear();

    *command_number += 1;
    *command_id = format!("A{}", *command_number);

    // ---------------------------------------------------------------------------

}