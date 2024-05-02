use std::env;
use std::net::{TcpStream};
use crate::socketMaker::make_socket;
use crate::Login::login;
mod socketMaker;
mod Login; 
mod sendAndReceive;

// --------------- ALL TODOS -----------------

// 1) make a file for sending commands to IMAP server. This is a general purpose thing so we can avoid code reuse -- done!
// 2) prioritise if we are an ipv6. Dunno how
// 3) when folder is undefined, read from "Inbox" folder
// 4) handle the Err case of Result<> in main, probably. ( I haven't a fucking clue how to do this)
// 5) part of 4, but error with code 3 when certain things dont exist in login (read spec). READ ed response to this https://edstem.org/au/courses/15616/discussion/1944353

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

    // for the tag for each command 
    let mut command_id: String; 
    let mut command_number: u32 = 1;
    command_id = format!("A{}", command_number);

    // iterate over args and assign them to their strings using .clone()
    let mut i = 1;
    while i < args.len() {
        // pattern matching
        match args[i].as_str() {
            "-u" => {
                i += 1;
                username = args[i].clone();
            }
            "-p" => {
                i += 1;
                password = args[i].clone();
            }
            "-f" => {
                i += 1;
                folder = args[i].clone();
            }
            "-n" => {
                i += 1;
                message_num = args[i].clone();
            }
            _ => {
                command = args[i].clone();
                server_name = args[i + 1].clone();
                break;
            }
        }
        i += 1;
    }

    // for testing
    println!("Username: {}", username);
    println!("Password: {}", password);
    println!("Folder: {}", folder);
    println!("Message Number: {}", message_num);
    println!("Command: {}", command);
    println!("Server Name: {}", server_name);

    // get the socket 
    let mut socket = make_socket(server_name);
    println!("Connection successful!");

    // then login 
    login(&mut socket, &mut command_id, &username, &password, &folder, &mut command_number);


    // then we send commands passed in the command line HERE and have some function to handle the output 



    


}

 