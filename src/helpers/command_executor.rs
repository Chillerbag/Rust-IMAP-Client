use crate::commands::{mime::mime_command, parse::parse_command, retrieve::retrieve_command, list::list_command};
use std::net::TcpStream;

/*
-------------------EXECUTE_COMMAND---------------
function used in main to generally execute the functions
for each command in the spec. Just means we abstract
the pattern matching for the command part of 
each case.
-------------------------------------------------
*/
pub fn execute_command(stream :&mut TcpStream, message_num: &mut String, command: &str, command_number: &mut u32) {
    match command {
        "retrieve"=>retrieve_command(stream,message_num, command_number),
        "parse"=>parse_command(stream, message_num, command_number),
        "mime"=>mime_command(stream, message_num, command_number),
        "list"=>list_command(stream, command_number),
        _=>()
    }
}

