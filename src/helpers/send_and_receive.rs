
// our function imports
use crate::helpers::parsing::rfc3501::parse_response;
use exiting::exit_server_response;
use super::{exiting, lexicon::rfc3501::Response};

// rust std imports
use std::net::TcpStream;
use std::io::Write;
use std::process;
use std::io::{self, BufRead, BufReader};

/*
-------------------SEND_COMMAND-------------------
write to the TCPStream returned in socket_maker.rs 
returns nothing.

https://stackoverflow.com/questions/30552187/reading-from-a-tcpstream-with-readread-to-string-hangs-until-the-connection-is
-------------------------------------------------
*/
pub fn send_command(stream: &mut TcpStream, command: String) {

    match stream.write_all(command.as_bytes()) {
        Ok(_) => (),
        Err(err) => {
            eprintln!("Error writing to stream (this should never happen): {}", err);
            process::exit(1);
        }
    }

}

/*
-------------------READ_COMMAND-------------------
create a buffered reader outside this function
then iterate over the response and read each line. 
if its empty, exit and error. 
at the end, parse the response into the relevant structure per IMAP protocol!
-------------------------------------------------
*/
// Adjusted function to use the correct reader
pub fn read_response_object(
    stream: TcpStream,  
    buffer: &mut String,
    command_id: &str,
) -> Result<Response, String> {
    // Create BufReader with UTF-8 replacement
    let mut reader = utf8_replacement_buf_reader(stream);

    let mut tag = "";
    let mut line_buffer = String::new();

    // while not at end
    while command_id != tag {
        // reset the line buffer to read next line
        line_buffer.clear();
        match reader.read_line(&mut line_buffer) {
            Ok(_) if line_buffer.is_empty() => {
                exit_server_response();
            }
            Ok(_) => {}
            Err(err) => {
                eprintln!("Error reading from stream: {}", err);
                process::exit(5);
            }
        }

        // read a response line
        (tag, _) = line_buffer.split_once(" ").unwrap_or(("", line_buffer.as_str()));
        buffer.push_str(&line_buffer);
        if line_buffer.starts_with("* BYE") {
            break;
        }
    }
    // parse
    eprintln!("{}", buffer);
    parse_response(buffer.to_string())
}
// Wrapper to replace invalid UTF-8 sequences with the replacement character
pub struct Utf8ReplacementReader<R: io::Read>(R);

impl<R: io::Read> Utf8ReplacementReader<R> {
    fn new(inner: R) -> Self {
        Utf8ReplacementReader(inner)
    }
}

impl<R: io::Read> io::Read for Utf8ReplacementReader<R> {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        let bytes_read = self.0.read(buf)?;
        for byte in &mut buf[..bytes_read] {
            if !byte.is_ascii() && !byte.is_ascii_control() {
                *byte = 0xEF; // UTF-8 replacement character (U+FFFD)
            }
        }
        Ok(bytes_read)
    }
}

// Convenience function to create BufReader with UTF-8 replacement
pub fn utf8_replacement_buf_reader(stream: TcpStream) -> BufReader<Utf8ReplacementReader<TcpStream>> {
    BufReader::new(Utf8ReplacementReader::new(stream))
}