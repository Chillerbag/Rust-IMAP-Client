use crate::commands::retrieve::do_fetch_interaction;
// our function imports
use crate::helpers::lexicon::rfc3501::*;
use crate::helpers::exiting::*;

// rust std imports
use std::net::TcpStream;


/*
-------------------PARSE_COMMAND------------------
send the command FETCH (number) ENVELOPE 
read the response into a vector of the header
read the vector and parse the string into the output
-------------------------------------------------
*/
pub fn parse_command(stream: &mut TcpStream, message_num: &mut String, command_number: &mut u32) {
    eprintln!("Parse command");

    /* format and send the command */
    let response_components = do_fetch_interaction(stream, "ENVELOPE", message_num, command_number);
    let Some(ResponseComponent::ResponseData(ResponseData::MessageData(MessageData {message_data_component: MessageDataComponent::Fetch(msg_att_components) ,..}))) = response_components.get(0) else {exit_parsing_with("prea".to_string());};
    let Some(MsgAttComponent::MsgAttStatic(MsgAttStatic::Envelope(Envelope { env_date, env_subject, env_from, env_to, .. }))) = msg_att_components.get(0) else {exit_parsing_with("a".to_string());};
    *command_number += 1;

    /* set up the formatting */
    let addresses_from = &env_from.address;
    let mut formatted_mailbox_name:String = String::new();
    let mut formatted_mailbox:String = String::new();


    /* format the from addresses */
    for i in addresses_from {
        if (i.addr_name.as_ref().unwrap_or(&"NIL".to_string())) != &"NIL".to_string() {
            formatted_mailbox_name.push_str(&format!("\"{}\"", i.addr_name.as_ref().unwrap_or(&"NIL".to_string())));
        }
        formatted_mailbox.push_str(i.addr_mailbox.as_ref().unwrap_or(&"NIL".to_string()));
        formatted_mailbox.push_str("@");
        formatted_mailbox.push_str(i.addr_host.as_ref().unwrap_or(&"NIL".to_string()));
    }

    /* set up formatting for to addresses */
    let addresses_to = &env_to.address;
    let mut formatted_mailbox_name_to:String = String::new();
    let mut formatted_mailbox_to:String = String::new();

    /* format the to addresses */
    for i in addresses_to {
        if (i.addr_name.as_ref().unwrap_or(&"NIL".to_string())) != &"NIL".to_string() {
            formatted_mailbox_name_to.push_str(i.addr_name.as_ref().unwrap_or(&"NIL".to_string()));
        }
        formatted_mailbox_to.push_str(i.addr_mailbox.as_ref().unwrap_or(&"NIL".to_string()));
        formatted_mailbox_to.push_str("@");
        formatted_mailbox_to.push_str(i.addr_host.as_ref().unwrap_or(&"NIL".to_string()));
    }

    /* print as spec specifies */
    if formatted_mailbox_name != "" {
        println!("From: {} <{}>",formatted_mailbox_name, formatted_mailbox);
    }
    else {
        println!("From: {}",formatted_mailbox);
    }
    if addresses_to.len() > 0  && formatted_mailbox_name_to != ""{
        println!("To: {} {}",formatted_mailbox_name_to, formatted_mailbox_to);
    }
    else if addresses_to.len() > 0  && formatted_mailbox_name_to == ""{
        println!("To: {}",formatted_mailbox_to);
    }
    else {
        println!("To:")
    }
    println!("Date: {}",env_date.as_ref().unwrap_or(&"NIL".to_string()));
    if env_subject.as_ref().unwrap_or(&"NIL".to_string()) != "NIL" {
        println!("Subject: {}",env_subject.as_ref().unwrap());
    }
    else {
        println!("Subject: <No subject>")
    }

}