use crate::commands::{mime::mime_command, parse::parse_command, retrieve::retrieve_command};
use std::net::TcpStream;

pub fn execute_command(stream :&mut TcpStream, message_num: &mut String, command: &str, command_number: &mut u32) {
    match command {
        "retrieve"=>retrieve_command(stream,message_num, command, command_number),
        "parse"=>parse_command(message_num, command, command_number),
        "mime"=>mime_command(message_num, command, command_number),
        "list"=>mime_command(message_num, command, command_number),
        _=>()
    }



}