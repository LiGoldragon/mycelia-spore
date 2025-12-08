use anyhow::Result;

use crate::model::{Hosting, HostingProvider, SporeConfig};

mod cloudflare;
use cloudflare::CloudflarePages;

impl Hosting {
    pub async fn apply(&self, config: &SporeConfig) -> Result<()> {
        match self.provider {
            HostingProvider::CloudflarePages => CloudflarePages::apply(config).await,
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
}
