use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct Task {
    /// Task id
    pub uuid: [u8; 16],
    /// Period
    pub period: u32,
    /// Machine owner.
    pub owner: Pubkey,
    /// Machine id of this task.
    pub machine_id: [u8; 16],
    /// The metadata by json format of this task.
    #[max_len(2048)]
    pub metadata: String,
}

impl Task {
    pub const METADATA_MAX_LENGTH: usize = 2048;
}
