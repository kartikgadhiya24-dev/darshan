use tokio::net::TcpStream;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use std::io;
use serde::{Deserialize, Serialize};

/// Represents an encrypted packet that can be safely routed through the neutral relay
#[derive(Serialize, Deserialize, Debug)]
pub struct RelayPacket {
    pub target_public_key: String, // Identity of the destination device
    pub encrypted_payload: Vec<u8>, // ChaCha20Poly1305 encrypted protocol message
}

/// Client to connect to the RemoteLinkDesk Relay Server when NAT Traversal fails.
pub struct RelayClient {
    stream: TcpStream,
}

impl RelayClient {
    /// Connects to the fallback relay server
    pub async fn connect(relay_server_url: &str, my_public_key: &str) -> io::Result<Self> {
        let mut stream = TcpStream::connect(relay_server_url).await?;
        
        // Authenticate with the relay server to subscribe to our own message queue
        let auth_msg = format!("SUBSCRIBE {}", my_public_key);
        stream.write_all(auth_msg.as_bytes()).await?;
        
        println!("Connected to Secure Relay Fallback.");
        
        Ok(Self { stream })
    }

    /// Sends an end-to-end encrypted packet through the relay to the peer
    pub async fn send_to_peer(&mut self, packet: &RelayPacket) -> io::Result<()> {
        let data = serde_json::to_vec(packet)?;
        self.stream.write_all(&data).await?;
        Ok(())
    }

    /// Receives an encrypted packet from the peer via the relay
    pub async fn receive_from_peer(&mut self) -> io::Result<RelayPacket> {
        let mut buf = vec![0u8; 4096];
        let n = self.stream.read(&mut buf).await?;
        
        if n == 0 {
            return Err(io::Error::new(io::ErrorKind::ConnectionAborted, "Relay disconnected"));
        }
        
        let packet: RelayPacket = serde_json::from_slice(&buf[..n])
            .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;
            
        Ok(packet)
    }
}
