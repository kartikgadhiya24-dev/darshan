use std::path::Path;
use tokio::fs::File;
use tokio::io::{AsyncWriteExt, AsyncSeekExt, Result, SeekFrom};
use sha2::{Sha256, Digest};
use crate::chunker::{FileChunk, FileTransferMetadata};

/// Handles writing incoming chunks securely to disk and verifying integrity
pub struct FileReceiver {
    file: File,
    hasher: Sha256,
    metadata: FileTransferMetadata,
    bytes_received: u64,
}

impl FileReceiver {
    pub async fn create(save_dir: &str, metadata: FileTransferMetadata) -> Result<Self> {
        let path = Path::new(save_dir).join(&metadata.file_name);
        
        // Ensure the file is safely created
        let file = File::create(path).await?;
        
        Ok(Self {
            file,
            hasher: Sha256::new(),
            metadata,
            bytes_received: 0,
        })
    }

    /// Writes an incoming network chunk to the physical disk safely
    pub async fn receive_chunk(&mut self, chunk: FileChunk) -> Result<()> {
        // Seek to the correct offset (allows out-of-order chunks or resume if supported later)
        self.file.seek(SeekFrom::Start(chunk.offset)).await?;
        self.file.write_all(&chunk.data).await?;
        
        // Update hash
        self.hasher.update(&chunk.data);
        self.bytes_received += chunk.data.len() as u64;
        
        Ok(())
    }

    /// Finalizes the transfer and verifies the cryptographic hash matches the sender
    pub async fn finalize(self) -> Result<bool> {
        self.file.sync_all().await?;
        
        if self.bytes_received != self.metadata.total_size {
            println!("Warning: File size mismatch.");
            return Ok(false);
        }

        let hash_result = self.hasher.finalize();
        let calculated_hash = hex::encode(hash_result);

        if calculated_hash == self.metadata.expected_sha256 {
            println!("File transfer successful. SHA-256 integrity verified.");
            Ok(true)
        } else {
            println!("CRITICAL: File transfer integrity failed. Hash mismatch!");
            Ok(false)
        }
    }
}
