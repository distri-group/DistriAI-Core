use anchor_lang::prelude::*;
use crate::errors::DistriAIError;
use crate::state::machine::*;

pub fn add_machine(ctx: Context<AddMachine>, uuid: [u8; 16], metadata: String) -> Result<()> {
    // Check if the length of metadata is greater than or equal to the predefined maximum length `Machine::METADATA_MAX_LENGTH`
    // If not, return `DistriAIError::StringTooLong` error
    require_gte!(
        Machine::METADATA_MAX_LENGTH,
        metadata.len(),
        DistriAIError::StringTooLong
    );
    // Get a mutable reference to the smart contract state data named `machine`
    let machine = &mut ctx.accounts.machine;
    machine.owner = ctx.accounts.owner.key();
    machine.uuid = uuid;
    machine.metadata = metadata;
    machine.status = MachineStatus::Idle;
    
    // Emit a `MachineEvent` event with the owner and UUID of the newly created machine
    emit!(MachineEvent {
        owner: machine.owner,
        uuid: machine.uuid,
    });
    Ok(())
}

pub fn remove_machine(ctx: Context<RemoveMachine>) -> Result<()> {
    let machine = &mut ctx.accounts.machine;
    require!(
        machine.status != MachineStatus::Renting,
        DistriAIError::IncorrectStatus
    );

    emit!(MachineEvent {
        owner: machine.owner,
        uuid: machine.uuid,
    });
    Ok(())
}

pub fn make_offer(ctx: Context<MakeOffer>, price: u64, max_duration: u32, disk: u32) -> Result<()> {
    let machine = &mut ctx.accounts.machine;
    require!(
        machine.status == MachineStatus::Idle,
        DistriAIError::IncorrectStatus
    );

    machine.status = MachineStatus::ForRent;
    machine.price = price;
    machine.max_duration = max_duration;
    machine.disk = disk;

    emit!(MachineEvent {
        owner: machine.owner,
        uuid: machine.uuid,
    });
    Ok(())
}

pub fn cancel_offer(ctx: Context<CancelOffer>) -> Result<()> {
    let machine = &mut ctx.accounts.machine;
    require!(
        machine.status == MachineStatus::ForRent,
        DistriAIError::IncorrectStatus
    );

    emit!(MachineEvent {
        owner: machine.owner,
        uuid: machine.uuid,
    });
    machine.status = MachineStatus::Idle;
    Ok(())
}

#[derive(Accounts)]
#[instruction(uuid: [u8; 16])]
pub struct AddMachine<'info> {
    #[account(
        init,
        seeds = [b"machine", owner.key().as_ref(), uuid.as_ref()],
        bump,
        payer = owner,
        space = 8 + Machine::INIT_SPACE
    )]
    pub machine: Box<Account<'info, Machine>>,

    #[account(mut)]
    pub owner: Signer<'info>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct RemoveMachine<'info> {
    #[account(
        mut,
        has_one = owner,
        close = owner
    )]
    pub machine: Box<Account<'info, Machine>>,

    #[account(mut)]
    pub owner: Signer<'info>,
}

#[derive(Accounts)]
pub struct MakeOffer<'info> {
    #[account(
        mut,
        has_one = owner
    )]
    pub machine: Box<Account<'info, Machine>>,

    #[account(mut)]
    pub owner: Signer<'info>,
}

#[derive(Accounts)]
pub struct CancelOffer<'info> {
    #[account(
        mut,
        has_one = owner
    )]
    pub machine: Box<Account<'info, Machine>>,

    #[account(mut)]
    pub owner: Signer<'info>,
}

#[event]
pub struct MachineEvent {
    pub owner: Pubkey,
    pub uuid: [u8; 16],
}
