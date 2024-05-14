use std::net::TcpStream;
use std::io::BufReader;
use crate::commands::send_and_receive::*;
use crate::helpers::lexicon::rfc3501::*;
use crate::helpers::exiting::*;


pub fn list_command(stream: &mut TcpStream, command_number: &mut u32) {
    eprintln!("List command");
    let command_id = format!("A{}", *command_number);
    let full_command = format!("{} FETCH 1:* ENVELOPE\r\n", command_id);
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
    let mut mail_id:u32 = 1;
    for resp_component in response_components {
        if let ResponseComponent::ResponseData(ResponseData::MessageData(MessageData { message_data_component: MessageDataComponent::Fetch(msg_att_components), .. })) = resp_component {
            if let Some(MsgAttComponent::MsgAttStatic(MsgAttStatic::Envelope(Envelope { env_subject, .. }))) = msg_att_components.get(0) {
                if let Some(subject) = env_subject.as_ref() {
                    println!("{}: {}", mail_id, subject);
                    mail_id += 1;
                } else {
                    println!("{}: <No subject>", mail_id);
                    mail_id += 1;
                }
            } 
            else {
                exit_parsing_with("Failed to parse envelope".to_string());
            }
        }
    }

}