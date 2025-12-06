use anyhow::{anyhow, Context, Result};
use serde_json::Value;
use std::collections::HashSet;
use std::env;
use std::process::Command;

use crate::model::{BuildType, CfFramework, SporeConfig};

fn require_env(key: &str) -> Result<String> {
    env::var(key).with_context(|| format!("{key} must be set"))
}

fn project_exists(project_name: &str) -> Result<bool> {
    let output = Command::new("wrangler")
        .args(["pages", "project", "list", "--format", "json"])
        .output()
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

fn cf_framework(build: &crate::model::Build) -> String {
    match build.framework {
        CfFramework::None => "none",
        CfFramework::Hugo => "hugo",
        CfFramework::Next => "next",
        CfFramework::Astro => "astro",
        CfFramework::Nuxt => "nuxt",
    }
    .to_string()
}

fn build_params(build: &crate::model::Build) -> (Option<String>, String) {
    let output = build.output_dir.clone();

    // Build commands remain None; build happens in external pipeline.
    let command = match build.build_type {
        BuildType::StaticPrebuilt => None,
        BuildType::HugoNix => None,
        BuildType::NextStatic => None,
        BuildType::AstroStatic => None,
    };

    (command, output)
}

fn ensure_project(config: &SporeConfig) -> Result<()> {
    let project_name = if config.hosting.project_name.is_empty() {
        config.site.id.clone()
    } else {
        config.hosting.project_name.clone()
    };

    let prod_branch = if config.hosting.production_branch.is_empty() {
        if config.repo.default_branch.is_empty() {
            "main".to_string()
        } else {
            config.repo.default_branch.clone()
        }
    } else {
        config.hosting.production_branch.clone()
    };

    let (build_cmd, build_output) = build_params(&config.build);
    let framework = cf_framework(&config.build);

    let exists = project_exists(&project_name)?;
    let mut args: Vec<String> = vec![
        "pages".into(),
        "project".into(),
        if exists { "update".into() } else { "create".into() },
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
        if !config.repo.slug.is_empty() {
            args.push("--source=github".into());
            args.push("--repo".into());
            args.push(config.repo.slug.clone());
        } else {
            args.push("--source=none".into());
        }
    }

    let status = Command::new("wrangler")
        .args(&args)
        .status()
        .context("failed to run wrangler pages project create/update")?;

    if !status.success() {
        return Err(anyhow!(
            "wrangler pages project {} failed with status {status}",
            if exists { "update" } else { "create" }
        ));
    }

    Ok(())
}

fn ensure_domains(config: &SporeConfig) -> Result<()> {
    let project_name = if config.hosting.project_name.is_empty() {
        config.site.id.clone()
    } else {
        config.hosting.project_name.clone()
    };

    let output = Command::new("wrangler")
        .args(["pages", "domain", "list", &project_name, "--format", "json"])
        .output()
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
        .filter_map(|d| d.get("domain").and_then(|v| v.as_str()).map(|s| s.to_string()))
        .collect();

    let mut desired: Vec<String> = Vec::new();
    desired.push(config.domains.primary.clone());
    desired.extend(config.domains.aliases.iter().cloned());

    for d in desired {
        if existing.contains(&d) {
            println!("Domain already attached: {d}");
            continue;
        }

        println!("Attaching domain: {d}");
        let status = Command::new("wrangler")
            .args(["pages", "domain", "add", &project_name, &d])
            .status()
            .context("failed to run wrangler pages domain add")?;

        if !status.success() {
            return Err(anyhow!("wrangler pages domain add {d} failed"));
        }
    }

    Ok(())
}

pub fn apply_cloudflare_pages(config: &SporeConfig) -> Result<()> {
    require_env("CLOUDFLARE_API_TOKEN")?;
    require_env("CLOUDFLARE_ACCOUNT_ID")?;
    ensure_project(config)?;
    ensure_domains(config)?;
    Ok(())
}
