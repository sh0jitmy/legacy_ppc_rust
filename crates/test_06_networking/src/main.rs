use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream, UdpSocket};
use std::thread;

fn main() {
    println!("========================================");
    println!(" [Rust Test 06] Networking (TCP / UDP)  ");
    println!("========================================");

    // 1. UDP loopback test
    let udp_server = UdpSocket::bind("127.0.0.1:0").expect("Failed to bind UDP server");
    let udp_server_addr = udp_server.local_addr().unwrap();
    let udp_client = UdpSocket::bind("127.0.0.1:0").expect("Failed to bind UDP client");

    let handle = thread::spawn(move || {
        let mut buf = [0u8; 64];
        let (len, _src) = udp_server.recv_from(&mut buf).unwrap();
        assert_eq!(&buf[..len], b"PING_UDP");
        udp_server.send_to(b"PONG_UDP", _src).unwrap();
    });

    udp_client.send_to(b"PING_UDP", udp_server_addr).unwrap();
    let mut recv_buf = [0u8; 64];
    let (rlen, _) = udp_client.recv_from(&mut recv_buf).unwrap();
    assert_eq!(&recv_buf[..rlen], b"PONG_UDP");
    handle.join().unwrap();
    println!("  [OK] UDP loopback ping-pong verified.");

    // 2. TCP loopback test
    let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind TCP listener");
    let server_addr = listener.local_addr().unwrap();

    let server_handle = thread::spawn(move || {
        let (mut stream, _) = listener.accept().unwrap();
        let mut msg = [0u8; 4];
        stream.read_exact(&mut msg).unwrap();
        assert_eq!(&msg, b"PING");
        stream.write_all(b"PONG").unwrap();
    });

    let mut client = TcpStream::connect(server_addr).expect("Failed to connect TCP");
    client.write_all(b"PING").unwrap();
    let mut resp = [0u8; 4];
    client.read_exact(&mut resp).unwrap();
    assert_eq!(&resp, b"PONG");
    server_handle.join().unwrap();
    println!("  [OK] TCP loopback ping-pong verified.");

    println!("  [PASS] Test 06 Networking passed.");
}
