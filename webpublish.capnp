@0x9c1f8d7e6b5a4a3b;

using Rust = import "rust.capnp";

$Rust.module("webpublish_capnp");

###############################################################################
# MVP: Git-first Cloudflare Pages project creation (Cloudflare builds first)
#
# This schema models only what is needed to drive the following operations:
#   1) Create (or ensure) a GitHub-connected Cloudflare Pages project
#   2) Attach one or more custom domains to that Pages project
#
# Execution is expected to be performed via Cloudflare’s official CLI tool
# ("wrangler"), which acts as the compatibility layer over Cloudflare’s
# control plane and authentication flows (including GitHub OAuth).
###############################################################################

struct WebPublishConfiguration {
  site @0 :SiteIdentity;
  # Human identity metadata for the site. Not required for the wrangler call
  # itself, but useful as a stable internal catalog record.

  source @1 :GitHubRepositorySource;
  # The GitHub repository used as the source of truth. Cloudflare Pages will
  # clone this repository and perform builds in Cloudflare’s infrastructure.

  pages @2 :CloudflarePagesProject;
  # Cloudflare Pages project parameters used for creation / reconciliation.

  domains @3 :DomainBindings;
  # Optional custom domain bindings applied after the project exists.
}

###############################################################################
# SITE IDENTITY (INTERNAL)
###############################################################################

struct SiteIdentity {
  stableId @0 :Text;
  # Stable internal identifier for this site.
  # Intended invariants:
  #   - must not change across redeployments
  #   - must not change if domains change
  #   - must not change if the Pages project name changes

  displayName @1 :Text;
  # Human-facing descriptive name.
}

###############################################################################
# GITHUB SOURCE (EXTERNAL)
###############################################################################

struct GitHubRepositorySource {
  owner @0 :Text;
  # GitHub account or organization name (for <owner>/<repo>).

  repository @1 :Text;
  # GitHub repository name (for <owner>/<repo>).
}

###############################################################################
# CLOUDFLARE PAGES PROJECT (EXTERNAL CONTROL PLANE)
###############################################################################

struct CloudflarePagesProject {
  projectName @0 :Text;
  # Cloudflare Pages project name.
  # Notes:
  #   - Used as the primary key in many wrangler operations.
  #   - Must be unique within the chosen Cloudflare account context.

  productionBranch @1 :Text;
  # Git branch used by Cloudflare Pages for "production" deployments.
  # Common values: "main", "master", "develop".

  accountId @2 :union {
    absent @0 :Void;
    # If absent, wrangler’s currently authenticated account context is used.
    # This is sufficient for single-account operators.

    value @1 :Text;
    # Cloudflare Account Identifier.
    #
    # Meaning:
    #   - Identifies which Cloudflare account owns the Pages project.
    #
    # Where it comes from:
    #   - Visible in the Cloudflare Dashboard URLs.
    #   - Obtainable via Cloudflare API "accounts" listing endpoints.
    #
    # How it is used:
    #   - Some Cloudflare workflows accept an explicit account id.
    #   - For wrangler, the practical behavior may still depend on authentication
    #     context; treat this as a selector hint to support multi-account setups.
    #
    # Security:
    #   - Not a secret. It is an identifier, not a credential.
  }

  buildCommand @3 :Text;
  # Build command executed by Cloudflare Pages in its build environment.
  #
  # Important:
  #   - This must be treated as opaque pass-through. It is not executed locally
  #     by WebPublish; it is stored in Cloudflare and executed by Cloudflare.
  #
  # Environment variables:
  #   - Cloudflare Pages defines build-time variables such as CF_PAGES_URL and
  #     others. If the command contains "$CF_PAGES_URL", it must remain literal
  #     until Cloudflare executes it (no local shell expansion).
  #
  # Convention in this stack:
  #   - Use a Nix flake build invocation (example patterns):
  #       "nix build .#pages --print-build-logs"
  #       "nix build .#default --print-build-logs"

  buildOutputDirectory @4 :Text;
  # Directory served as the Pages output.
  #
  # Interpretation:
  #   - Relative to the repository root as seen by Cloudflare Pages.
  #   - Cloudflare will serve files under this directory as the site content.
  #
  # Convention in this stack:
  #   - When Nix produces a symlink named "result", common choices include:
  #       "result"
  #       "result/public"
  #
  # Note:
  #   - Cloudflare Pages expects a directory path, not a file.
}

###############################################################################
# DOMAIN BINDINGS (EXTERNAL CONTROL PLANE)
###############################################################################

struct DomainBindings {
  primaryDomain @0 :union {
    absent @0 :Void;
    # No primary domain binding requested.

    domain @1 :Text;
    # A custom domain to attach to the Pages project.
    # Example: "ligoldragon.com"
  }

  alternateDomains @1 :List(Text);
  # Additional domains to attach to the Pages project.
  # Example: ["www.ligoldragon.com"]
  #
  # Notes:
  #   - Ordering is not semantically meaningful.
  #   - Idempotency is expected at execution time (already-bound is success).
}
