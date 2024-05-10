
use std::net::TcpStream;
use std::io::{Result,Write};
use std::process;
use std::io::{BufRead, BufReader};
use std::str;


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

// this expects that you define a bufReader outside it. not sure we can do it any other way, not sure redefining the bufReader every time is ok.

pub fn read_response(reader: &mut BufReader<TcpStream>, buffer: &mut String, command_id: String) {
    let mut tag = "";
    let mut line_buffer = String::new();
   // println!("{}", line_buffer);
    while (command_id != tag) {
        //println!("{}", line_buffer);
        line_buffer.clear();
        match reader.read_line(&mut line_buffer) {
            // probably can do if let Err(err) to avoid checking nothing with Ok(_) which is ugly
            Ok(_) => (),
            Err(err) => {
                eprintln!("Error reading from stream (this should never happen): {}", err);
                process::exit(5);
            }
        }
        
    
        (tag,_) = line_buffer.split_once(" ").unwrap_or(("",line_buffer.as_str()));
        buffer.push_str(&line_buffer);
    }

}
