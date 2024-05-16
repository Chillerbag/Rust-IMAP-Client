use std::net::TcpStream;
use crate::{commands::retrieve::{do_fetch_interaction, get_body_from_response_components}, helpers::{exiting::*, lexicon::rfc2045::*, parsing::{general::remove_start, rfc2045::get_headers}}};


/*
-------------------MIME_COMMAND------------------
verify the email is a mime message and find it's boundary parameter
read the email body 
extract the first plain text block and print
-------------------------------------------------
*/
pub fn mime_command(stream: &mut TcpStream,message_num: &mut String, command_number: &mut u32) {
    eprintln!("Mime command");
    //start by just verifying that this is a mime message
    let headers_string = get_body_from_response_components(do_fetch_interaction(stream, "BODY.PEEK[HEADER.FIELDS (Content-Type Mime-Version)]", message_num, command_number));
    let (_,headers) = get_headers(headers_string.clone());
    //both of these fields must be present
    let mut boundary_value:Option<String> = None;
    let mut mime_header_flag:bool = false;
    for header in headers {
        match header {
            Field { field_name: FieldName { chars: field_name }, field_body: Some(field_bodies )} 
            if field_name == "content-type" &&  field_bodies_to_vec(field_bodies.clone()).contains(&"multipart/alternative".to_string()) => {
                for content in field_bodies_to_vec(field_bodies.clone()) {
                    if content.starts_with("boundary=") {
                        let mut rest = remove_start("boundary=", content).unwrap();
                        if rest.starts_with("\"") {
                            rest = rest.split_at(1).1.to_string(); 
                        }
                        let end_chars = "\";";
                        let value = rest.split_at(rest.chars().position(|c| end_chars.contains(c)).unwrap_or(rest.len())).0.to_string();
                        boundary_value = Some(value);
                    }
                }
                
            }

            Field { field_name: FieldName { chars: field_name }, 
                field_body: Some(FieldBody { field_body_contents: FieldBodyContents { ascii: field_body_contents }, next_field_body: None }) 
            } 
            if field_name == "mime-version".to_string() && field_body_contents == "1.0".to_string()=> {
                mime_header_flag = true;
            }
            _ => {}
        }
    }

    if !mime_header_flag || boundary_value == None  {
        exit_parsing_with("Email doesn't contain correct Mime headers".to_string())
    }

    // Mime verified, time to parse the body, first split it up
    let body = get_body_from_response_components(do_fetch_interaction(stream,"BODY.PEEK[]", message_num,command_number));
    let boundary = boundary_value.unwrap();

    let split_delimiter = format!("\r\n--{}\r\n",boundary);
    let end_delimiter = format!("\r\n--{}--",boundary);
    let body = body.split_once(&end_delimiter).unwrap_or((&body,"")).0;
    let blocks = body.split(&split_delimiter);
    // check each block until we find the correct one
    let mut correct_block :Option<String>= None;
    for block in blocks {
        let (rest,block_headers) = get_headers(block.to_string());
        //need both flags to parse
        let mut content_type_match = false;
        let mut content_encoding_match = false;
        let valid_encodings : Vec<String> = vec!["quoted-printable","8bit","7bit"].into_iter().map(|s| s.to_string()).collect();
        for header in block_headers {
            match header {
                Field { field_name: FieldName { chars: field_name }, field_body: Some(field_bodies)}
                if field_name == "content-type" && 
                    field_bodies_to_vec(field_bodies.clone()).contains(&"text/plain".to_string()) && 
                    field_bodies_to_vec(field_bodies.clone()).contains(&"charset=UTF-8".to_string()) 
                => {
                    content_type_match = true;
                }
                
                
                Field { field_name: FieldName { chars: field_name }, field_body: Some(field_bodies)} 
                if field_name == "content-transfer-encoding" && 
                    field_bodies_to_vec(field_bodies.clone()).into_iter().fold(false,|b,body| b|| valid_encodings.contains(&body)) 
                => {
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
    } else {
        exit_parsing_with("No plaintext block found".to_string())
    }
}

//function to parse each field body, also unfolds cases with ";"
fn field_bodies_to_vec(field_bodies :FieldBody) -> Vec<String> {
    let mut bodies = Vec::new();
    let mut current_body = Some(Box::new(field_bodies));
    while let Some(body) = current_body {
        let mut contents = (*body).field_body_contents.ascii;
        if contents.ends_with(";") {
            let (s,_) = contents.split_at(contents.len()-1);
            contents = s.to_string();
        }
        if !contents.contains(";") {
            bodies.push(contents);
        }
        else {
            let mut sub_bodies = contents.split("; ");
            while let Some(contents) = sub_bodies.next() {
                bodies.push(contents.to_string());
                
            }
        }
        current_body = (*body).next_field_body;
    }
    bodies
}
