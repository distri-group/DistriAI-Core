use anchor_lang::prelude::*;

#[account]
pub struct Task {
    /// Task id
    pub uuid: [u8; 16], // 16
    /// Machine owner.
    pub owner: Pubkey,  // 32
    /// Machine id of this task.
    pub machine_id: [u8; 16], // 16
    /// The metadata by json format of this task.
    pub metadata: String, // 4 + 2048
}

impl Task {
    pub const MAXIMUM_SIZE: usize = 16 + 32 + 16 + (4 + 2048);
    pub const METADATA_MAX_LENGTH: usize = 2048;
}
