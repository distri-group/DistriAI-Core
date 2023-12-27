use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{transfer_checked, Mint, Token, TokenAccount, TransferChecked},
};
use solana_program::{pubkey, pubkey::Pubkey};
use crate::errors::DistriAIError;
use crate::state::machine::*;
use crate::state::order::*;

const MINT_PUBKEY: Pubkey = pubkey!("896KfVVY6VRGQs1d9CKLnKUEgXXCCJcEEg7LwSK84vWE");

pub fn place_order(
    ctx: Context<PlaceOrder>,
    order_id: [u8; 16],
    duration: u32,
    metadata: String,
) -> Result<()> {
    require_gte!(
        Order::METADATA_MAX_LENGTH,
        metadata.len(),
        DistriAIError::StringTooLong
    );

    let machine = &mut ctx.accounts.machine;
    require!(
        machine.status == MachineStatus::ForRent,
        DistriAIError::IncorrectStatus
    );
    require_gte!(
        machine.max_duration,
        duration,
        DistriAIError::DurationTooMuch
    );

    // Transfer token from buyer to vault
    let total = machine.price.saturating_mul(duration.into());
    let cpi_context = CpiContext::new(
        ctx.accounts.token_program.to_account_info(),
        TransferChecked {
            from: ctx.accounts.buyer_ata.to_account_info(),
            mint: ctx.accounts.mint.to_account_info(),
            to: ctx.accounts.vault.to_account_info(),
            authority: ctx.accounts.buyer.to_account_info(),
        },
    );
    transfer_checked(cpi_context, total, ctx.accounts.mint.decimals)?;

    machine.status = MachineStatus::Renting;

    let order = &mut ctx.accounts.order;
    order.order_id = order_id;
    order.buyer = ctx.accounts.buyer.key();
    order.seller = machine.owner;
    order.machine_id = machine.uuid;
    order.price = machine.price;
    order.duration = duration;
    order.total = total;
    order.metadata = metadata;
    order.status = OrderStatus::Training;

    emit!(OrderEvent {
        order_id: order.order_id,
        buyer: order.buyer,
        seller: order.seller,
        machine_id: order.machine_id,
    });
    Ok(())
}

pub fn renew_order(ctx: Context<RenewOrder>, duration: u32) -> Result<()> {
    let order = &mut ctx.accounts.order;
    require!(
        order.status == OrderStatus::Training,
        DistriAIError::IncorrectStatus
    );

    let new_duration = order.duration.saturating_add(duration);
    let machine = &ctx.accounts.machine;
    require_gte!(
        machine.max_duration,
        new_duration,
        DistriAIError::DurationTooMuch
    );

    let total = order.price.saturating_mul(duration.into());

    order.duration = new_duration;
    order.total = order.total.saturating_add(total);

    emit!(OrderEvent {
        order_id: order.order_id,
        buyer: order.buyer,
        seller: order.seller,
        machine_id: order.machine_id,
    });

    // Transfer token from buyer to vault
    let total = machine.price.saturating_mul(duration.into());
    let cpi_context = CpiContext::new(
        ctx.accounts.token_program.to_account_info(),
        TransferChecked {
            from: ctx.accounts.buyer_ata.to_account_info(),
            mint: ctx.accounts.mint.to_account_info(),
            to: ctx.accounts.vault.to_account_info(),
            authority: ctx.accounts.buyer.to_account_info(),
        },
    );
    transfer_checked(cpi_context, total, ctx.accounts.mint.decimals)?;
    Ok(())
}

pub fn order_completed(ctx: Context<OrderCompleted>, metadata: String, score: u8) -> Result<()> {
    require_gte!(
        Order::METADATA_MAX_LENGTH,
        metadata.len(),
        DistriAIError::StringTooLong
    );

    let order = &mut ctx.accounts.order;
    require!(
        order.status == OrderStatus::Training,
        DistriAIError::IncorrectStatus
    );
    order.metadata = metadata;
    order.status = OrderStatus::Completed;

    let machine = &mut ctx.accounts.machine;
    require!(
        machine.status == MachineStatus::Renting,
        DistriAIError::IncorrectStatus
    );
    machine.status = MachineStatus::ForRent;
    machine.completed_count = machine.completed_count.saturating_add(1);
    machine.score = score;

    // Transfer token from vault to seller
    let mint_key = ctx.accounts.mint.key();
    let signer: &[&[&[u8]]] = &[&[b"vault", mint_key.as_ref(), &[ctx.bumps.vault]]];
    let cpi_context = CpiContext::new_with_signer(
        ctx.accounts.token_program.to_account_info(),
        TransferChecked {
            from: ctx.accounts.vault.to_account_info(),
            mint: ctx.accounts.mint.to_account_info(),
            to: ctx.accounts.seller_ata.to_account_info(),
            authority: ctx.accounts.vault.to_account_info(),
        },
        signer,
    );
    transfer_checked(cpi_context, order.total, ctx.accounts.mint.decimals)?;

    emit!(OrderEvent {
        order_id: order.order_id,
        buyer: order.buyer,
        seller: order.seller,
        machine_id: order.machine_id,
    });
    Ok(())
}

