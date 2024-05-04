
use std::net::TcpStream;
use std::io::Write;
use std::process;
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

// this expects that you define a bufReader outside it. not sure we can do it any other way, not sure redefining the bufReader every time is ok.

pub fn read_response(reader: &mut BufReader<TcpStream>, buffer: &mut String) {

    match reader.read_line( buffer) {

        // probably can do if let Err(err) to avoid checking nothing with Ok(_) which is ugly
        Ok(_) => (),
        Err(err) => {
            eprintln!("Error reading from stream (this should never happen): {}", err);
            process::exit(1);
        }
    }


}