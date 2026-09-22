use tokio::net::{TcpListener, TcpStream};
use std::io;

/// A simple wrapper for accepting direct local TCP connections.
pub struct DirectServer {
    listener: TcpListener,
}

impl DirectServer {
    pub async fn bind(port: u16) -> io::Result<Self> {
        let listener = TcpListener::bind(format!("0.0.0.0:{}", port)).await?;
        Ok(Self { listener })
    }

    pub async fn accept(&self) -> io::Result<TcpStream> {
        let (stream, addr) = self.listener.accept().await?;
        println!("Accepted direct connection from: {}", addr);
        // The stream would then be wrapped in TLS or ChaCha20Poly1305 encryption
        Ok(stream)
    }
}

/// A simple wrapper for connecting to a discovered device.
pub struct DirectClient;

impl DirectClient {
    pub async fn connect(ip: &str, port: u16) -> io::Result<TcpStream> {
        let stream = TcpStream::connect(format!("{}:{}", ip, port)).await?;
        println!("Connected directly to {}:{}", ip, port);
        Ok(stream)
    }
}
