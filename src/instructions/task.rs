use anchor_lang::prelude::*;
use crate::errors::DistriAIError;
use crate::state::machine::*;
use crate::state::reward::*;
use crate::state::task::*;

pub fn submit_task(
    ctx: Context<SubmitTask>,
    uuid: [u8; 16],
    period: u32,
    metadata: String,
) -> Result<()> {
    require_gte!(
        Task::METADATA_MAX_LENGTH,
        metadata.len(),
        DistriAIError::StringTooLong
    );
    let machine = &mut ctx.accounts.machine;
    require!(
        machine.status == MachineStatus::ForRent,
        DistriAIError::IncorrectStatus
    );
    require_eq!(
        period,
        Reward::current_period()?,
        DistriAIError::InvalidPeriod
    );

    let task = &mut ctx.accounts.task;
    task.uuid = uuid;
    task.period = period;
    task.owner = ctx.accounts.owner.key();
    task.machine_id = machine.uuid;
    task.metadata = metadata;

    let reward_machine = &mut ctx.accounts.reward_machine;
    reward_machine.period = period;
    reward_machine.owner = machine.owner;
    reward_machine.machine_id = machine.uuid;
    reward_machine.task_num = reward_machine.task_num.saturating_add(1);

    let reward = &mut ctx.accounts.reward;
    reward.period = period;
    if reward.start_time == 0 {
        reward.start_time = Reward::start_time(period);
        reward.pool = Reward::pool(period);
    }
    if reward_machine.task_num == 1 {
        reward.machine_num = reward.machine_num.saturating_add(1);
    }
    reward.unit_periodic_reward = reward.pool.saturating_div(reward.machine_num.into());
    reward.task_num = reward.task_num.saturating_add(1);

    emit!(TaskEvent {
        uuid: task.uuid,
        period: task.period,
        owner: task.owner,
        machine_id: task.machine_id,
    });
    Ok(())
}

#[derive(Accounts)]
#[instruction(uuid: [u8; 16], period: u32)]
pub struct SubmitTask<'info> {
    #[account(
        mut,
        has_one = owner
    )]
    pub machine: Account<'info, Machine>,

    #[account(
        init,
        seeds = [b"task", owner.key().as_ref(), uuid.as_ref()],
        bump,
        payer = owner,
        space = 8 + Task::INIT_SPACE
    )]
    pub task: Account<'info, Task>,

    #[account(
        init_if_needed,
        seeds = [b"reward", period.to_le_bytes().as_ref()],
        bump,
        payer = owner,
        space = 8 + Reward::INIT_SPACE
    )]
    pub reward: Account<'info, Reward>,

    #[account(
        init_if_needed,
        seeds = [b"reward-machine", period.to_le_bytes().as_ref(), owner.key().as_ref(), machine.uuid.as_ref()],
        bump,
        payer = owner,
        space = 8 + RewardMachine::INIT_SPACE
    )]
    pub reward_machine: Account<'info, RewardMachine>,

    #[account(mut)]
    pub owner: Signer<'info>,

    pub system_program: Program<'info, System>,
}

#[event]
pub struct TaskEvent {
    pub uuid: [u8; 16],
    pub period: u32,
    pub owner: Pubkey,
    pub machine_id: [u8; 16],
}
