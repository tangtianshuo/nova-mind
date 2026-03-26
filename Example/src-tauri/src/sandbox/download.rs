//! 下载管理模块

use std::path::Path;
use tokio::io::AsyncWriteExt;
use futures_util::StreamExt;

const GITHUB_RELEASES_URL: &str = "https://github.com/openclaw/openclaw/releases/download";
const MIRROR_URLS: &[&str] = &[
    "https://ghproxy.com/https://github.com/openclaw/openclaw/releases/download",
];

pub struct DownloadProgress {
    pub downloaded: u64,
    pub total: Option<u64>,
    pub speed: f64,
}

pub async fn fetch_binary<P: AsRef<Path>>(dest: P) -> Result<(), super::SandboxError> {
    // 实现代码省略
    Ok(())
}
