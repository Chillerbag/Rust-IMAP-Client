use std::net::TcpStream;
use std::io::BufReader;
use std::process;
use crate::commands::send_and_receive::*;
use crate::helpers::exiting::*;

pub fn retrieve_command(stream: &mut TcpStream, message_num: &mut String, command_number: &mut u32) {
    eprintln!("Retrieve command");
    let command_id = format!("A{}", *command_number);
    let full_command = format!("{} FETCH {} BODY.PEEK[] \r\n", command_id, &message_num);
    send_command(stream, full_command);
    

    // Read server response
    let mut response = String::new();
    let mut reader = BufReader::new(stream.try_clone().expect("error cloning stream"));
    read_response(&mut reader, &mut response, command_id.clone());
    *command_number += 1;

    // get the body 
    let body = read_next_response_data(& response,&command_id);
    print!("{}",body);

}

fn read_next_response_data(response: & str,command_id :&str) -> String {
    let out_body :&mut String = &mut String::new();
    let final_line :&mut String =  &mut response.to_string();
    
    while !is_response_done(final_line.as_str(), command_id, false) {
        //process the first line and then do the rest
        let Some((first_line,rest)) = response.split_once("\r\n") else {exit_parsing()};
        // we do assume message data here...
        let Some(("*",message_data)) = first_line.split_once(" ") else {exit_parsing()};
        let Some((_nz_number,message_data_rest)) = message_data.split_once(" ") else {exit_parsing()};
        let Some(("FETCH",msg_att)) = message_data_rest.split_once(" ") else {exit_parsing()};
        let Some(("(BODY[] ",number_almost)) = msg_att.split_once("{") else {exit_parsing()};
        let Some((number,_)) = number_almost.split_once("}") else {exit_parsing()};
        let length_of_body = number.parse::<usize>().unwrap_or_else(|e| exit_parsing_with(e.to_string()));
        let (body,tail) = rest.split_at(length_of_body);
        *out_body = body.to_string();
    
        let Some((")", next_line)) = tail.split_once("\r\n") else {exit_parsing()};
        *final_line = next_line.to_string();
    
    }
    is_response_done(final_line.as_str(),command_id,true);

    out_body.clone()
    


    
}
fn is_response_done(final_line : &str,command_id :&str, exit_if_not_done: bool) -> bool {
    
    let Some((read_command_id, final_line)) = final_line.split_once(" ") else {
        if !exit_if_not_done {
            return false;
        }
        println!("No command id string found for fetch");
        process::exit(3)
    };
    if read_command_id != command_id {
        if !exit_if_not_done {
            return false;
        }
        println!("Read incorrect command id for fetch {} != {}",read_command_id,command_id);
        process::exit(3)

    }
    let Some((result,_final_line)) = final_line.split_once(" ") else {
        if !exit_if_not_done {
            return false;
        }
        println!("No result string found for fetch");
        process::exit(3)
    };
    match result {
        "OK" => {}
        "NO" => {
            if !exit_if_not_done {
                return true;
            }
            println!("fetch error: can't fetch that data");
            process::exit(5);
        }
        "BAD" => {
            if !exit_if_not_done {
                return true;
            }
            println!("Message not found");
            process::exit(3);
        }
        _ => {
            if !exit_if_not_done {
                return true;
            }
            println!("invalid result response");
            process::exit(3);
        }
    }
    return true;
    
}