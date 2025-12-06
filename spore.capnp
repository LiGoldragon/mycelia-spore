@0xbad0bad0bad0bad0;  # Replace with a unique ID.

using Rust = import "rust.capnp";

$Rust.module("spore_capnp");

enum SiteKind {
  treelink @0;
  blog     @1;
  store    @2;
  docs     @3;
}

enum HostingProvider {
  cloudflarePages @0;
  localStatic     @1;
  s3Static        @2;
  criomosHost     @3;
}

enum RepoProvider {
  github @0;
  gitlab @1;
  none   @2;
}

enum BuildType {
  staticPrebuilt @0;
  hugoNix        @1;
  nextStatic     @2;
  astroStatic    @3;
}

enum CfFramework {
  none  @0;
  hugo  @1;
  next  @2;
  astro @3;
  nuxt  @4;
}

struct Site {
  id    @0 :Text;
  title @1 :Text;
  kind  @2 :SiteKind;
}

struct Repo {
  provider      @0 :RepoProvider;
  slug          @1 :Text;
  defaultBranch @2 :Text;
}

struct Build {
  type       @0 :BuildType;
  outputDir  @1 :Text;
  framework  @2 :CfFramework;
}

struct Domains {
  primary @0 :Text;
  aliases @1 :List(Text);
}

struct Hosting {
  provider         @0 :HostingProvider;
  projectName      @1 :Text;
  productionBranch @2 :Text;
}

struct DnsRecord {
  name  @0 :Text;
  type  @1 :Text;
  value @2 :Text;
  ttl   @3 :UInt32;
}

struct DnsConfig {
  provider @0 :HostingProvider;
  zoneId   @1 :Text;
  records  @2 :List(DnsRecord);
}

struct PurchaseConfig {
  enum Registrar {
    namecheap     @0;
    gandi         @1;
    cloudflareReg @2;
  }

  provider @0 :Registrar;
  domain   @1 :Text;
}

struct SporeConfig {
  site     @0 :Site;
  repo     @1 :Repo;
  build    @2 :Build;
  domains  @3 :Domains;
  hosting  @4 :Hosting;
  dns      @5 :DnsConfig;
  purchase @6 :PurchaseConfig;
}