pub fn order_failed(ctx: Context<OrderFailed>, metadata: String) -> Result<()> {
    require_gte!(
        Order::METADATA_MAX_LENGTH,
        metadata.len(),
        DistriAIError::StringTooLong
    );

    let order = &mut ctx.accounts.order;
    require!(
        order.status == OrderStatus::Training,
        DistriAIError::IncorrectStatus
    );
    order.metadata = metadata;
    order.status = OrderStatus::Failed;

    let machine = &mut ctx.accounts.machine;
    require!(
        machine.status == MachineStatus::Renting,
        DistriAIError::IncorrectStatus
    );
    machine.status = MachineStatus::ForRent;
    machine.failed_count = machine.failed_count.saturating_add(1);

    // Transfer token from vault to seller
    let mint_key = ctx.accounts.mint.key();
    let signer: &[&[&[u8]]] = &[&[b"vault", mint_key.as_ref(), &[ctx.bumps.vault]]];
    let cpi_context = CpiContext::new_with_signer(
        ctx.accounts.token_program.to_account_info(),
        TransferChecked {
            from: ctx.accounts.vault.to_account_info(),
            mint: ctx.accounts.mint.to_account_info(),
            to: ctx.accounts.buyer_ata.to_account_info(),
            authority: ctx.accounts.vault.to_account_info(),
        },
        signer,
    );
    transfer_checked(cpi_context, order.total, ctx.accounts.mint.decimals)?;

    emit!(OrderEvent {
        order_id: order.order_id,
        buyer: order.buyer,
        seller: order.seller,
        machine_id: order.machine_id,
    });
    Ok(())
}

pub fn remove_order(ctx: Context<RemoveOrder>) -> Result<()> {
    let order = &mut ctx.accounts.order;
    require!(
        order.status != OrderStatus::Training,
        DistriAIError::IncorrectStatus
    );

    emit!(OrderEvent {
        order_id: order.order_id,
        buyer: order.buyer,
        seller: order.seller,
        machine_id: order.machine_id,
    });
    Ok(())
}

#[derive(Accounts)]
#[instruction(order_id: [u8; 16])]
pub struct PlaceOrder<'info> {
    #[account(mut)]
    pub machine: Account<'info, Machine>,

    #[account(
        init,
        seeds = [b"order", buyer.key().as_ref(), order_id.as_ref()],
        bump,
        payer = buyer,
        space = 8 + Order::MAXIMUM_SIZE
    )]
    pub order: Account<'info, Order>,

    #[account(mut)]
    pub buyer: Signer<'info>,

    #[account(
        mut,
        associated_token::mint = mint,
        associated_token::authority = buyer
    )]
    pub buyer_ata: Account<'info, TokenAccount>,

    #[account(
        init_if_needed,
        seeds = [b"vault", mint.key().as_ref()],
        bump,
        payer = buyer,
        token::mint = mint,
        token::authority = vault
    )]
    pub vault: Account<'info, TokenAccount>,
    
    #[account(
        address = MINT_PUBKEY
    )]
    pub mint: Account<'info, Mint>,
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct RenewOrder<'info> {
    #[account(
        mut,
        constraint = machine.uuid == order.machine_id && machine.owner == order.seller
    )]
    pub machine: Account<'info, Machine>,

    #[account(
        mut,
        has_one = buyer
    )]
    pub order: Account<'info, Order>,

    #[account(mut)]
    pub buyer: Signer<'info>,

    #[account(
        mut,
        associated_token::mint = mint,
        associated_token::authority = buyer
    )]
    pub buyer_ata: Account<'info, TokenAccount>,

    #[account(
        mut,
        seeds = [b"vault", mint.key().as_ref()],
        bump
    )]
    pub vault: Account<'info, TokenAccount>,

    #[account(
        address = MINT_PUBKEY
    )]
    pub mint: Account<'info, Mint>,
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
}

#[derive(Accounts)]
pub struct OrderCompleted<'info> {
    #[account(
        mut,
        constraint = machine.uuid == order.machine_id && machine.owner == order.seller
    )]
    pub machine: Account<'info, Machine>,

    #[account(
        mut,
        has_one = seller
    )]
    pub order: Account<'info, Order>,

    #[account(mut)]
    pub seller: Signer<'info>,

    #[account(
        init_if_needed,
        payer = seller,
        associated_token::mint = mint,
        associated_token::authority = seller
    )]
    pub seller_ata: Account<'info, TokenAccount>,

    #[account(
        mut,
        seeds = [b"vault", mint.key().as_ref()],
        bump
    )]
    pub vault: Account<'info, TokenAccount>,

    #[account(
        address = MINT_PUBKEY
    )]
    pub mint: Account<'info, Mint>,
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct OrderFailed<'info> {
    #[account(
        mut,
        constraint = machine.uuid == order.machine_id && machine.owner == order.seller
    )]
    pub machine: Account<'info, Machine>,

    #[account(
        mut,
        has_one = seller
    )]
    pub order: Account<'info, Order>,

    #[account(mut)]
    pub seller: Signer<'info>,

    #[account(
        mut,
        associated_token::mint = mint,
        associated_token::authority = order.buyer
    )]
    pub buyer_ata: Account<'info, TokenAccount>,

    #[account(
        mut,
        seeds = [b"vault", mint.key().as_ref()],
        bump
    )]
    pub vault: Account<'info, TokenAccount>,

    #[account(
        address = MINT_PUBKEY
    )]
    pub mint: Account<'info, Mint>,
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
}

#[derive(Accounts)]
pub struct RemoveOrder<'info> {
    #[account(
        mut,
        has_one = buyer,
        close = buyer
    )]
    pub order: Account<'info, Order>,

    #[account(mut)]
    pub buyer: Signer<'info>,
}

#[event]
pub struct OrderEvent {
    pub order_id: [u8; 16],
    pub buyer: Pubkey,
    pub seller: Pubkey,
    pub machine_id: [u8; 16],
}
