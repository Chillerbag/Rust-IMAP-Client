
// our function imports 
mod helpers;
mod commands; 
use helpers::exiting::exit_command_line;
use helpers::send_and_receive::{read_response_object, send_command};
use helpers::socket_maker::make_socket;
use helpers::command_executor::execute_command;
use commands::login::login_command;

// Rust std imports
use std::io::BufReader;
use std::env;
use std::net::Shutdown;
use std::process;




/* 
-------------------FETCHMAIL-------------------
The structure of the solution is as follows

1) Commands - This contains all the different commands the spec requires us to send
2) Helpers - these are all functions that allow us to send commands and interpret results
3) helpers/lexicon - where we keep the defined IMAP responses as per rfc 3501 section 9 - we ensure we receive every part of every response
4) helpers/parse - where we parse responses into the lexicon structures.

Program execution:
- We setup the socket and command variables
- we login initially using the function in login.rs
- we execute any subsequent commands

- command execution process:
    - commands create their formatted command 
    - they use send_command in send_and_receive.rs to send that command
    - the response is read into read_response_object
    - the response is converted to its relevant struct / enum in lexicon/rfc3501.rs based on IMAP rfc 3501 section 9
    - the parsing of these responses to be easily read is done parsing/rfc3501.rs based on IMAP rfc 3501 section 9
    - taking the relevant elements from the struct, we print the desired formatted
     response

- justification:
    - the lexicon / parsing mechanism allows us to emulate *exactly* the structs and types used in IMAP responses. This makes our response interpretation infallible.
    - as opposed to string matching a response haphazardly.
    - this does mean we map a significant portion of the rfc3501 into this codebase, including things like NStrings, but this means we can account for every edge case,
    - and thereby facilitate expansion
-----------------------------------------------
*/ 


fn main() {

    /*---- VARIABLE SETUP ---- */
    let args: Vec<String> = env::args().collect(); // get command-line arguments
    let mut username = String::new();
    let mut password = String::new();
    let mut folder = String::new();
    let mut message_num = String::new();
    let mut command: String = String::new();
    let mut server_name: String = String::new();
    let mut arg_check: u32 = 0; 
    let mut command_number: u32 = 1; // for the tag for each command 

    // iterate over args and assign them to their strings using .clone()
    let mut i = 1;
    while i < args.len() {
        if args.len() < 6 {
            exit_command_line();
        }
        // pattern matching
        match args[i].as_str() {
            "-u" => {
                username = args[i+1].clone();
                arg_check += 1; 
                
            }
            "-p" => {
                password = args[i+1].clone();
                arg_check += 1;
            }
            "-f" => {
                folder = args[i+1].clone();
                if folder == "" {
                    exit_command_line();
                }
            }
            "-n" => {
                message_num = args[i+1].clone();
            }
            _ => {
                if args.len() < i + 2 {
                    exit_command_line(); // handle OOB
                }
                command = args[i].clone();
                server_name = args[i + 1].clone();
                arg_check += 2;
                break;
            }
        }
        i += 2;
    }
    // check if enough args were provided
    if arg_check != 4 {
        exit_command_line();
    }

    /*---- MAKE THE SOCKET ---- */
    let mut socket = make_socket(server_name).unwrap_or_else( |e| {
        println!("Failed to connect to server: {}", e);
        process::exit(1)
    });
    
    /*---- LOGGING IN ----*/
    // check if the folder var is empty, and if so, use inbox.
    if folder.is_empty() {
        folder = "INBOX".to_string(); 
    }
    login_command(&mut socket, &username, &password, &mut folder, &mut command_number);

    /*---- ALL OTHER COMMANDS ---- */
    execute_command(&mut socket, &mut message_num, &command, &mut command_number);

    /*---- CLEANING UP ---- */
    let command_id = format!("A{}", command_number);
    command = format!("{} LOGOUT \r\n", command_id);
    let mut reader = BufReader::new(socket.try_clone().expect("error cloning stream"));
    let mut response = String::new();
    send_command(&mut socket, command); // disconnect from IMAP server
    let _ = read_response_object(&mut reader, &mut response, command_id);
    let _ = socket.shutdown(Shutdown::Both);
    

    return ();
}


