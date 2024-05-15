use std::net::{ToSocketAddrs, TcpStream};
use std::io::Error;

/*
-------------------MAKE_SOCKET-------------------
takes a server string and combining it with the port defined for use with IMAP, make a socket address
unwrap it from the iterable, and call the connect function to set up connection from the socket
-------------------------------------------------
*/

pub fn make_socket(server_name: String) -> Result<TcpStream, Error> {
    let port = 143;
    // get the ip 
    let mut addrs_iter = format!("{}:{}", server_name, port).to_socket_addrs()?;
    let connect_addr = addrs_iter.next().unwrap();
    // Connect to the address and return the TcpStream
    TcpStream::connect(connect_addr)
}