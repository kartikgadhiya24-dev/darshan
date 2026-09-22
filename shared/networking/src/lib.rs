pub mod discovery;
pub mod connection;

pub use discovery::{DiscoveryService, DiscoveryBeacon};
pub use connection::{DirectServer, DirectClient};
