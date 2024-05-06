mod helpers;
mod commands; 

use std::env;

use helpers::socket_maker::make_socket;
use helpers::command_executor::execute_command;
use commands::login::login_command;



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

    // for the tag for each command 
    let mut command_number: u32 = 1;

    // iterate over args and assign them to their strings using .clone()
    let mut i = 1;
    while i < args.len() {
        // pattern matching
        match args[i].as_str() {
            "-u" => {
                username = args[i+1].clone();
            }
            "-p" => {
                password = args[i+1].clone();
            }
            "-f" => {
                folder = args[i+1].clone();
            }
            "-n" => {
                message_num = args[i+1].clone();
            }
            _ => {
                command = args[i].clone();
                server_name = args[i + 1].clone();
                break;
            }
        }
        i += 2;
    }

    // get the socket 
    let mut socket = make_socket(server_name).unwrap();
    
    // then login 

    // check if the folder is empty, and if so, use inbox.
    if folder.is_empty() {
        folder = "INBOX".to_string(); 
    }
    
    login_command(&mut socket, &username, &password, &mut folder, &mut command_number);


    // then we send commands passed in the command line HERE and have some function to handle the output 

    execute_command(&mut message_num, &command, &mut command_number);

    


}

 