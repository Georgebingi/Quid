#![cfg(test)]

use super::*;
use crate::types::{MilestoneStatus, ProgramStatus};
use soroban_sdk::Env;

#[test]
fn test_default_statuses() {
    let env = Env::default();
    let contract_id = env.register(QuidMilestoneEscrowContract, ());
    let client = QuidMilestoneEscrowContractClient::new(&env, &contract_id);

    assert_eq!(client.get_program_status(), ProgramStatus::Active);
    assert_eq!(client.get_milestone_status(), MilestoneStatus::Pending);
}

#[test]
fn test_status_storage_roundtrip() {
    let env = Env::default();
    let contract_id = env.register(QuidMilestoneEscrowContract, ());
    let client = QuidMilestoneEscrowContractClient::new(&env, &contract_id);

    client.set_program_status(&ProgramStatus::Completed);
    client.set_milestone_status(&MilestoneStatus::Paid);

    assert_eq!(client.get_program_status(), ProgramStatus::Completed);
    assert_eq!(client.get_milestone_status(), MilestoneStatus::Paid);
}
