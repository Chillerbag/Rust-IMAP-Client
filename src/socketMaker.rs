use std::net::{ToSocketAddrs, TcpStream};

pub fn make_socket(server_name: String) -> TcpStream {
    let port = 143;

    // get the ip (probably need to get rid of unwrap here)
    let mut addrs_iter = format!("{}:{}", server_name, port).to_socket_addrs().unwrap();

    // TODO: iterate and check if ipv6 or ipv4, not just taking first
    let connect_addr = addrs_iter.next().unwrap();

    // Connect to the address and return the TcpStream
    TcpStream::connect(connect_addr).unwrap()

    
}