use rand::Rng;
use std::time::{SystemTime, Duration};

const PIN_EXPIRATION_SECONDS: u64 = 300; // 5 minutes

pub struct PairingSession {
    pub pin: String,
    pub created_at: SystemTime,
}

impl PairingSession {
    /// Generates a new 6-digit short-lived pairing pin for QR code or manual entry.
    pub fn new() -> Self {
        let mut rng = rand::thread_rng();
        let pin = format!("{:06}", rng.gen_range(0..1000000));
        
        Self {
            pin,
            created_at: SystemTime::now(),
        }
    }

    /// Validates an entered PIN and ensures it has not expired.
    pub fn validate(&self, entered_pin: &str) -> bool {
        if self.pin != entered_pin {
            return false;
        }

        if let Ok(elapsed) = self.created_at.elapsed() {
            if elapsed > Duration::from_secs(PIN_EXPIRATION_SECONDS) {
                return false; // PIN expired
            }
        } else {
            return false; // Time manipulation
        }

        true
    }
}
