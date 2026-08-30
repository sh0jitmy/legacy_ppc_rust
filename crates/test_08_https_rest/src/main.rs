use openssl::ssl::{SslConnector, SslMethod, SslStream, SslVerifyMode};
use serde::{Deserialize, Serialize};
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::thread;

#[derive(Serialize, Deserialize, Debug, PartialEq)]
struct RestPayload {
    device_id: String,
    temperature_c: f64,
    status: String,
    uptime_sec: u32,
}

fn test_json_serde() {
    println!("  [1/2] Testing JSON serialization & deserialization (serde)...");
    let payload = RestPayload {
        device_id: "P1022-E500V2-NODE01".to_string(),
        temperature_c: 45.25,
        status: "RUNNING".to_string(),
        uptime_sec: 3600,
    };

    let serialized = serde_json::to_string(&payload).expect("Failed to serialize JSON");
    println!("    Serialized JSON ({} bytes): {}", serialized.len(), serialized);

    let deserialized: RestPayload = serde_json::from_str(&serialized).expect("Failed to deserialize JSON");
    assert_eq!(payload, deserialized);
    println!("    [OK] JSON roundtrip verified successfully.");
}

fn test_tls_https_loopback() {
    println!("  [2/2] Testing TLS 1.2/1.3 & HTTPS communication...");

    // Create self-signed TLS acceptor for mock HTTPS server
    let mut acceptor_builder = openssl::ssl::SslAcceptor::mozilla_intermediate(SslMethod::tls()).unwrap();
    let rsa = openssl::rsa::Rsa::generate(2048).unwrap();
    let pkey = openssl::pkey::PKey::from_rsa(rsa).unwrap();

    let mut x509_builder = openssl::x509::X509::builder().unwrap();
    x509_builder.set_version(2).unwrap();
    let name = openssl::x509::X509Name::builder().unwrap().build();
    x509_builder.set_subject_name(&name).unwrap();
    x509_builder.set_issuer_name(&name).unwrap();
    x509_builder.set_pubkey(&pkey).unwrap();
    x509_builder.sign(&pkey, openssl::hash::MessageDigest::sha256()).unwrap();
    let cert = x509_builder.build();

    acceptor_builder.set_private_key(&pkey).unwrap();
    acceptor_builder.set_certificate(&cert).unwrap();
    let acceptor = acceptor_builder.build();

    let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind TLS server");
    let server_addr = listener.local_addr().unwrap();

    // Mock HTTPS Server Thread
    let server_handle = thread::spawn(move || {
        let (stream, _) = listener.accept().unwrap();
        let mut ssl_stream = acceptor.accept(stream).expect("TLS accept failed");
        
        let mut buf = [0u8; 1024];
        let n = ssl_stream.read(&mut buf).unwrap();
        let req_str = String::from_utf8_lossy(&buf[..n]);
        assert!(req_str.starts_with("POST /api/v1/telemetry HTTP/1.1"));

        let body = "{\"code\":200,\"message\":\"Telemetry Recorded\"}";
        let resp = format!(
            "HTTP/1.1 200 OK\r\nContent-Type: application/json\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{}",
            body.len(), body
        );
        ssl_stream.write_all(resp.as_bytes()).unwrap();
        ssl_stream.flush().unwrap();
    });

    // HTTPS Client
    let mut connector_builder = SslConnector::builder(SslMethod::tls()).unwrap();
    // Allow self-signed test cert for loopback verification
    connector_builder.set_verify(SslVerifyMode::NONE);
    let connector = connector_builder.build();

    let tcp_stream = TcpStream::connect(server_addr).expect("TCP connect failed");
    let mut tls_stream: SslStream<TcpStream> = connector.connect("localhost", tcp_stream).expect("TLS connect failed");

    println!("    TLS Handshake established! Protocol version: {}", tls_stream.ssl().version_str());

    let payload = RestPayload {
        device_id: "P1022-E500V2-NODE01".to_string(),
        temperature_c: 48.5,
        status: "TELEMETRY_OK".to_string(),
        uptime_sec: 7200,
    };
    let json_body = serde_json::to_string(&payload).unwrap();
    let http_req = format!(
        "POST /api/v1/telemetry HTTP/1.1\r\nHost: localhost\r\nContent-Type: application/json\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{}",
        json_body.len(), json_body
    );

    tls_stream.write_all(http_req.as_bytes()).unwrap();
    tls_stream.flush().unwrap();

    let mut resp_buf = Vec::new();
    tls_stream.read_to_end(&mut resp_buf).unwrap();
    let resp_str = String::from_utf8_lossy(&resp_buf);
    println!("    HTTPS Response received:\n    {}", resp_str.lines().next().unwrap_or(""));
    assert!(resp_str.contains("200 OK"));
    assert!(resp_str.contains("Telemetry Recorded"));

    server_handle.join().unwrap();
    println!("    [OK] HTTPS REST POST & JSON Response verified successfully.");
}

fn main() {
    println!("========================================");
    println!(" [Rust Test 08] HTTPS & REST Client     ");
    println!("========================================");

    test_json_serde();
    test_tls_https_loopback();

    println!("========================================");
    println!("  [PASS] Test 08 HTTPS & REST Client passed.");
    println!("========================================");
}
