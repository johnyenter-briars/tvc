pub mod routes;
pub mod cec;

use serde::Deserialize;
use std::{fs, io, path::PathBuf};

pub const DOMAIN: &str = "0.0.0.0";
pub const OSD_NAME: &'static str = "TVC";

#[derive(Debug, Deserialize)]
pub struct AppConfig {
    pub port: u16,
    pub use_sudo: bool,
}

pub fn load_config() -> io::Result<AppConfig> {
    let path = PathBuf::from("tvc.config.json");
    let contents = fs::read_to_string(&path).map_err(|err| {
        io::Error::new(
            err.kind(),
            format!("failed to read {}: {}", path.display(), err),
        )
    })?;

    let config = serde_json::from_str(&contents).map_err(|err| {
        io::Error::new(
            io::ErrorKind::InvalidData,
            format!("failed to parse {}: {}", path.display(), err),
        )
    })?;

    Ok(config)
}
