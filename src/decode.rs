use anyhow::{Result, anyhow};
use std::io::{self, BufReader};

use capnp::message::ReaderOptions;
use capnp::serialize_packed;

use crate::model::*;
use crate::spore_capnp::spore_configuration;

impl SiteIntent {
    fn from_reader(intent: crate::spore_capnp::site_intent::Reader) -> Result<Self> {
        use crate::spore_capnp::site_intent::Which;
        match intent.which()? {
            Which::IdentityIndex(()) => Ok(SiteIntent::IdentityIndex),
            Which::Publication(()) => Ok(SiteIntent::Publication),
            Which::Commerce(()) => Ok(SiteIntent::Commerce),
            Which::Documentation(()) => Ok(SiteIntent::Documentation),
        }
    }
}

impl HostingAuthorityRole {
    fn from_reader(role: crate::spore_capnp::hosting_authority_role::Reader) -> Result<Self> {
        use crate::spore_capnp::hosting_authority_role::Which;
        Ok(match role.which()? {
            Which::DesignatedOrigin(()) => HostingAuthorityRole::DesignatedOrigin,
            Which::DelegatedOrigin(()) => HostingAuthorityRole::DelegatedOrigin,
            Which::FederatedOrigin(()) => HostingAuthorityRole::FederatedOrigin,
            Which::ExternallyManagedOrigin(()) => HostingAuthorityRole::ExternallyManagedOrigin,
        })
    }
}

impl DeploymentArtifactKind {
    fn from_reader(kind: crate::spore_capnp::deployment_artifact_kind::Reader) -> Result<Self> {
        use crate::spore_capnp::deployment_artifact_kind::Which;
        Ok(match kind.which()? {
            Which::StaticContent(()) => DeploymentArtifactKind::StaticContent,
            Which::VersionedStaticContent(()) => DeploymentArtifactKind::VersionedStaticContent,
            Which::CompositeStaticContent(()) => DeploymentArtifactKind::CompositeStaticContent,
        })
    }
}

impl SporeConfiguration {
    pub fn from_reader(reader: spore_configuration::Reader) -> Result<Self> {
        let site_identity_reader = reader.get_site_identity()?;
        let deployment_artifact_reader = reader.get_deployment_artifact()?;
        let domain_assignment_reader = reader.get_domain_assignment()?;
        let hosting_designation_reader = reader.get_hosting_designation()?;
        let name_resolution_reader = reader.get_name_resolution()?;

        let site_identity = SiteIdentity {
            canonical_id: site_identity_reader.get_canonical_id()?.to_string(),
            human_readable_name: site_identity_reader.get_human_readable_name()?.to_string(),
            intent: SiteIntent::from_reader(site_identity_reader.get_intent()?)?,
        };

        let deployment_artifact = DeploymentArtifact {
            kind: DeploymentArtifactKind::from_reader(deployment_artifact_reader.get_kind()?)?,
            output_path: deployment_artifact_reader.get_output_path()?.to_string(),
        };

        let alternate_domains_reader = domain_assignment_reader.get_alternate_domains()?;
        let mut alternate_domains = Vec::with_capacity(alternate_domains_reader.len() as usize);
        for i in 0..alternate_domains_reader.len() {
            alternate_domains.push(alternate_domains_reader.get(i)?.to_string());
        }

        let domain_assignment = DomainAssignment {
            canonical_domain: domain_assignment_reader.get_canonical_domain()?.to_string(),
            alternate_domains,
        };

        let hosting_designation = HostingDesignation {
            authority_role: HostingAuthorityRole::from_reader(
                hosting_designation_reader.get_authority_role()?,
            )?,
            external_binding_reference: hosting_designation_reader
                .get_external_binding_reference()
                .unwrap_or_default()
                .to_string(),
        };

        let records_reader = name_resolution_reader.get_records()?;
        let mut records = Vec::with_capacity(records_reader.len() as usize);
        for i in 0..records_reader.len() {
            let r = records_reader.get(i)?;
            records.push(NameResolutionRecord {
                record_name: r.get_record_name()?.to_string(),
                record_type: r.get_record_type()?.to_string(),
                record_value: r.get_record_value()?.to_string(),
                time_to_live_seconds: r.get_time_to_live_seconds(),
            });
        }

        let name_resolution = NameResolutionConfiguration { records };

        let domain_acquisition = if reader.has_domain_acquisition() {
            let acquisition_reader = reader.get_domain_acquisition()?;
            Some(DomainAcquisitionInstruction {
                registrar_identifier: acquisition_reader.get_registrar_identifier()?.to_string(),
                domain_name: acquisition_reader.get_domain_name()?.to_string(),
            })
        } else {
            None
        };

        Ok(SporeConfiguration {
            site_identity,
            deployment_artifact,
            domain_assignment,
            hosting_designation,
            name_resolution,
            domain_acquisition,
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
    type Item = Result<SporeConfiguration>;

    fn next(&mut self) -> Option<Self::Item> {
        match serialize_packed::read_message(&mut self.reader, self.opts) {
            Ok(message) => {
                let root: spore_configuration::Reader = match message.get_root() {
                    Ok(r) => r,
                    Err(e) => return Some(Err(anyhow!("get_root failed: {e}"))),
                };
                Some(SporeConfiguration::from_reader(root))
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
