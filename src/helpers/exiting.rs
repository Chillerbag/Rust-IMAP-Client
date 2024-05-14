use std::process;

pub(crate) fn exit_command_line() -> ! {
    println!("Commandline input failure");
    process::exit(1)
}

pub(crate) fn exit_connection() -> ! {
    println!("Connection failure");
    process::exit(2)
}

pub(crate) fn exit_server_response() -> ! {
    println!("Server response failure");
    process::exit(3)
}

pub(crate) fn exit_server_response_with(error:String) -> ! {
    println!("{}",error);
    process::exit(3)
}

pub(crate) fn exit_parsing() -> ! {
    println!("Parsing failure in server response");
    process::exit(4)
}

pub(crate) fn exit_parsing_with(error :String) -> ! {
    println!("{}",error);
    process::exit(4)
}

pub(crate)fn exit_other(error :String) -> ! {
    println!("{}",error);
    process::exit(5)
}