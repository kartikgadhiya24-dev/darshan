use serde::{Deserialize, Serialize};
use std::net::{SocketAddr, Ipv4Addr};
use tokio::net::UdpSocket;
use std::sync::Arc;

const DISCOVERY_PORT: u16 = 48593; // A dedicated UDP port for RemoteLinkDesk

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DiscoveryBeacon {
    pub device_name: String,
    pub public_key: String, // Hex string of the Ed25519 identity
    pub tcp_port: u16,
}

pub struct DiscoveryService {
    socket: Arc<UdpSocket>,
}

impl DiscoveryService {
    /// Binds the UDP socket for listening to discovery broadcasts
    pub async fn new() -> std::io::Result<Self> {
        let addr = SocketAddr::from(([0, 0, 0, 0], DISCOVERY_PORT));
        let socket = UdpSocket::bind(addr).await?;
        socket.set_broadcast(true)?;
        
        Ok(Self {
            socket: Arc::new(socket),
        })
    }

    /// Periodically broadcast our identity to the local network
    pub async fn broadcast_beacon(socket: Arc<UdpSocket>, beacon: DiscoveryBeacon) {
        let broadcast_addr = SocketAddr::from((Ipv4Addr::new(255, 255, 255, 255), DISCOVERY_PORT));
        
        loop {
            if let Ok(message) = serde_json::to_string(&beacon) {
                let _ = socket.send_to(message.as_bytes(), broadcast_addr).await;
            }
            tokio::time::sleep(tokio::time::Duration::from_secs(3)).await;
        }
    }

    /// Listen for other devices on the network
    pub async fn listen(&self) {
        let mut buf = [0; 1024];
        loop {
            if let Ok((len, addr)) = self.socket.recv_from(&mut buf).await {
                if let Ok(msg) = std::str::from_utf8(&buf[..len]) {
                    if let Ok(beacon) = serde_json::from_str::<DiscoveryBeacon>(msg) {
                        println!("Discovered device: {} at {}", beacon.device_name, addr.ip());
                        // In a real app, this would trigger an event/callback to the UI
                    }
                }
            }
        }
    }
}
