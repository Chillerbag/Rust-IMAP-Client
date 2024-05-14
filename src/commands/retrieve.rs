use std::net::TcpStream;
use std::io::BufReader;
use crate::commands::send_and_receive::*;
use crate::helpers::exiting::*;
use crate::helpers::lexicon::*;

pub fn retrieve_command(stream: &mut TcpStream, message_num: &mut String, command_number: &mut u32) {
    eprintln!("Retrieve command");
    let command_id = format!("A{}", *command_number);
    let full_command = format!("{} FETCH {} BODY.PEEK[] \r\n", command_id, &message_num);
    send_command(stream, full_command);
    
    // Read server response
    
    let mut response = String::new();
    let mut reader = BufReader::new(stream.try_clone().expect("error cloning stream"));
    let resp = read_response_object(&mut reader, &mut response, command_id.clone());
    eprintln!("{:?}",resp);
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
        exit_other("No email body found".to_string())
    }
    let Some(ResponseComponent::ResponseData(ResponseData::MessageData(MessageData {message_data_component: MessageDataComponent::Fetch(msg_att_components) ,..}))) = response_components.get(0) else {exit_parsing_with("prea".to_string());};
    let Some(MsgAttComponent::MsgAttStatic(MsgAttStatic::NonStructuredBody(MsgAttStaticBodyNonStructuredComponent {nstring:Some(body),..}))) = msg_att_components.get(0) else {exit_parsing_with("a".to_string());};
    *command_number += 1;
    //output body
    print!("{}",body);
}