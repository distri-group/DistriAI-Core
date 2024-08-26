use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct Reward {
    /// Reward period.
    pub period: u32,
    /// Start time of this reward period.
    pub start_time: i64,
    /// Reward pool in this period.
    pub pool: u64,
    /// Participating machine number in this period.
    pub machine_num: u32,
    /// Periodic reward per machine in this period.
    pub unit_periodic_reward: u64,
    /// Task number in this period.
    pub task_num: u32,
    /// Task reward per task in this period.
    pub unit_task_reward: u64,
}

impl Reward {
    /// Period 0 start time: 2024-02-27 00:00:00 UTC
    const GENESIS_TIME: i64 = 1708992000;
    const PERIOD_DURATION: i64 = 86400;
    const DECAY_PERIODS: u32 = 4;
    const DECAY_RATE_NUMERATOR: u64 = 9737;
    const DECAY_RATE_DENOMINATOR: u64 = 10000;
    // const GENESIS_POOL: u64 = 65_750_000_000_000;
    /// Checkpoint every 10 decays
    const POOL_CHECKPOINTS: [u64; 31] = [
        65_750_000_000_000,
        50_367_000_000_000,
        38_583_000_000_000,
        29_556_000_000_000,
        22_641_000_000_000,
        17_344_000_000_000,
        13_286_000_000_000,
        10_178_000_000_000,
        7_797_000_000_000,
        5_973_000_000_000,
        4_575_000_000_000,
        3_505_000_000_000,
        2_685_000_000_000,
        2_057_000_000_000,
        1_576_000_000_000,
        1_207_000_000_000,
        925_000_000_000,
        708_000_000_000,
        543_000_000_000,
        416_000_000_000,
        318_000_000_000,
        244_000_000_000,
        187_000_000_000,
        143_000_000_000,
        110_000_000_000,
        84_000_000_000,
        64_000_000_000,
        49_000_000_000,
        38_000_000_000,
        29_000_000_000,
        22_000_000_000,
    ];
    // current_period calculates the current reward period based on the current Unix timestamp.
    // It returns a Result with a u32 representing the current period or an error if the operation fails.
    pub fn current_period() -> Result<u32> {
        let now_ts = Clock::get()?.unix_timestamp;
        let period: u32 = now_ts
            .saturating_sub(Reward::GENESIS_TIME)
            .saturating_div(Reward::PERIOD_DURATION)
            .try_into()
            .unwrap();
        Ok(period)
    }

    pub fn pool(period: u32) -> u64 {
        // Calculate how many full DECAY_PERIODS fit into the given period
        let decay_times: usize = period
            .saturating_div(Reward::DECAY_PERIODS)
            .try_into()
            .unwrap();
        
        // Determine the checkpoint index based on the decay times
        let mut checkpoint_index: usize = decay_times.saturating_div(10).try_into().unwrap();
        
        // Ensure checkpoint_index does not exceed the length of POOL_CHECKPOINTS
        if checkpoint_index > Reward::POOL_CHECKPOINTS.len() - 1 {
            checkpoint_index = Reward::POOL_CHECKPOINTS.len() - 1;
        }
        
        // Calculate remaining decay times after considering the checkpoint index
        let remaining_decay_times = decay_times.saturating_sub(checkpoint_index.saturating_mul(10));
        
        // Initialize the pool with the value from POOL_CHECKPOINTS at checkpoint_index
        let mut pool = Reward::POOL_CHECKPOINTS[checkpoint_index];
        
        // Apply decay for the remaining decay times
        for _ in 0..remaining_decay_times {
            pool = pool
                .saturating_mul(Reward::DECAY_RATE_NUMERATOR)
                .saturating_div(Reward::DECAY_RATE_DENOMINATOR);
        }
        pool
    }

    pub fn start_time(period: u32) -> i64 {
        Reward::PERIOD_DURATION
            .saturating_mul(period.into())
            .saturating_add(Reward::GENESIS_TIME)
    }
}

#[account]
#[derive(InitSpace)]
pub struct RewardMachine {
    pub period: u32,
    /// Machine owner.
    pub owner: Pubkey,
    /// Machine id.
    pub machine_id: [u8; 16],
    /// Task number submited in this period.
    pub task_num: u32,
    /// Reward has been claimed.
    pub claimed: bool,
}
