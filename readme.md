# Mycelia Spore
Typed provisioning daemon using Cap’n Proto + Rust

Mycelia Spore is a Rust daemon that receives Cap’n Proto messages
(`SporeConfig`) and performs deterministic hosting operations for
websites. The initial hosting backend is Cloudflare Pages, and the
architecture is designed for seamless extension to additional
providers.

The system is part of a broader Mycelia / Treelink-inspired
infrastructure where typed binary configurations flow through
reproducible pipelines and hosting is automated from a single,
explicit schema.

## Goals

- Treat the Cap’n Proto schema as the single source of truth for site
  configuration.
- Maintain strong Rust typing end-to-end, with minimal glue.
- Support continuous, streamed provisioning via stdin.
- Integrate cleanly with Nix for build, packaging, and deployment.
- Keep hosting backends modular and replaceable.

## High-level architecture

1. Upstream tools emit `SporeConfig` messages using the `spore.capnp`
   schema.
2. Messages are serialized in **packed Cap’n Proto** format.
3. `mycelia-spore` reads a stream of messages from stdin.
4. Each message is decoded into a typed Rust `SporeConfig`.
5. A hosting backend is selected (e.g. Cloudflare Pages).
6. The backend ensures that projects and domains are in the requested
   state.

## Cap’n Proto schema

The schema is the authoritative definition of configuration. A minimal
example:

```capnp
@0xbad0bad0bad0bad0;

using Rust = import "rust.capnp";

$Rust.module("spore_capnp");

struct Site {
  id    @0 :Text;
  title @1 :Text;
}

struct SporeConfig {
  site @0 :Site;
}
```

In the actual project, the schema also defines:

- `SiteKind`, `HostingProvider`, `RepoProvider`
- `BuildType`, `CfFramework`
- `Domains`, `DnsConfig`, `PurchaseConfig`
- The full `SporeConfig` tree

Rust types for these are generated at build time by `capnpc-rust`.

## Rust domain model

The daemon converts Cap’n Proto readers into a small domain model used
by the hosting logic. Example:

```rust
#[derive(Debug, Clone)]
pub struct Site {
    pub id: String,
    pub title: String,
}

#[derive(Debug, Clone)]
pub struct SporeConfig {
    pub site: Site,
    // repo, build, domains, hosting, dns, purchase, ...
}
```

The mapping is centralized in `decode.rs`, using functions that convert
from `spore_capnp::spore_config::Reader<'_>` into `SporeConfig`.

## Nix integration

The project is designed to be managed primarily via Nix flakes.

### Build with Nix

```sh
nix build .#mycelia-spore
```

This produces a `result` symlink with the `mycelia-spore` binary.

### Run with Nix

```sh
# Single config message from a file
nix run .#mycelia-spore -- < config.bin

# Stream of messages from another process
produce-spores | nix run .#mycelia-spore --
```

### Nix development environment

```sh
# Drop into a dev shell with rustc, cargo, capnp, etc.
nix develop
```

Inside the dev shell:

```sh
cargo build
cargo test
```

Nix can also provide `wrangler` and other runtime tools required by
specific hosting backends.

## Building without Nix

Nix is preferred but not strictly required.

```sh
# Install the Cap’n Proto compiler
# Debian/Ubuntu:
sudo apt install capnproto

# macOS (Homebrew):
brew install capnp

# Then build with Cargo
cargo build
```

## Running the daemon

The daemon reads one or more packed Cap’n Proto `SporeConfig` messages
from stdin. For example:

```sh
# Single message from a file
mycelia-spore < config.bin
```

```sh
# Stream of messages
generate-spores | mycelia-spore
```

Each message is processed in sequence. Hosting changes are applied
idempotently where possible.

## Environment variables (Cloudflare Pages backend)

The Cloudflare Pages backend expects the following environment
variables to be set:

```sh
export CLOUDFLARE_API_TOKEN=...
export CLOUDFLARE_ACCOUNT_ID=...
```

The daemon will exit with an error if these are missing when the
Cloudflare backend is in use.

## Hosting backends

Backends live under `src/hosting/` and are selected based on the
`HostingProvider` enum in the incoming configuration.

Current planned backends:

- `CloudflarePages` – uses `wrangler` to manage Pages projects and
  domains.
- `LocalStatic` – serves static files from a local directory or
  system service.
- `S3Static` – syncs static files to S3-compatible storage and
  configures a static site.
- `CriomosHost` – native Mycelia / CriomOS hosting.

Adding a backend usually involves:

1. Extending `HostingProvider` in the Cap’n Proto schema.
2. Extending the Rust `HostingProvider` enum in `model.rs`.
3. Implementing a new module under `src/hosting/`.
4. Adding a branch in the `apply_hosting` dispatcher.

## Streaming model

`mycelia-spore` is intended to run as a long-lived process in a Nix or
systemd-managed service:

```sh
nix run .#mycelia-spore --
```

Upstream tools can connect via pipes, fifos, or supervised process
chains, pushing new `SporeConfig` messages whenever a site needs to be
created or updated.

This design allows:

- batch provisioning,
- incremental updates,
- and integration into larger pipelines.

## Error handling

- Schema violations or decoding failures result in a logged error and
  a non-zero exit code (for single-run usage).
- Backend-specific failures (e.g. `wrangler` errors) are surfaced with
  as much stderr detail as possible.
- The intent is for upstream orchestrators to detect failures and
  retry or escalate.

## License of Non-Authority

This work is released under the License of Non-Authority. No permission
is granted because none is required. No ownership is claimed because
none exists. All persons are free to use, modify, share, or ignore any
work, including this one, without condition, obligation, attribution, or
restriction. Authority is neither asserted nor recognized.
