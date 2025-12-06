use anyhow::Result;

use crate::model::{HostingProvider, SporeConfig};

mod cloudflare;
pub use cloudflare::apply_cloudflare_pages;

pub fn apply_hosting(config: &SporeConfig) -> Result<()> {
    match config.hosting.provider {
        HostingProvider::CloudflarePages => apply_cloudflare_pages(config),
        HostingProvider::LocalStatic => {
            // placeholder for future local static hosting
            Ok(())
        }
        HostingProvider::S3Static => {
            // placeholder
            Ok(())
        }
        HostingProvider::CriomosHost => {
            // placeholder
            Ok(())
        }
    }
}
