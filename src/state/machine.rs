use anchor_lang::prelude::*;

#[account]
pub struct Machine {
    pub owner: Pubkey,  // 32
    pub uuid: [u8; 16], // 16
    /// The metadata by json format of this machine.
    pub metadata: String, // 4 + 2048
    /// The status of this machine.
    pub status: MachineStatus, // 1 + 1
    /// The price of this machine.
    pub price: u64, //8
    /// The maximum number of hours the machine can be rent.
    pub max_duration: u32, //4
    /// The GB amount of this machine's avaliable disk.
    pub disk: u32, //4

    pub completed_count: u32, //4

    pub failed_count: u32, //4

    pub score: u8, // 1
}

impl Machine {
    pub const MAXIMUM_SIZE: usize = 32 + 16 + (4 + 2048) + (1 + 1) + 8 + 4 + 4 + 4 + 4 + 1;
    pub const METADATA_MAX_LENGTH: usize = 2048;
}

/// MachineStatus holds the current state of the machine.
#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq, Eq)]
pub enum MachineStatus {
    /// This machine is idle, not display in the market.
    Idle,
    /// This machine is for rent, display in the market.
    ForRent,
    /// This machine is on lease, not display in the market.
    Renting,
}
