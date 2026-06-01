use soroban_sdk::{contracttype, Address, Bytes};

/// Represents the kind of attestation issued to a contributor.
/// Each kind indicates the role or achievement being recognized.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
#[contracttype]
pub enum AttestationKind {
    /// Recognition for active contribution to a project or mission
    Contributor = 0,
    /// Recognition for reviewing and validating contributions
    Reviewer = 1,
    /// Recognition for founding or initiating a project
    Founder = 2,
    /// Recognition for supporting the ecosystem
    EcosystemPartner = 3,
    /// Recognition for completing a program or series of missions
    ProgramCompletion = 4,
}

/// Represents a portable attestation record issued by the contract.
/// Attestations are immutable proof of achievement or contribution.
#[derive(Clone, Debug, PartialEq, Eq)]
#[contracttype]
pub struct Attestation {
    /// Unique identifier for the attestation
    pub id: Bytes,
    /// The recipient of the attestation (the subject being recognized)
    pub subject: Address,
    /// The entity issuing the attestation
    pub issuer: Address,
    /// The kind of recognition being issued
    pub kind: AttestationKind,
    /// Human-readable label or title for the attestation
    pub label: Bytes,
    /// IPFS CID or other content identifier for metadata
    pub metadata_cid: Bytes,
    /// Timestamp when the attestation was issued (seconds since epoch)
    pub issued_at: u64,
    /// Optional expiration timestamp (seconds since epoch), None means no expiration
    pub expires_at: Option<u64>,
    /// Whether the attestation has been revoked
    pub revoked: bool,
}

impl Attestation {
    /// Check if the attestation is currently valid (not expired and not revoked)
    pub fn is_valid(&self, current_time: u64) -> bool {
        if self.revoked {
            return false;
        }
        
        if let Some(expiration) = self.expires_at {
            return current_time <= expiration;
        }
        
        true
    }
}
