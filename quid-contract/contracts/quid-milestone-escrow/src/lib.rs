#![no_std]

use soroban_sdk::{contract, contractimpl, Env};

mod error;
mod types;

use types::{DataKey, MilestoneStatus, ProgramStatus};

#[contract]
pub struct QuidMilestoneEscrowContract;

#[contractimpl]
impl QuidMilestoneEscrowContract {
    pub fn get_program_status(env: Env) -> ProgramStatus {
        env.storage()
            .persistent()
            .get(&DataKey::ProgramStatus)
            .unwrap_or_default()
    }

    pub fn set_program_status(env: Env, status: ProgramStatus) {
        env.storage()
            .persistent()
            .set(&DataKey::ProgramStatus, &status);
    }

    pub fn get_milestone_status(env: Env) -> MilestoneStatus {
        env.storage()
            .persistent()
            .get(&DataKey::MilestoneStatus)
            .unwrap_or_default()
    }

    pub fn set_milestone_status(env: Env, status: MilestoneStatus) {
        env.storage()
            .persistent()
            .set(&DataKey::MilestoneStatus, &status);
    }
}

mod test;
