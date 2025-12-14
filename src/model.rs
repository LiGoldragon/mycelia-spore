#[derive(Debug, Clone, Copy)]
pub enum SiteIntent {
    IdentityIndex,
    Publication,
    Commerce,
    Documentation,
}

#[derive(Debug, Clone, Copy)]
pub enum HostingAuthorityRole {
    DesignatedOrigin,
    DelegatedOrigin,
    FederatedOrigin,
    ExternallyManagedOrigin,
}

#[derive(Debug, Clone, Copy)]
pub enum DeploymentArtifactKind {
    StaticContent,
    VersionedStaticContent,
    CompositeStaticContent,
}

#[derive(Debug, Clone)]
pub struct SiteIdentity {
    pub canonical_id: String,
    pub human_readable_name: String,
    pub intent: SiteIntent,
}

#[derive(Debug, Clone)]
pub struct DeploymentArtifact {
    pub kind: DeploymentArtifactKind,
    pub output_path: String,
}

#[derive(Debug, Clone)]
pub struct DomainAssignment {
    pub canonical_domain: String,
    pub alternate_domains: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct HostingDesignation {
    pub authority_role: HostingAuthorityRole,
    pub external_binding_reference: String,
}

#[derive(Debug, Clone)]
pub struct NameResolutionRecord {
    pub record_name: String,
    pub record_type: String,
    pub record_value: String,
    pub time_to_live_seconds: u32,
}

#[derive(Debug, Clone)]
pub struct NameResolutionConfiguration {
    pub records: Vec<NameResolutionRecord>,
}

#[derive(Debug, Clone)]
pub struct DomainAcquisitionInstruction {
    pub registrar_identifier: String,
    pub domain_name: String,
}

#[derive(Debug, Clone)]
pub struct SporeConfiguration {
    pub site_identity: SiteIdentity,
    pub deployment_artifact: DeploymentArtifact,
    pub domain_assignment: DomainAssignment,
    pub hosting_designation: HostingDesignation,
    pub name_resolution: NameResolutionConfiguration,
    pub domain_acquisition: Option<DomainAcquisitionInstruction>,
}
