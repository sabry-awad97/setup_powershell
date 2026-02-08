use anyhow::Result;
use async_trait::async_trait;
use futures_util::StreamExt;
use std::path::Path;
use tokio::fs;
use tokio::io::AsyncWriteExt;

use crate::domain::interfaces::Downloader;

/// HTTP file downloader using reqwest
pub struct HttpDownloader {
    client: reqwest::Client,
}

impl HttpDownloader {
    pub fn new() -> Self {
        Self {
            client: reqwest::Client::new(),
        }
    }
}

impl Default for HttpDownloader {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl Downloader for HttpDownloader {
    async fn download(&self, url: &str, path: &Path) -> Result<()> {
        let response = self.client.get(url).send().await?;

        if !response.status().is_success() {
            anyhow::bail!("Failed to download: HTTP {}", response.status());
        }

        let mut file = fs::File::create(path).await?;
        let mut stream = response.bytes_stream();

        while let Some(chunk) = stream.next().await {
            let chunk = chunk?;
            file.write_all(&chunk).await?;
        }

        file.flush().await?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_downloader_creation() {
        let downloader = HttpDownloader::new();
        assert!(std::mem::size_of_val(&downloader) > 0);
    }
}
