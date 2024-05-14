use std::net::TcpStream;
use crate::helpers::{exiting::*, lexicon::rfc3501::*, send_and_receive::{read_response_object, send_command}};
use std::io::BufReader;

pub fn mime_command(stream: &mut TcpStream,message_num: &mut String, command_number: &mut u32) {
    eprintln!("Mime command");

    let command_id = format!("A{}", *command_number);
    let full_command = format!("{} FETCH {} BODY.PEEK[HEADER.FIELDS (Mime-Version Content-Type)] \r\n", command_id, &message_num);
    send_command(stream, full_command);
    
    let mut response = String::new();
    let mut reader = BufReader::new(stream.try_clone().expect("error cloning stream"));
    
    let resp  = read_response_object(&mut reader, &mut response, command_id.clone());
    eprintln!("{:?}",resp);
    let Ok(Response {response_components, response_done: ResponseDone::ResponseTagged(resp_tag)}) =  resp else {exit_server_response();};
    match resp_tag {
        ResponseTagged {resp_cond_state:RespCondState::Ok(_),tag:Tag { chars }} if chars == command_id => {}
        ResponseTagged {resp_cond_state:RespCondState::Ok(_),..} => {exit_server_response_with("Incorrect command id".to_string())}
        ResponseTagged {resp_cond_state:RespCondState::Bad(_),..} => {
            exit_server_response_with("Message not found".to_string());
        }
        ResponseTagged {resp_cond_state:RespCondState::No(_),..} => {
            exit_server_response_with("Server Communication error with sent command".to_string());}
    }
    if response_components.len() <=0 {
        exit_other("No email body found".to_string())
    }
    
    let Some(ResponseComponent::ResponseData(ResponseData::MessageData(MessageData {message_data_component: MessageDataComponent::Fetch(msg_att_components) ,..}))) = response_components.get(0) else {exit_parsing_with("prea".to_string());};
    let Some(MsgAttComponent::MsgAttStatic(MsgAttStatic::NonStructuredBody(MsgAttStaticBodyNonStructuredComponent {nstring:Some(body),..}))) = msg_att_components.get(0) else {exit_parsing_with("a".to_string());};
    *command_number += 1;
    eprintln!("{}",body);
    // let body = do_retrieve_interaction(stream,message_num,command_number);
    
    // let message = parse_mime_message(body);
    // let rest = body.split_once("
    // Mime-Version: 1.0
    // Content-Type: multipart/alternative;
    //  boundary=\"");
    // rest.skip(1).next();
    //output body
    // print!("{:?}",message);

    // get the body 
    // let boundary = get_boundary(& response, &command_id);
    // let body = read_next_response_data(& response, &command_id);
    // print!("{}",body);
}

