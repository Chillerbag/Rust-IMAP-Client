// our function imports
use crate::commands::send_and_receive::*;
use crate::helpers::lexicon::rfc3501::*;
use crate::helpers::exiting::*;

// rust std imports
use std::net::TcpStream;
use std::io::BufReader;

/*
-------------------LIST_COMMAND------------------
SEND the command FETCH 1:* ENVELOPE to get all the 
mail in a selected mailboxes header
read the response into a vector of headers
read over all the response components to get all the headers

-------------------------------------------------
*/
pub fn list_command(stream: &mut TcpStream, command_number: &mut u32) {

    /* setting up and sending the command */
    eprintln!("List command");
    let command_id = format!("A{}", *command_number);
    let full_command = format!("{} FETCH 1:* ENVELOPE\r\n", command_id);
    send_command(stream, full_command);
    
    /* defining whats needed to read the buffer then reading */
    let mut response = String::new();
    let resp = read_response_object( stream.try_clone().expect("error cloning stream"), &mut response, &command_id);

    // read into responsecomponents
    let Ok(Response {response_components, response_done: ResponseDone::ResponseTagged(resp_tag)}) =  resp else {exit_server_response();};
    match resp_tag {
        // deal with the different responses to the command
        ResponseTagged {resp_cond_state:RespCondState::Ok(_),tag:Tag { chars }} if chars == command_id => {}
        ResponseTagged {resp_cond_state:RespCondState::Ok(_),..} => {exit_server_response_with("Incorrect command id".to_string())}
        ResponseTagged {resp_cond_state:RespCondState::Bad(_),..} => {
            exit_server_response_with("Message not found".to_string());
        }
        // deal with dead server
        ResponseTagged {resp_cond_state:RespCondState::No(_),..} => {
            exit_server_response_with("Server Communication error with sent command".to_string());}
    }
    // deal with no header
    if response_components.len() <=0 {
        exit_other("No email header".to_string())
    }

    /* convert to envelope struct and iterate over the responseobjects, printing each subject (env_subject) in struct  */
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