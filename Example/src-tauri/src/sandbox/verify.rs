//! 校验模块

use std::path::Path;

pub async fn get_version(bin_path: &Path) -> Result<String, super::SandboxError> {
    // 实现代码省略
    unimplemented!()
}

pub async fn checksum(bin_path: &Path) -> Result<(), super::SandboxError> {
    // 实现代码省略
    Ok(())
}
