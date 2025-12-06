#[derive(Debug, Clone, Copy)]
pub enum SiteKind {
    Treelink,
    Blog,
    Store,
    Docs,
}

#[derive(Debug, Clone, Copy)]
pub enum HostingProvider {
    CloudflarePages,
    LocalStatic,
    S3Static,
    CriomosHost,
}

#[derive(Debug, Clone, Copy)]
pub enum RepoProvider {
    Github,
    Gitlab,
    None,
}

#[derive(Debug, Clone, Copy)]
pub enum BuildType {
    StaticPrebuilt,
    HugoNix,
    NextStatic,
    AstroStatic,
}

#[derive(Debug, Clone, Copy)]
pub enum CfFramework {
    None,
    Hugo,
    Next,
    Astro,
    Nuxt,
}

#[derive(Debug, Clone)]
pub struct Site {
    pub id: String,
    pub title: String,
    pub kind: SiteKind,
}

#[derive(Debug, Clone)]
pub struct Repo {
    pub provider: RepoProvider,
    pub slug: String,
    pub default_branch: String,
}

#[derive(Debug, Clone)]
pub struct Build {
    pub build_type: BuildType,
    pub output_dir: String,
    pub framework: CfFramework,
}

#[derive(Debug, Clone)]
pub struct Domains {
    pub primary: String,
    pub aliases: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct Hosting {
    pub provider: HostingProvider,
    pub project_name: String,
    pub production_branch: String,
}

#[derive(Debug, Clone)]
pub struct DnsRecord {
    pub name: String,
    pub record_type: String,
    pub value: String,
    pub ttl: u32,
}

#[derive(Debug, Clone)]
pub struct DnsConfig {
    pub provider: HostingProvider,
    pub zone_id: String,
    pub records: Vec<DnsRecord>,
}

#[derive(Debug, Clone)]
pub enum Registrar {
    Namecheap,
    Gandi,
    CloudflareReg,
}

#[derive(Debug, Clone)]
pub struct PurchaseConfig {
    pub provider: Registrar,
    pub domain: String,
}

#[derive(Debug, Clone)]
pub struct SporeConfig {
    pub site: Site,
    pub repo: Repo,
    pub build: Build,
    pub domains: Domains,
    pub hosting: Hosting,
    pub dns: Option<DnsConfig>,
    pub purchase: Option<PurchaseConfig>,
}
