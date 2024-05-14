use std::net::TcpStream;
use std::io::BufReader;
use std::process;
use crate::commands::send_and_receive::*;
use crate::helpers::{exiting::*, lexicon::*};


pub fn parse_command(stream: &mut TcpStream, message_num: &mut String, command_number: &mut u32) {
    eprintln!("Parse command");
    let command_id = format!("A{}", *command_number);
    let full_command = format!("{} FETCH {} ENVELOPE \r\n", command_id, &message_num);
    send_command(stream, full_command);
    
    let mut response = String::new();
    let mut reader = BufReader::new(stream.try_clone().expect("error cloning stream"));
    let resp  = read_response_object(&mut reader, &mut response, command_id.clone());

    let Ok(Response {response_components, response_done: ResponseDone::ResponseTagged(resp_tag)}) =  resp else {exit_server_response();};
    match (resp_tag) {
        ResponseTagged {resp_cond_state:RespCondState::Ok(_),tag:Tag { chars }} if chars == command_id => {}
        ResponseTagged {resp_cond_state:RespCondState::Ok(_),..} => {exit_server_response_with("Incorrect command id".to_string())}
        ResponseTagged {resp_cond_state:RespCondState::Bad(_),..} => {
            exit_server_response_with("Message not found".to_string());
        }
        ResponseTagged {resp_cond_state:RespCondState::No(_),..} => {
            exit_server_response_with("Server Communication error with sent command".to_string());}
    }
    if response_components.len() <=0 {
        exit_other("No email header".to_string())
    }
    let Some(ResponseComponent::ResponseData(ResponseData::MessageData(MessageData {message_data_component: MessageDataComponent::Fetch(msg_att_components) ,..}))) = response_components.get(0) else {exit_parsing_with("prea".to_string());};
    let Some(MsgAttComponent::MsgAttStatic(MsgAttStatic::Envelope(Envelope { env_date, env_subject, env_from, env_to, .. }))) = msg_att_components.get(0) else {exit_parsing_with("a".to_string());};
    *command_number += 1;
    //output envelope
    let addresses_from = &env_from.address;
    let mut formatted_mailbox_name:String = String::new();
    let mut formatted_mailbox:String = String::new();


    for i in addresses_from {
        if (i.addr_name.as_ref().unwrap_or(&"NIL".to_string())) != &"NIL".to_string() {
            formatted_mailbox_name.push_str(i.addr_name.as_ref().unwrap_or(&"NIL".to_string()));
        }
        formatted_mailbox.push_str(i.addr_mailbox.as_ref().unwrap_or(&"NIL".to_string()));
        formatted_mailbox.push_str("@");
        formatted_mailbox.push_str(i.addr_host.as_ref().unwrap_or(&"NIL".to_string()));
    }

    let addresses_to = &env_to.address;
    let mut formatted_mailbox_name_to:String = String::new();
    let mut formatted_mailbox_to:String = String::new();


    for i in addresses_to {
        if (i.addr_name.as_ref().unwrap_or(&"NIL".to_string())) != &"NIL".to_string() {
            formatted_mailbox_name_to.push_str(i.addr_name.as_ref().unwrap_or(&"NIL".to_string()));
        }
        formatted_mailbox_to.push_str(i.addr_mailbox.as_ref().unwrap_or(&"NIL".to_string()));
        formatted_mailbox_to.push_str("@");
        formatted_mailbox_to.push_str(i.addr_host.as_ref().unwrap_or(&"NIL".to_string()));
    }

    eprintln!("{:?}", env_from.address);
    if formatted_mailbox_name != "" {
        eprintln!("From: {:?} {:?}",formatted_mailbox_name, formatted_mailbox);
    }
    else {
        eprintln!("From: {:?}",formatted_mailbox);
    }
    eprintln!("To: {:?}",formatted_mailbox_to);
    eprintln!("Date: {:?}",env_date.as_ref().unwrap_or(&"NIL".to_string()));
    if env_subject.as_ref().unwrap_or(&"NIL".to_string()) != "NIL" {
        eprintln!("Subject: {:?}",env_subject.as_ref().unwrap());
    }
    else {
        eprintln!("Subject: <No subject>")
    }

}