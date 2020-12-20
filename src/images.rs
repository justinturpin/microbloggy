use std::path::{Path, PathBuf};
use std::process::Command;

use tide::Result;

pub struct GmImageConvert {
    gm_path: String
}

impl GmImageConvert {
    pub fn new(gm_path: String) -> GmImageConvert {
        GmImageConvert {
            gm_path: gm_path
        }
    }

    pub async fn convert_image(&self, source: &Path, dest: &Path) -> Result<()> {
        let source = PathBuf::from(source);
        let dest = PathBuf::from(dest);
        let gm_path = self.gm_path.clone();

        async_std::task::spawn_blocking(move || {
            let result = Command::new(gm_path)
                .arg("convert")
                .arg(source.as_os_str())
                .arg(dest.as_os_str())
                .output();

            match result {
                Ok(_) => Result::Ok(()),
                Err(e) => Result::Err(tide::Error::from(e))
            }
        }).await
    }

    pub async fn thumbnail_image(&self, source: &Path, dest: &Path, width: u32, height: u32) -> Result<()> {
        let source = PathBuf::from(source);
        let dest = PathBuf::from(dest);
        let gm_path = self.gm_path.clone();

        async_std::task::spawn_blocking(move || {
            let result = Command::new(gm_path)
                .arg("convert")
                .arg(source.as_os_str())
                .arg("-thumbnail")
                .arg(format!("{}x{}", width, height))
                .arg(dest.as_os_str())
                .output();

            match result {
                Ok(_) => Result::Ok(()),
                Err(e) => Result::Err(tide::Error::from(e))
            }
        }).await
    }
}
