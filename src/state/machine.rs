use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct Machine {
    pub owner: Pubkey,
    pub uuid: [u8; 16],
    /// The metadata by json format of this machine.
    #[max_len(2048)]
    pub metadata: String,
    /// The status of this machine.
    pub status: MachineStatus,
    /// The price of this machine.
    pub price: u64,
    /// The maximum number of hours the machine can be rent.
    pub max_duration: u32,
    /// The GB amount of this machine's avaliable disk.
    pub disk: u32,

    pub completed_count: u32,

    pub failed_count: u32,

    pub score: u8,
    /// Total claimed periodic rewards.
    pub claimed_periodic_rewards: u64,
    /// Total claimed task rewards.
    pub claimed_task_rewards: u64,
    // Rencently order pda
    pub order_pda: Pubkey,
}

impl Machine {
    pub const METADATA_MAX_LENGTH: usize = 2048;
}

#[account]
#[derive(InitSpace)]
pub struct MachineNew {
    pub owner: Pubkey,  // 32
    pub uuid: [u8; 16], // 16
    /// The metadata by json format of this machine.
    #[max_len(2048)]
    pub metadata: String, // 4 + 2048
    /// The status of this machine.
    pub status: MachineStatus, // 1 + 1
    /// The price of this machine.
    pub price: u64, // 8
    /// The maximum number of hours the machine can be rent.
    pub max_duration: u32, // 4
    /// The GB amount of this machine's avaliable disk.
    pub disk: u32, // 4

    pub completed_count: u32, // 4

    pub failed_count: u32, // 4

    pub score: u8, // 1
    /// Total claimed periodic rewards.
    pub claimed_periodic_rewards: u64, // 8
    /// Total claimed task rewards.
    pub claimed_task_rewards: u64, // 8
    // Rencently order pda
    pub order_pda: Pubkey,
}

/// MachineStatus holds the current state of the machine.
#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq, Eq, InitSpace)]
pub enum MachineStatus {
    /// This machine is idle, not display in the market.
    Idle,
    /// This machine is for rent, display in the market.
    ForRent,
    /// This machine is on lease, not display in the market.
    Renting,
}
