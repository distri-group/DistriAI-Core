use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct Statistics {
    pub owner: Pubkey,
    pub machine_reward_claimed: u64,
    pub machine_reward_claimable: u64,
    pub ai_model_dataset_reward_claimed: u64,
    pub ai_model_dataset_reward_claimable: u64,
    pub machine_earning: u64,
    pub ai_model_dataset_earning: u64,
}
