use tokio::{fs, io::AsyncWriteExt};
use std::path::PathBuf;

pub struct LocalStorage {
    root: PathBuf
}

impl LocalStorage {
    pub fn new(root: impl Into<PathBuf>) -> Self {
        Self { root: root.into() }
    }

    pub async fn save_blob(&self, repo: &str, digest: &str, data: &[u8]) -> std::io::Result<()> {
        let path = self.root.join(repo).join("blobs").join("sha256").join(digest);
        fs::create_dir_all(path.parent().unwrap()).await?;
        let mut file = fs::File::create(path).await?;
        file.write_all(data).await?;
        Ok(())
    }

    pub async fn load_blob(&self, repo: &str, digest: &str) -> std::io::Result<Vec<u8>> {
        let path = self.root.join(repo).join("blobs").join("sha256").join(digest);
        fs::read(path).await
    }
}
