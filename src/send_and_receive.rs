
use std::net::TcpStream;
use std::io::Write;
use std::process;
use std::io::{BufRead, BufReader};



pub fn send_command(stream: &mut TcpStream, command: String) {

    match stream.write_all(command.as_bytes()) {
        Ok(_) => println!("Successfully written command"),
        Err(err) => {
            eprintln!("Error writing to stream: {}", err);
            process::exit(1);
        }
    }

}

// this expects that you define a bufReader outside it. not sure we can do it any other way, not sure redefining the bufReader every time is ok.

// i have no fucking idea how the &mut works here with buffer, to be honest. It is treated as immutable. What is going on with the declaration. IDK. Will look into tomorrow. 
pub fn read_response(reader: &mut BufReader<TcpStream>, buffer: &mut String) {

    match reader.read_line( buffer) {
        Ok(_) => println!("Server response to command: {}", buffer),
        Err(err) => {
            eprintln!("Error reading from stream: {}", err);
            process::exit(1);
        }
    }

    buffer.clear();

}