
use crate::helpers::parsing::rfc3501::parse_response;

use super::{exiting, lexicon::rfc3501::Response};
use std::net::TcpStream;
use std::io::Write;
use std::process;
use exiting::exit_server_response;
use std::io::{BufRead, BufReader};




// thank god for this: https://stackoverflow.com/questions/30552187/reading-from-a-tcpstream-with-readread-to-string-hangs-until-the-connection-is
// have to use read_line not read_to_string

pub fn send_command(stream: &mut TcpStream, command: String) {

    match stream.write_all(command.as_bytes()) {
        Ok(_) => (),
        Err(err) => {
            eprintln!("Error writing to stream (this should never happen): {}", err);
            process::exit(1);
        }
    }

}

pub fn read_response_object(reader: &mut BufReader<TcpStream>, buffer: &mut String, command_id: String) -> Result<Response,String> {
    //TODO:Streamline this function
    let mut tag = "";
    let mut line_buffer = String::new();
    while command_id != tag {
        line_buffer.clear();
        match reader.read_line(&mut line_buffer) {
            // probably can do if let Err(err) to avoid checking nothing with Ok(_) which is ugly
            Ok(_) if line_buffer.is_empty() => 
                {
                    exit_server_response();
                }
            Ok(_) => {}
            Err(err) => {
                eprintln!("Error reading from stream (this should never happen): {}", err);
                process::exit(5);
            }
        }
    
        (tag,_) = line_buffer.split_once(" ").unwrap_or(("",line_buffer.as_str()));
        buffer.push_str(&line_buffer);
        if line_buffer.starts_with("* BYE") {
            break;
        }
        
    }
    eprintln!("{}",buffer.to_string());
    parse_response(buffer.to_string())

}