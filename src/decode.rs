use anyhow::{Result, anyhow};
use std::io::{self, BufReader};

use capnp::message::ReaderOptions;
use capnp::serialize_packed;

use crate::model::*;
use crate::spore_capnp::spore_config;

impl SiteKind {
    fn from_reader(kind: spore_capnp::site_kind::Reader) -> Result<Self> {
        use spore_capnp::site_kind::Which;
        match kind.which()? {
            Which::Treelink(()) => Ok(SiteKind::Treelink),
            Which::Blog(()) => Ok(SiteKind::Blog),
            Which::Store(()) => Ok(SiteKind::Store),
            Which::Docs(()) => Ok(SiteKind::Docs),
        }
    }
}

impl HostingProvider {
    fn from_reader(p: spore_capnp::hosting_provider::Reader) -> Result<Self> {
        use spore_capnp::hosting_provider::Which;
        Ok(match p.which()? {
            Which::CloudflarePages(()) => HostingProvider::CloudflarePages,
            Which::LocalStatic(()) => HostingProvider::LocalStatic,
            Which::S3Static(()) => HostingProvider::S3Static,
            Which::CriomosHost(()) => HostingProvider::CriomosHost,
        })
    }
}

impl RepoProvider {
    fn from_reader(p: spore_capnp::repo_provider::Reader) -> Result<Self> {
        use spore_capnp::repo_provider::Which;
        Ok(match p.which()? {
            Which::Github(()) => RepoProvider::Github,
            Which::Gitlab(()) => RepoProvider::Gitlab,
            Which::None(()) => RepoProvider::None,
        })
    }
}

impl BuildType {
    fn from_reader(t: spore_capnp::build_type::Reader) -> Result<Self> {
        use spore_capnp::build_type::Which;
        Ok(match t.which()? {
            Which::StaticPrebuilt(()) => BuildType::StaticPrebuilt,
            Which::HugoNix(()) => BuildType::HugoNix,
            Which::NextStatic(()) => BuildType::NextStatic,
            Which::AstroStatic(()) => BuildType::AstroStatic,
        })
    }
}

impl CfFramework {
    fn from_reader(f: spore_capnp::cf_framework::Reader) -> Result<Self> {
        use spore_capnp::cf_framework::Which;
        Ok(match f.which()? {
            Which::None(()) => CfFramework::None,
            Which::Hugo(()) => CfFramework::Hugo,
            Which::Next(()) => CfFramework::Next,
            Which::Astro(()) => CfFramework::Astro,
            Which::Nuxt(()) => CfFramework::Nuxt,
        })
    }
}

impl Registrar {
    fn from_reader(r: spore_capnp::purchase_config::registrar::Reader) -> Result<Self> {
        use spore_capnp::purchase_config::registrar::Which;
        Ok(match r.which()? {
            Which::Namecheap(()) => Registrar::Namecheap,
            Which::Gandi(()) => Registrar::Gandi,
            Which::CloudflareReg(()) => Registrar::CloudflareReg,
        })
    }
}

impl SporeConfig {
    pub fn from_reader(reader: spore_config::Reader) -> Result<Self> {
        let site_reader = reader.get_site()?;
        let repo_reader = reader.get_repo()?;
        let build_reader = reader.get_build()?;
        let domains_reader = reader.get_domains()?;
        let hosting_reader = reader.get_hosting()?;

        let site = Site {
            id: site_reader.get_id()?.to_string(),
            title: site_reader.get_title()?.to_string(),
            kind: SiteKind::from_reader(site_reader.get_kind()?)?,
        };

        let repo = Repo {
            provider: RepoProvider::from_reader(repo_reader.get_provider()?)?,
            slug: repo_reader.get_slug()?.to_string(),
            default_branch: repo_reader.get_default_branch()?.to_string(),
        };

        let build = Build {
            build_type: BuildType::from_reader(build_reader.get_type()?)?,
            output_dir: build_reader.get_output_dir()?.to_string(),
            framework: CfFramework::from_reader(build_reader.get_framework()?)?,
        };

        let aliases_list = domains_reader.get_aliases()?;
        let mut aliases = Vec::with_capacity(aliases_list.len() as usize);
        for i in 0..aliases_list.len() {
            aliases.push(aliases_list.get(i)?.to_string());
        }

        let domains = Domains {
            primary: domains_reader.get_primary()?.to_string(),
            aliases,
        };

        let hosting = Hosting {
            provider: HostingProvider::from_reader(hosting_reader.get_provider()?)?,
            project_name: hosting_reader.get_project_name()?.to_string(),
            production_branch: hosting_reader.get_production_branch()?.to_string(),
        };

        let dns = if reader.has_dns() {
            let dns_reader = reader.get_dns()?;
            let records_list = dns_reader.get_records()?;
            let mut records = Vec::with_capacity(records_list.len() as usize);
            for i in 0..records_list.len() {
                let r = records_list.get(i)?;
                records.push(DnsRecord {
                    name: r.get_name()?.to_string(),
                    record_type: r.get_type()?.to_string(),
                    value: r.get_value()?.to_string(),
                    ttl: r.get_ttl(),
                });
            }
            Some(DnsConfig {
                provider: HostingProvider::from_reader(dns_reader.get_provider()?)?,
                zone_id: dns_reader.get_zone_id()?.to_string(),
                records,
            })
        } else {
            None
        };

        let purchase = if reader.has_purchase() {
            let p = reader.get_purchase()?;
            Some(PurchaseConfig {
                provider: Registrar::from_reader(p.get_provider()?)?,
                domain: p.get_domain()?.to_string(),
            })
        } else {
            None
        };

        Ok(SporeConfig {
            site,
            repo,
            build,
            domains,
            hosting,
            dns,
            purchase,
        })
    }
}

pub struct SporeStream<R: io::Read> {
    reader: BufReader<R>,
    opts: ReaderOptions,
}

impl SporeStream<io::Stdin> {
    pub fn from_stdin() -> Self {
        Self::new(io::stdin())
    }
}

impl<R: io::Read> SporeStream<R> {
    pub fn new(reader: R) -> Self {
        SporeStream {
            reader: BufReader::new(reader),
            opts: ReaderOptions::new(),
        }
    }
}

impl<R: io::Read> Iterator for SporeStream<R> {
    type Item = Result<SporeConfig>;

    fn next(&mut self) -> Option<Self::Item> {
        match serialize_packed::read_message(&mut self.reader, self.opts) {
            Ok(message) => {
                let root: spore_config::Reader = match message.get_root() {
                    Ok(r) => r,
                    Err(e) => return Some(Err(anyhow!("get_root failed: {e}"))),
                };
                Some(SporeConfig::from_reader(root))
            }
            Err(e) => {
                if e.kind() == io::ErrorKind::UnexpectedEof {
                    None
                } else {
                    Some(Err(anyhow!("Cap'n Proto read error: {e}")))
                }
            }
        }
    }
}
