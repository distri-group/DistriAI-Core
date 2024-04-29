use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct Dataset {
    pub owner: Pubkey,
    #[max_len(50)]
    pub name: String,
    pub scale: u8,
    pub license: u8,
    pub type1: u8,
    pub type2: u8,
    #[max_len(128)]
    pub tags: String,
    pub create_time: i64,
    pub update_time: i64,
}

impl Dataset {
    pub const NAME_MAX_LENGTH: usize = 50;
    pub const TAGS_MAX_LENGTH: usize = 128;
}
