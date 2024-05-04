use crate::commands::{mime::mime_command, parse::parse_command, retrieve::retrieve_command};

pub fn execute_command(command_id: &mut String, message_num: &mut String, command: &mut String, command_number: &mut u32) {

    let command_val:&str = &command;

    match command_val {
        "retrieve"=>retrieve_command(command_id, message_num, command, command_number),
        "parse"=>parse_command(command_id, message_num, command, command_number),
        "mime"=>mime_command(command_id, message_num, command, command_number),
        "list"=>mime_command(command_id, message_num, command, command_number),
        _=>()
    }



}