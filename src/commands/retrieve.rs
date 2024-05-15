// our function imports
use std::net::TcpStream;
use std::io::BufReader;
use crate::commands::send_and_receive::*;
use crate::helpers::exiting::*;
use crate::helpers::lexicon::rfc3501::*;


/*
-------------------RETRIEVE_COMMAND------------------
READ into the body object from do_retrieve_interaction.rs
print the body 
-------------------------------------------------
*/
pub fn retrieve_command(stream: &mut TcpStream, message_num: &mut String, command_number: &mut u32) {
    eprintln!("Retrieve command");
    let body = do_retrieve_interaction(stream,message_num,command_number);
    
    //output body
    print!("{}",body);
}

/*
-------------------RETRIEVE_COMMAND------------------
send the command FETCH (message_num) BODY.PEEK[]
READ into the body object from server
convert the returned struct to a string
-------------------------------------------------
*/
pub(crate) fn do_retrieve_interaction(stream: &mut TcpStream, message_num: &mut String, command_number: &mut u32) -> String {
    let command_id = format!("A{}", *command_number);
    let full_command = format!("{} FETCH {} BODY.PEEK[] \r\n", command_id, &message_num);
    send_command(stream, full_command);
    
    // Read server response
    
    let mut response = String::new();
    let mut reader = BufReader::new(stream.try_clone().expect("error cloning stream"));
    let resp = read_response_object(&mut reader, &mut response, command_id.clone());
    // convert the response to a response_components object
    let Ok(Response {response_components, response_done: ResponseDone::ResponseTagged(resp_tag)}) =  resp else {exit_server_response();};
    match resp_tag {
        // deal with the different responses to a command
        ResponseTagged {resp_cond_state:RespCondState::Ok(_),tag:Tag { chars }} if chars == command_id => {}
        ResponseTagged {resp_cond_state:RespCondState::Ok(_),..} => {exit_server_response_with("Incorrect command id".to_string())}
        ResponseTagged {resp_cond_state:RespCondState::Bad(_),..} => {
            exit_server_response_with("Message not found".to_string());
        }
        // if returning no, realise there is a server error
        ResponseTagged {resp_cond_state:RespCondState::No(_),..} => {
            exit_server_response_with("Server Communication error with sent command".to_string());}
    }
    // if this vec is length zero, nothing was returned
    if response_components.len() <=0 {
        exit_other("No email body found".to_string())
    }

    // get the retrieve response from response component and convert to string
    let Some(ResponseComponent::ResponseData(ResponseData::MessageData(MessageData {message_data_component: MessageDataComponent::Fetch(msg_att_components) ,..}))) = response_components.get(0) else {exit_parsing_with("prea".to_string());};
    let Some(MsgAttComponent::MsgAttStatic(MsgAttStatic::NonStructuredBody(MsgAttStaticBodyNonStructuredComponent {nstring:Some(body),..}))) = msg_att_components.get(0) else {exit_parsing_with("a".to_string());};
    *command_number += 1;
    body.to_string()
}
