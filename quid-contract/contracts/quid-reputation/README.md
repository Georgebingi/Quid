# Quid Reputation Contract

This contract manages portable attestation records that recognize contributors and participants in the Quid ecosystem.

## Features

- **Attestation Kinds**: Multiple recognition types (Contributor, Reviewer, Founder, EcosystemPartner, ProgramCompletion)
- **Portable Records**: Attestations are serializable and can be transferred across the Soroban network
- **Revocation Support**: Issued attestations can be revoked if needed
- **Expiration Handling**: Attestations can have optional expiration dates

## Building

```bash
cargo build --target wasm32-unknown-unknown --release
```

## Testing

```bash
cargo test
```
