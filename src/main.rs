mod helpers;
mod commands; 
use std::io::{BufRead, BufReader};
use std::env;
use std::net::{TcpStream, Shutdown};

use helpers::socket_maker::make_socket;
use helpers::command_executor::execute_command;
use commands::login::login_command;
use helpers::exiting::exit_command_line;
use helpers::send_and_receive::send_command;
use helpers::send_and_receive::read_response;
use std::process;
use std::io::Result;




// --------------- ALL TODOS -----------------

// 2) prioritise if we are an ipv6. Dunno how, but we pass the test case for this
// 4) handle the Err case of Result<> in main, probably. ( I haven't a clue how to do this)

// -------------------------------------------

fn main() {
    // get command-line arguments
    let args: Vec<String> = env::args().collect();

    // these need to be mutable for some reason
    let mut username = String::new();
    let mut password = String::new();
    let mut folder = String::new();
    let mut message_num = String::new();
    let mut command: String = String::new();
    let mut server_name: String = String::new();

    let mut arg_check: u32 = 0; 

    // for the tag for each command 
    let mut command_number: u32 = 1;

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

    // get the socket 
    let mut socket = make_socket(server_name).unwrap_or_else( |e| {
        println!("Failed to connect to server: {}", e);
        process::exit(1)
    });
    
    // then login 

    // check if the folder var is empty, and if so, use inbox.
    if folder.is_empty() {
        folder = "INBOX".to_string(); 
    }
    
    login_command(&mut socket, &username, &password, &mut folder, &mut command_number);


    // then we send commands passed in the command line HERE and have some function to handle the output
    execute_command(&mut socket, &mut message_num, &command, &mut command_number);
    let command_id = format!("A{}", command_number);
    command = format!("{} LOGOUT \r\n", command_id);
    let mut reader = BufReader::new(socket.try_clone().expect("error cloning stream"));
    let mut response = String::new();
    // disconnect from IMAP server
    send_command(&mut socket, command);
    read_response(&mut reader, &mut response, command_id);
    socket.shutdown(Shutdown::Both);
    

    return ();
}


