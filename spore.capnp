@0x9c1f8d7e6b5a4a3b;

using Rust = import "rust.capnp";

$Rust.module("spore_capnp");
# Generated Rust code will live under the `spore_capnp` module.

###############################################################################
# CORE CONCEPTS
###############################################################################

enum SiteIntent {
  identityIndex @0;
  # A lightweight identity or index site whose primary purpose
  # is to reference other resources.

  publication @1;
  # A site whose primary purpose is publishing authored content.

  commerce @2;
  # A site whose primary purpose is offering goods or services.

  documentation @3;
  # A site whose primary purpose is reference or technical material.
}

###############################################################################
# AUTHORITY AND ROLE
###############################################################################

enum HostingAuthorityRole {
  designatedOrigin @0;
  # This site is served by a node explicitly designated as an
  # authoritative origin within the Criome.

  delegatedOrigin @1;
  # This site is served by an external authority delegated
  # responsibility by the Criome.

  federatedOrigin @2;
  # This site is served through a federation of cooperating nodes,
  # none of which are singularly authoritative.

  externallyManagedOrigin @3;
  # This site is served by an external system that is not governed
  # by Criome authority but is bound by contract or interface.
}

###############################################################################
# ARTIFACT DESCRIPTION
###############################################################################

enum DeploymentArtifactKind {
  staticContent @0;
  # A static, immutable artifact intended to be served as-is.

  versionedStaticContent @1;
  # A static artifact with explicit versioning semantics.

  compositeStaticContent @2;
  # A static artifact composed of multiple independently generated
  # components assembled into a single deployment unit.
}

###############################################################################
# SITE IDENTITY
###############################################################################

struct SiteIdentity {
  canonicalId @0 :Text;
  # Stable internal identifier for the site.
  # Must remain invariant across redeployments.

  humanReadableName @1 :Text;
  # Descriptive name intended for human reference.

  intent @2 :SiteIntent;
  # Declared purpose of the site.
}

###############################################################################
# ARTIFACT CONTRACT
###############################################################################

struct DeploymentArtifact {
  kind @0 :DeploymentArtifactKind;
  # Declares the contractual nature of the artifact.

  outputPath @1 :Text;
  # Filesystem-relative path containing the deployable artifact.
  # Interpretation is backend-specific but semantically stable.
}

###############################################################################
# NAMING AND ADDRESSING
###############################################################################

struct DomainAssignment {
  canonicalDomain @0 :Text;
  # Primary domain name representing the site’s canonical address.

  alternateDomains @1 :List(Text);
  # Additional domain names bound to the same site identity.
}

###############################################################################
# HOSTING DESIGNATION
###############################################################################

struct HostingDesignation {
  authorityRole @0 :HostingAuthorityRole;
  # Declares the role of the hosting authority within the Criome.

  externalBindingReference @1 :Text;
  # Optional opaque identifier used to bind this designation
  # to an external system when applicable.
}

###############################################################################
# NAME RESOLUTION (TRANSITIONAL INTERFACE)
###############################################################################

struct NameResolutionRecord {
  recordName @0 :Text;
  # Name of the record (e.g. root, subdomain).

  recordType @1 :Text;
  # Record type (A, AAAA, CNAME, TXT, etc.).

  recordValue @2 :Text;
  # Value associated with the record.

  timeToLiveSeconds @3 :UInt32;
  # DNS cache lifetime in seconds.
}

struct NameResolutionConfiguration {
  records @0 :List(NameResolutionRecord);
  # Complete set of name-resolution records to apply.
}

###############################################################################
# ECONOMIC ACTIONS (OPTIONAL)
###############################################################################

struct DomainAcquisitionInstruction {
  registrarIdentifier @0 :Text;
  # Identifier for the external registrar or acquisition mechanism.

  domainName @1 :Text;
  # Fully qualified domain name to be acquired.
}

###############################################################################
# ROOT CONFIGURATION
###############################################################################

struct SporeConfiguration {
  siteIdentity @0 :SiteIdentity;
  # Identity and declared intent of the site.

  deploymentArtifact @1 :DeploymentArtifact;
  # Description of the artifact being deployed.

  domainAssignment @2 :DomainAssignment;
  # Domain names bound to the site.

  hostingDesignation @3 :HostingDesignation;
  # Authority and role governing hosting.

  nameResolution @4 :NameResolutionConfiguration;
  # Name resolution instructions (transitional interface).

  domainAcquisition @5 :DomainAcquisitionInstruction;
  # Optional instruction to acquire a domain prior to deployment.
}
