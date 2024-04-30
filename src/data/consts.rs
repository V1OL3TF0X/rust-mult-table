use std::path::PathBuf;

use platform_dirs::AppDirs;
pub const CELL_N: usize = 10;
pub const CELL_WIDTH: u16 = 35;
pub const SPACING: u16 = 10;
pub fn app_dir() -> PathBuf {
    AppDirs::new(Some("rust_mult_table"), false)
        .unwrap()
        .config_dir
}

pub async fn create_app_dir() {
    let _ = tokio::fs::create_dir_all(app_dir()).await;
}
