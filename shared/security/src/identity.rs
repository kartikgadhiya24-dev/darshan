use ed25519_dalek::{SigningKey, VerifyingKey, Signature, Signer, Verifier};
use rand::rngs::OsRng;

/// Represents the cryptographic identity of a RemoteLinkDesk device.
pub struct DeviceIdentity {
    pub signing_key: SigningKey,
}

impl DeviceIdentity {
    /// Generates a new random device identity. This should be generated once
    /// on first startup and securely stored on the host OS.
    pub fn generate_new() -> Self {
        let mut csprng = OsRng;
        let signing_key = SigningKey::generate(&mut csprng);
        Self { signing_key }
    }

    /// Load an existing identity from bytes (e.g., from secure storage).
    pub fn from_bytes(bytes: &[u8; 32]) -> Self {
        let signing_key = SigningKey::from_bytes(bytes);
        Self { signing_key }
    }

    /// Returns the public verifying key representing the device ID.
    pub fn public_key(&self) -> VerifyingKey {
        self.signing_key.verifying_key()
    }

    /// Signs a message to prove identity to another device.
    pub fn sign_message(&self, message: &[u8]) -> Signature {
        self.signing_key.sign(message)
    }

    /// Verifies a signature from a remote device.
    pub fn verify_signature(public_key: &VerifyingKey, message: &[u8], signature: &Signature) -> bool {
        public_key.verify(message, signature).is_ok()
    }
}
