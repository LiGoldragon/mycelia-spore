@0x9c1f8d7e6b5a4a3b;

using Rust = import "rust.capnp";

$Rust.module("webpublish_capnp");

###############################################################################
# MVP: Git-first Cloudflare Pages project creation (Cloudflare builds first)
###############################################################################

struct WebPublishConfiguration {
  site @0 :SiteIdentity;
  source @1 :GitHubRepositorySource;
  pages @2 :CloudflarePagesProject;
  domains @3 :DomainBindings;
}

struct SiteIdentity {
  stableId @0 :Text;
  displayName @1 :Text;
}

struct GitHubRepositorySource {
  owner @0 :Text;
  repository @1 :Text;
  # Example: owner="LiGoldragon", repository="webpage".
}

struct CloudflarePagesProject {
  projectName @0 :Text;
  # Example: "webpage".

  productionBranch @1 :Text;
  # Example: "main".

  sourceProvider @2 :PagesSourceProvider;
  # MVP: github.

  buildCommand @3 :Text;
  # Example: "hugo --gc --minify -b $CF_PAGES_URL".

  buildOutputDirectory @4 :Text;
  # Example: "public".
}

enum PagesSourceProvider {
  github @0;
}

struct DomainBindings {
  primaryDomain @0 :union {
    absent @0 :Void;
    domain @1 :Text;
  }

  alternateDomains @1 :List(Text);
}
