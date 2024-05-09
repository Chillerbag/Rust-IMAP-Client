use std::net::TcpStream;
use std::io::BufReader;
use std::process;
use crate::commands::send_and_receive::*;

pub fn retrieve_command(stream: &mut TcpStream, message_num: &mut String, command: &str, command_number: &mut u32) {
    eprintln!("Retrieve command");
    let mut response = String::new();
    let mut reader = BufReader::new(stream.try_clone().expect("error cloning stream"));
   

    // -------------------------------- logging in --------------------------------


    let command_id = format!("A{}", *command_number);
    let full_command = format!("{} FETCH {} BODY.PEEK[] \r\n", command_id, &message_num);
    send_command(stream, full_command);

    // Read server response until end of line
    read_response(&mut reader, &mut response, command_id.clone());

    
    let body = read_next_response_data(& response,&command_id);



    print!("{}",body);

        
    // command of logging is executed, so increment
    *command_number += 1;



}

fn read_next_response_data<'a>(response: &'a str,command_id :&str) -> &'a str {
    //process the first line and then do the rest
    let Some((first_line,rest)) = response.split_once("\r\n") else {exit_parsing()};
    // we do assume message data here...
    let Some(("*",message_data)) = first_line.split_once(" ") else {exit_parsing()};
    let Some((_nz_number,message_data_rest)) = message_data.split_once(" ") else {exit_parsing()};
    let Some(("FETCH",msg_att)) = message_data_rest.split_once(" ") else {exit_parsing()};
    let Some(("(BODY[] ",number_almost)) = msg_att.split_once("{") else {exit_parsing()};
    let Some((number,_)) = number_almost.split_once("}") else {exit_parsing()};
    let length_of_body = number.parse::<usize>().unwrap_or_else(|e| exit_parsing());
    let (body,tail) = rest.split_at(length_of_body);

    let Some((")", final_line)) = tail.split_once("\r\n") else {exit_parsing()};

    is_response_done(final_line,command_id);

    body
    


    
}

fn is_response_done(final_line : &str,command_id :&str) {
    
    
    let Some((read_command_id, final_line)) = final_line.split_once(" ") else {
        println!("No command id string found for fetch\n");
        process::exit(3)
    };
    if read_command_id != command_id {
        println!("Read incorrect command id for fetch {} != {}\n",read_command_id,command_id);
        process::exit(3)

    }
    let Some((result,_final_line)) = final_line.split_once(" ") else {
        println!("No result string found for fetch\n");
        process::exit(3)
    };
    match result {
        "OK" => {}
        "NO" => {
            println!("fetch error: can't fetch that data\n");
            process::exit(5);
        }
        "BAD" => {
            println!("command unknown or arguments invalid\n");
            process::exit(5);
        }
        _ => {
            println!("invalid result response\n");
            process::exit(3);
        }
    }
}

fn exit_command_line() -> ! {
    println!("Commandline input failure\n");
    process::exit(1)
}
fn exit_connection() -> ! {

    println!("Connection failure\n");
    process::exit(2)
}
fn exit_server_response() -> ! {

    println!("Server response failure\n");
    process::exit(3)
}
fn exit_parsing() -> ! {
    println!("Parsing failure in server response\n");
    process::exit(4)
}
fn exit_other(error :String) -> ! {

    println!("{}",error);
    process::exit(5)
}