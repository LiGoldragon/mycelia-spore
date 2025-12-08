use anyhow::{Context, Result, anyhow};
use serde_json::Value;
use std::collections::HashSet;
use std::env;
use tokio::process::Command;

use crate::model::{BuildType, CfFramework, SporeConfig};

pub struct CloudflarePages<'a> {
    config: &'a SporeConfig,
}

impl<'a> CloudflarePages<'a> {
    pub async fn apply(config: &'a SporeConfig) -> Result<()> {
        let pages = CloudflarePages { config };
        pages.require_env("CLOUDFLARE_API_TOKEN")?;
        pages.require_env("CLOUDFLARE_ACCOUNT_ID")?;
        pages.ensure_project().await?;
        pages.ensure_domains().await?;
        Ok(())
    }

    fn require_env(&self, key: &str) -> Result<String> {
        env::var(key).with_context(|| format!("{key} must be set"))
    }

    async fn project_exists(&self, project_name: &str) -> Result<bool> {
        let output = Command::new("wrangler")
            .args(["pages", "project", "list", "--format", "json"])
            .output()
            .await
            .context("failed to run wrangler pages project list")?;

        if !output.status.success() {
            return Err(anyhow!(
                "wrangler pages project list failed: {}",
                String::from_utf8_lossy(&output.stderr)
            ));
        }

        let stdout = String::from_utf8_lossy(&output.stdout);
        let val: Value = serde_json::from_str(&stdout)?;
        let exists = val
            .as_array()
            .unwrap_or(&vec![])
            .iter()
            .any(|p| p.get("name").and_then(|n| n.as_str()) == Some(project_name));
        Ok(exists)
    }

    fn cf_framework(&self) -> String {
        match self.config.build.framework {
            CfFramework::None => "none",
            CfFramework::Hugo => "hugo",
            CfFramework::Next => "next",
            CfFramework::Astro => "astro",
            CfFramework::Nuxt => "nuxt",
        }
        .to_string()
    }

    fn build_params(&self) -> (Option<String>, String) {
        let output = self.config.build.output_dir.clone();

        // Build commands remain None; build happens in external pipeline.
        let command = match self.config.build.build_type {
            BuildType::StaticPrebuilt => None,
            BuildType::HugoNix => None,
            BuildType::NextStatic => None,
            BuildType::AstroStatic => None,
        };

        (command, output)
    }

    fn project_name(&self) -> String {
        if self.config.hosting.project_name.is_empty() {
            self.config.site.id.clone()
        } else {
            self.config.hosting.project_name.clone()
        }
    }

    fn production_branch(&self) -> String {
        if self.config.hosting.production_branch.is_empty() {
            if self.config.repo.default_branch.is_empty() {
                "main".to_string()
            } else {
                self.config.repo.default_branch.clone()
            }
        } else {
            self.config.hosting.production_branch.clone()
        }
    }

    async fn ensure_project(&self) -> Result<()> {
        let project_name = self.project_name();
        let prod_branch = self.production_branch();

        let (build_cmd, build_output) = self.build_params();
        let framework = self.cf_framework();

        let exists = self.project_exists(&project_name).await?;
        let mut args: Vec<String> = vec![
            "pages".into(),
            "project".into(),
            if exists {
                "update".into()
            } else {
                "create".into()
            },
            project_name.clone(),
            "--production-branch".into(),
            prod_branch,
        ];

        if !build_output.is_empty() {
            args.push("--build-output".into());
            args.push(build_output);
        }

        if let Some(cmd) = build_cmd {
            args.push("--build-command".into());
            args.push(cmd);
        }

        if !exists {
            args.push("--framework".into());
            args.push(framework);
            if !self.config.repo.slug.is_empty() {
                args.push("--source=github".into());
                args.push("--repo".into());
                args.push(self.config.repo.slug.clone());
            } else {
                args.push("--source=none".into());
            }
        }

        let status = Command::new("wrangler")
            .args(&args)
            .status()
            .await
            .context("failed to run wrangler pages project create/update")?;

        if !status.success() {
            return Err(anyhow!(
                "wrangler pages project {} failed with status {status}",
                if exists { "update" } else { "create" }
            ));
        }

        Ok(())
    }

    async fn ensure_domains(&self) -> Result<()> {
        let project_name = self.project_name();

        let output = Command::new("wrangler")
            .args(["pages", "domain", "list", &project_name, "--format", "json"])
            .output()
            .await
            .context("failed to run wrangler pages domain list")?;

        if !output.status.success() {
            return Err(anyhow!(
                "wrangler pages domain list failed: {}",
                String::from_utf8_lossy(&output.stderr)
            ));
        }

        let stdout = String::from_utf8_lossy(&output.stdout);
        let val: Value = serde_json::from_str(&stdout)?;

        let existing: HashSet<String> = val
            .as_array()
            .unwrap_or(&vec![])
            .iter()
            .filter_map(|d| {
                d.get("domain")
                    .and_then(|v| v.as_str())
                    .map(|s| s.to_string())
            })
            .collect();

        let mut desired: Vec<String> = Vec::new();
        desired.push(self.config.domains.primary.clone());
        desired.extend(self.config.domains.aliases.iter().cloned());

        for d in desired {
            if existing.contains(&d) {
                println!("Domain already attached: {d}");
                continue;
            }

            println!("Attaching domain: {d}");
            let status = Command::new("wrangler")
                .args(["pages", "domain", "add", &project_name, &d])
                .status()
                .await
                .context("failed to run wrangler pages domain add")?;

            if !status.success() {
                return Err(anyhow!("wrangler pages domain add {d} failed"));
            }
        }

        Ok(())
    }
}
