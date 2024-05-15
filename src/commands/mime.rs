use std::net::TcpStream;
use crate::{commands::retrieve::do_retrieve_interaction, helpers::{exiting::*, lexicon::{rfc2045::*, rfc3501::*}, parsing::general::{remove_start, DecodeProtocol}, send_and_receive::{read_response_object, send_command}}};
use std::io::BufReader;

pub fn mime_command(stream: &mut TcpStream,message_num: &mut String, command_number: &mut u32) {
    eprintln!("Mime command");

    let command_id = format!("A{}", *command_number);
    let full_command = format!("{} FETCH {} BODY.PEEK[HEADER.FIELDS (Content-Type Mime-Version)] \r\n", command_id, &message_num);
    send_command(stream, full_command);
    
    let mut response = String::new();
    let mut reader = BufReader::new(stream.try_clone().expect("error cloning stream"));
    
    let resp  = read_response_object(&mut reader, &mut response, command_id.clone());
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
    let Some(MsgAttComponent::MsgAttStatic(MsgAttStatic::NonStructuredBody(MsgAttStaticBodyNonStructuredComponent {nstring:Some(headers_string),..}))) = msg_att_components.get(0) else {exit_parsing_with("a".to_string());};
    *command_number += 1;
    let (_,headers) = get_headers(headers_string.clone());
    let mut boundary_value:Option<String> = None;
    let mut mime_header_flag:bool = false;
    for header in headers {
        match header {
            Field { field_name: FieldName { chars: field_name }, 
                field_body: Some(FieldBody { field_body_contents: FieldBodyContents { ascii: field_body_contents }, 
                next_field_body: field_bodies }) 
            } 
            if field_name == "content-type" && field_body_contents == " multipart/alternative;" => {
                let mut current_body = field_bodies;
                while let Some(body) = current_body {
                    let contents = (*body).field_body_contents.ascii;
                    if contents.starts_with("boundary=") {
                        let mut rest = remove_start("boundary=", contents).unwrap();
                        if rest.starts_with("\"") {
                            rest = rest.split_at(1).1.to_string(); 
                        }
                        let end_chars = "\";";
                        let value = rest.split_at(rest.chars().position(|c| end_chars.contains(c)).unwrap_or(rest.len())).0.to_string();
                        boundary_value = Some(value);
                    }
                    current_body = (*body).next_field_body;
                }
            }

            Field { field_name: FieldName { chars: field_name }, 
                field_body: Some(FieldBody { field_body_contents: FieldBodyContents { ascii: field_body_contents }, next_field_body: None }) 
            } 
            if field_name == "mime-version".to_string() && field_body_contents == " 1.0".to_string()=> {
                mime_header_flag = true;
            }
            _ => {}
        }
    }

    if !mime_header_flag || boundary_value == None  {
        exit_parsing_with("Email doesn't contain correct Mime headers".to_string())
    }

    let body = do_retrieve_interaction(stream,message_num,command_number);
    let boundary = boundary_value.unwrap();
    let split_delimiter = format!("\r\n--{}\r\n",boundary);
    let end_delimiter = format!("\r\n--{}--",boundary);
    let body = body.split_once(&end_delimiter).unwrap_or((&body,"")).0;
    let blocks = body.split(&split_delimiter);
    let mut correct_block :Option<String>= None;
    for block in blocks {
        let (rest,block_headers) = get_headers(block.to_string());
        let mut content_type_match = false;
        let mut content_encoding_match = false;
        let valid_encodings : Vec<String> = vec![" quoted-printable"," 8bit"," 7bit"].into_iter().map(|s| s.to_string()).collect();
        for header in block_headers {
            match header {
                Field { field_name: FieldName { chars: field_name }, 
                    field_body: Some(FieldBody { field_body_contents: FieldBodyContents { ascii: field_body_contents }, 
                    next_field_body: field_bodies }) 
                } 
                //TODO:deal with folded headers generally zzzz
                if field_name == "content-type" && field_body_contents == " text/plain;" => {
                    let mut current_body = field_bodies;
                    while let Some(body) = current_body {
                        let contents = (*body).field_body_contents.ascii;
                        if contents == "charset=UTF-8" {
                            content_type_match = true;
                            break;
                        }
                        current_body = (*body).next_field_body;
                    }

                }
                
                Field { field_name: FieldName { chars: field_name }, 
                    field_body: Some(FieldBody { field_body_contents: FieldBodyContents { ascii: field_body_contents }, .. }) 
                } 
                if field_name == "content-type" && field_body_contents == " text/plain; charset=UTF-8" => {
                    content_type_match = true;
                }
                
                Field { field_name: FieldName { chars: field_name }, 
                    field_body: Some(FieldBody { field_body_contents: FieldBodyContents { ascii: field_body_contents }, .. }) 
                } 
                if field_name == "content-transfer-encoding" && valid_encodings.contains(&field_body_contents) => {
                    content_encoding_match = true;
                }
                _ => {}
            }
        }
        if content_type_match && content_encoding_match {
            correct_block = Some(rest);
            break;
        }
    }

    if let Some(s) = correct_block {
        print!("{}",s);
    }
    
}

fn get_headers(string: String) -> (String,Vec<Field>) {
    let mut rest = string;
    let mut headers= Vec::new();
    while Field::can_parse(rest.to_string()) {
        let (rs,part) = Field::parse_new(rest.to_string()).unwrap();
        rest = rs;
        headers.push(part);
    }
    if rest != "" {rest = remove_start("\r\n",rest).unwrap();}
    (rest,headers)
}

