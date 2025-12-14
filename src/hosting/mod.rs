use anyhow::Result;

use crate::model::{HostingAuthorityRole, HostingDesignation, SporeConfiguration};

impl HostingDesignation {
    pub async fn apply(&self, config: &SporeConfiguration) -> Result<()> {
        let role = match self.authority_role {
            HostingAuthorityRole::DesignatedOrigin => "designated-origin",
            HostingAuthorityRole::DelegatedOrigin => "delegated-origin",
            HostingAuthorityRole::FederatedOrigin => "federated-origin",
            HostingAuthorityRole::ExternallyManagedOrigin => "externally-managed-origin",
        };

        println!(
            "Applying hosting designation {role} for {} (artifact: {}, domains: {})",
            config.site_identity.canonical_id,
            config.deployment_artifact.output_path,
            config.domain_assignment.canonical_domain
        );

        if let Some(acquisition) = &config.domain_acquisition {
            println!(
                "Domain acquisition requested via {} for {}",
                acquisition.registrar_identifier, acquisition.domain_name
            );
        }

        Ok(())
    }
}
