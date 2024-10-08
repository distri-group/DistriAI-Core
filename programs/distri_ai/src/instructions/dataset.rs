use anchor_lang::prelude::*;
use crate::errors::DistriAIError;
use crate::state::dataset::*;

// Define a public function to create a new dataset
pub fn create_dataset(
    ctx: Context<CreateDataset>,
    name: String,
    scale: u8,
    license: u8,
    type1: u8,
    type2: u8,
    tags: String,
) -> Result<()> {
    require_gte!(
        Dataset::NAME_MAX_LENGTH,
        name.len(),
        DistriAIError::StringTooLong
    );
    require_gte!(
        Dataset::TAGS_MAX_LENGTH,
        tags.len(),
        DistriAIError::StringTooLong
    );

    let dataset = &mut ctx.accounts.dataset;
    dataset.owner = ctx.accounts.owner.key();
    dataset.name = name;
    dataset.scale = scale;
    dataset.license = license;
    dataset.type1 = type1;
    dataset.type2 = type2;
    dataset.tags = tags;
    let now_ts = Clock::get()?.unix_timestamp;
    dataset.create_time = now_ts;
    dataset.update_time = now_ts;

    emit!(DatasetEvent {
        owner: dataset.owner,
        name: dataset.name.clone(),
    });
    Ok(())
}

// Define a public function `remove_dataset` that takes a `Context` with a `RemoveDataset` instruction.
pub fn remove_dataset(ctx: Context<RemoveDataset>) -> Result<()> {
    let dataset = &mut ctx.accounts.dataset;

    emit!(DatasetEvent {
        owner: dataset.owner,
        name: dataset.name.clone(),
    });
    Ok(())
}

#[derive(Accounts)]
#[instruction(name: String)]
// Define a public struct `CreateDataset` with a lifetime parameter `'info` for use in Solana programs.
pub struct CreateDataset<'info> {
    #[account(
        init,
        seeds = [
            b"dataset",
            owner.key().as_ref(),
            &anchor_lang::solana_program::hash::hash(name.as_bytes()).to_bytes(),
        ],
        bump,
        payer = owner,
        space = 8 + Dataset::INIT_SPACE
    )]
    pub dataset: Account<'info, Dataset>,

    #[account(mut)]
    pub owner: Signer<'info>,

    pub system_program: Program<'info, System>,
}

// The `RemoveDataset` struct is a collection of accounts that will be used in a Solana program instruction.
// It is derived from the `Accounts` macro, which provides functionality for automatically handling account relationships and transactions.
#[derive(Accounts)]
pub struct RemoveDataset<'info> {
    #[account(
        mut,
        has_one = owner,
        close = owner
    )]
    pub dataset: Account<'info, Dataset>,

    #[account(mut)]
    pub owner: Signer<'info>,
}

#[event]
pub struct DatasetEvent {
    pub owner: Pubkey,
    pub name: String,
}
