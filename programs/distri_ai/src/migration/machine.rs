use anchor_lang::prelude::*;
use crate::state::machine::*;

pub fn migrate_machine_new(ctx: Context<MigrationMachineNew>) -> Result<()> {
    let machine_before = &mut ctx.accounts.machine_before;
    let machine_after = &mut ctx.accounts.machine_after;
    machine_after.owner = machine_before.owner;
    machine_after.uuid = machine_before.uuid;
    machine_after.metadata = machine_before.metadata.clone();
    machine_after.status = machine_before.status.clone();
    machine_after.price = machine_before.price;
    machine_after.max_duration = machine_before.max_duration;
    machine_after.disk = machine_before.disk;
    machine_after.completed_count = machine_before.completed_count;
    machine_after.failed_count = machine_before.failed_count;
    machine_after.score = machine_before.score;
    machine_after.claimed_periodic_rewards = machine_before.claimed_periodic_rewards;
    machine_after.claimed_task_rewards = machine_before.claimed_task_rewards;

    Ok(())
}

pub fn migrate_machine_rename(ctx: Context<MigrationMachineRename>) -> Result<()> {
    let machine_before = &mut ctx.accounts.machine_before;
    let machine_after = &mut ctx.accounts.machine_after;
    machine_after.owner = machine_before.owner;
    machine_after.uuid = machine_before.uuid;
    machine_after.metadata = machine_before.metadata.clone();
    machine_after.status = machine_before.status.clone();
    machine_after.price = machine_before.price;
    machine_after.max_duration = machine_before.max_duration;
    machine_after.disk = machine_before.disk;
    machine_after.completed_count = machine_before.completed_count;
    machine_after.failed_count = machine_before.failed_count;
    machine_after.score = machine_before.score;
    machine_after.claimed_periodic_rewards = machine_before.claimed_periodic_rewards;
    machine_after.claimed_task_rewards = machine_before.claimed_task_rewards;
    machine_after.order_pda = machine_before.order_pda;
    
    Ok(())
}

/// Defines a structure for migrating a machine account to a new version.
///
/// The `Accounts` macro is used to automatically handle account de/serialization and provide
/// convenient access to account information within the program.
///
/// `'info` is a lifetime specifier that indicates the data lives as long as the program execution.
#[derive(Accounts)]
pub struct MigrationMachineNew<'info> {
    #[account(
        mut,
        close = signer
    )]
    pub machine_before: Account<'info, Machine>,

    #[account(
        init,
        seeds = [b"machine-new", machine_before.owner.as_ref(), machine_before.uuid.as_ref()],
        bump,
        payer = signer,
        space = 8 + MachineNew::INIT_SPACE
    )]
    pub machine_after: Account<'info, MachineNew>,

    #[account(mut)]
    pub signer: Signer<'info>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct MigrationMachineRename<'info> {
    #[account(
        mut,
        close = signer
    )]
    pub machine_before: Account<'info, MachineNew>,

    #[account(
        init,
        seeds = [b"machine", machine_before.owner.as_ref(), machine_before.uuid.as_ref()],
        bump,
        payer = signer,
        space = 8 + Machine::INIT_SPACE
    )]
    pub machine_after: Account<'info, Machine>,

    #[account(mut)]
    pub signer: Signer<'info>,

    pub system_program: Program<'info, System>,
}
