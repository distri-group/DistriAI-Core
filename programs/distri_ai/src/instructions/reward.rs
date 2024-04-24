use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{transfer_checked, Mint, Token, TokenAccount, TransferChecked},
};
use crate::dist_token;
use crate::errors::DistriAIError;
use crate::state::machine::*;
use crate::state::reward::*;

pub fn reward_pool_deposit(ctx: Context<RewardPoolDeposit>, amount: u64) -> Result<()> {
    let cpi_context = CpiContext::new(
        ctx.accounts.token_program.to_account_info(),
        TransferChecked {
            from: ctx.accounts.signer_ata.to_account_info(),
            mint: ctx.accounts.mint.to_account_info(),
            to: ctx.accounts.reward_pool.to_account_info(),
            authority: ctx.accounts.signer.to_account_info(),
        },
    );
    transfer_checked(cpi_context, amount, ctx.accounts.mint.decimals)?;
    Ok(())
}

pub fn claim(ctx: Context<Claim>, period: u32) -> Result<()> {
    require_gt!(
        Reward::current_period()?,
        period,
        DistriAIError::InvalidPeriod
    );

    let reward = &ctx.accounts.reward;
    let reward_machine = &mut ctx.accounts.reward_machine;
    require!(!reward_machine.claimed, DistriAIError::RepeatClaim);
    reward_machine.claimed = true;

    let machine = &mut ctx.accounts.machine;
    machine.claimed_periodic_rewards = machine.claimed_periodic_rewards.saturating_add(reward.unit_periodic_reward);

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
    transfer_checked(cpi_context, reward.unit_periodic_reward, ctx.accounts.mint.decimals)?;

    emit!(RewardEvent {
        period: reward_machine.period,
        owner: reward_machine.owner,
        machine_id: reward_machine.machine_id,
    });
    Ok(())
}

#[derive(Accounts)]
pub struct RewardPoolDeposit<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,

    #[account(
        mut,
        associated_token::mint = mint,
        associated_token::authority = signer
    )]
    pub signer_ata: Account<'info, TokenAccount>,

    #[account(
        init_if_needed,
        seeds = [b"reward-pool", mint.key().as_ref()],
        bump,
        payer = signer,
        token::mint = mint,
        token::authority = reward_pool
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

#[derive(Accounts)]
#[instruction(period: u32)]
pub struct Claim<'info> {
    #[account(
        mut,
        has_one = owner
    )]
    pub machine: Account<'info, Machine>,

    #[account(
        seeds = [b"reward", period.to_le_bytes().as_ref()],
        bump
    )]
    pub reward: Account<'info, Reward>,

    #[account(
        mut,
        seeds = [b"reward-machine", period.to_le_bytes().as_ref(), owner.key().as_ref(), machine.uuid.as_ref()],
        bump
    )]
    pub reward_machine: Account<'info, RewardMachine>,

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
pub struct RewardEvent {
    pub period: u32,
    pub owner: Pubkey,
    pub machine_id: [u8; 16],
}
