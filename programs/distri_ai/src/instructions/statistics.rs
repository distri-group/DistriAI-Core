use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{transfer_checked, Mint, Token, TokenAccount, TransferChecked},
};
use crate::pubkeys::*;
use crate::state::statistics::*;

pub fn report_ai_model_dataset_reward(ctx: Context<ReportAiModelDatasetReward>, amount: u64) -> Result<()> {
    let statistics_owner = &mut ctx.accounts.statistics_owner;
    statistics_owner.ai_model_dataset_reward_claimable = statistics_owner.ai_model_dataset_reward_claimable.saturating_add(amount);

    emit!(StatisticsEvent {
        owner: statistics_owner.owner,
    });
    Ok(())
}

pub fn claim_ai_model_dataset_reward(ctx: Context<ClaimAiModelDatasetReward>) -> Result<()> {
    let statistics_owner = &mut ctx.accounts.statistics_owner;

    // Transfer token from reward pool to owner
    let mint_key = ctx.accounts.mint.key();
    let signer: &[&[&[u8]]] = &[&[b"reward-pool", mint_key.as_ref(), &[ctx.bumps.reward_pool]]];
    let cpi_context = CpiContext::new_with_signer(
        ctx.accounts.token_program.to_account_info(),
        TransferChecked {
            from: ctx.accounts.reward_pool.to_account_info(),
            mint: ctx.accounts.mint.to_account_info(),
            to: ctx.accounts.owner_ata.to_account_info(),
            authority: ctx.accounts.reward_pool.to_account_info(),
        },
        signer,
    );
    transfer_checked(cpi_context, statistics_owner.ai_model_dataset_reward_claimable, ctx.accounts.mint.decimals)?;

    statistics_owner.ai_model_dataset_reward_claimed = statistics_owner.ai_model_dataset_reward_claimed
        .saturating_add(statistics_owner.ai_model_dataset_reward_claimable);
    statistics_owner.ai_model_dataset_reward_claimable = 0;

    emit!(StatisticsEvent {
        owner: statistics_owner.owner,
    });
    Ok(())
}

#[derive(Accounts)]
pub struct ReportAiModelDatasetReward<'info> {
    #[account()]
    pub owner: AccountInfo<'info>,

    #[account(
        mut,
        seeds = [b"statistics", owner.key().as_ref()],
        bump
    )]
    pub statistics_owner: Account<'info, Statistics>,

    #[account(
        mut,
        address = admin::ID
    )]
    pub admin: Signer<'info>,
}

#[derive(Accounts)]
pub struct ClaimAiModelDatasetReward<'info> {
    #[account(mut)]
    pub owner: Signer<'info>,

    #[account(
        init_if_needed,
        payer = owner,
        associated_token::mint = mint,
        associated_token::authority = owner
    )]
    pub owner_ata: Account<'info, TokenAccount>,

    #[account(
        mut,
        seeds = [b"statistics", owner.key().as_ref()],
        bump,
    )]
    pub statistics_owner: Account<'info, Statistics>,

    #[account(
        mut,
        seeds = [b"reward-pool", mint.key().as_ref()],
        bump
    )]
    pub reward_pool: Account<'info, TokenAccount>,

    #[account(
        address = dist_token::ID
    )]
    pub mint: Account<'info, Mint>,
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
}

#[event]
pub struct StatisticsEvent {
    pub owner: Pubkey,
}
