use std::path::Path;
use tokio::fs::File;
use tokio::io::{AsyncReadExt, Result};
use serde::{Deserialize, Serialize};
use sha2::{Sha256, Digest};

const CHUNK_SIZE: usize = 1024 * 1024; // 1MB chunks

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FileTransferMetadata {
    pub file_name: String,
    pub total_size: u64,
    pub expected_sha256: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct FileChunk {
    pub offset: u64,
    pub data: Vec<u8>,
}

/// Handles reading large files from disk safely and converting them into network-ready chunks
pub struct FileChunker {
    file_path: String,
}

impl FileChunker {
    pub fn new(path: &str) -> Self {
        Self { file_path: path.to_string() }
    }

    /// Calculates the SHA-256 hash of the entire file prior to transfer
    pub async fn calculate_hash(&self) -> Result<String> {
        let mut file = File::open(&self.file_path).await?;
        let mut hasher = Sha256::new();
        let mut buffer = [0u8; CHUNK_SIZE];

        loop {
            let n = file.read(&mut buffer).await?;
            if n == 0 { break; }
            hasher.update(&buffer[..n]);
        }

        let hash_result = hasher.finalize();
        Ok(hex::encode(hash_result))
    }

    /// Stream the file chunks (in a real app, this takes a callback or returns a stream)
    pub async fn process_chunks<F, Fut>(&self, mut callback: F) -> Result<()>
    where
        F: FnMut(FileChunk) -> Fut,
        Fut: std::future::Future<Output = Result<()>>,
    {
        let mut file = File::open(&self.file_path).await?;
        let mut buffer = vec![0u8; CHUNK_SIZE];
        let mut current_offset: u64 = 0;

        loop {
            let n = file.read(&mut buffer).await?;
            if n == 0 { break; }

            let chunk = FileChunk {
                offset: current_offset,
                data: buffer[..n].to_vec(),
            };

            callback(chunk).await?;
            current_offset += n as u64;
        }

        Ok(())
    }
}
