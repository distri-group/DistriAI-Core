use anchor_lang::prelude::*;

#[account]
pub struct Reward {
    pub period: u32, // 4
    /// Reward pool in this period.
    pub pool: u64, // 8
    /// Participating machine in this period.
    pub machine_num: u32, // 4
}

impl Reward {
    pub const MAXIMUM_SIZE: usize = 4 + 8 + 4;
    const GENESIS_TIME: i64 = 1706745600;
    const PERIOD_DURATION: i64 = 3600 * 24;
    const DECAY_PERIODS: u32 = 4;
    const DECAY_RATE_NUMERATOR: u64 = 9737;
    const DECAY_RATE_DENOMINATOR: u64 = 10000;
    const GENESIS_POOL: u64 = 65_750_000_000_000;

    pub fn current_period() -> Result<u32> {
        let now_ts = Clock::get()?.unix_timestamp;
        let period: u32 = now_ts
            .saturating_sub(Reward::GENESIS_TIME)
            .saturating_div(Reward::PERIOD_DURATION)
            .try_into()
            .unwrap();
        return Ok(period);
    }

    pub fn pool(period: u32) -> u64 {
        let decay_times: u32 = period.saturating_div(Reward::DECAY_PERIODS);
        let mut pool = Reward::GENESIS_POOL;
        for _ in 0..decay_times {
            pool = pool
                .saturating_mul(Reward::DECAY_RATE_NUMERATOR)
                .saturating_div(Reward::DECAY_RATE_DENOMINATOR);
        }
        return pool;
    }
}

#[account]
pub struct RewardMachine {
    pub period: u32, // 4
    /// Machine owner.
    pub owner: Pubkey, // 32
    /// Machine id.
    pub machine_id: [u8; 16], // 16
    /// Task number submited in this period.
    pub task_num: u32, // 4
    /// Reward has been claimed.
    pub claimed: bool, // 1
}

impl RewardMachine {
    pub const MAXIMUM_SIZE: usize = 4 + 32 + 16 + 4 + 1;
}
