use std::env;
use std::net::{TcpStream};
use crate::socketMaker::make_socket;
mod socketMaker; 

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
    let socket = make_socket(server_name);

    // then login 


}

 