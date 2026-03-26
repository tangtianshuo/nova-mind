//! 自动更新模块

use std::path::Path;
use semver::Version;

pub async fn fetch_latest_version() -> Result<String, super::SandboxError> {
    // 实现代码省略
    unimplemented!()
}

pub async fn download_version(version: &str, dest: &Path) -> Result<(), super::SandboxError> {
    // 实现代码省略
    unimplemented!()
}
