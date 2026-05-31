use soroban_sdk::contracttype;

#[derive(Clone, Debug, Default, PartialEq, Eq, Copy)]
#[contracttype]
pub enum ProgramStatus {
    #[default]
    Active,
    Completed,
    Cancelled,
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Copy)]
#[contracttype]
pub enum MilestoneStatus {
    #[default]
    Pending,
    Paid,
    Cancelled,
}

#[contracttype]
pub enum DataKey {
    ProgramStatus,
    MilestoneStatus,
}
