use anchor_lang::prelude::*;
use crate::state::order::*;

pub fn migrate_order_new(ctx: Context<MigrationOrderNew>) -> Result<()> {
    let order = &mut ctx.accounts.order;
    let order_new = &mut ctx.accounts.order_new;
    order_new.order_id = order.order_id;
    order_new.buyer = order.buyer;
    order_new.seller = order.seller;
    order_new.machine_id = order.machine_id;
    order_new.price = order.price;
    order_new.duration = order.duration;
    order_new.total = order.total;
    order_new.metadata = order.metadata.clone();
    order_new.status = match order.status {
        OrderStatus::Preparing => OrderStatus::Training,
        OrderStatus::Training => OrderStatus::Completed,
        OrderStatus::Completed => OrderStatus::Failed,
        _ => OrderStatus::Refunded,
    };
    order_new.order_time = order.order_time;
    order_new.start_time = order.order_time;
    order_new.refund_time = order.refund_time;

    Ok(())
}

pub fn migrate_order_rename(ctx: Context<MigrationOrderRename>) -> Result<()> {
    let order = &mut ctx.accounts.order;
    let order_new = &mut ctx.accounts.order_new;
    order.order_id = order_new.order_id;
    order.buyer = order_new.buyer;
    order.seller = order_new.seller;
    order.machine_id = order_new.machine_id;
    order.price = order_new.price;
    order.duration = order_new.duration;
    order.total = order_new.total;
    order.metadata = order_new.metadata.clone();
    order.status = order_new.status.clone();
    order.order_time = order_new.order_time;
    order.start_time = order_new.order_time;
    order.refund_time = order_new.refund_time;

    Ok(())
}

#[derive(Accounts)]
pub struct MigrationOrderNew<'info> {
    #[account(
        mut,
        close = signer
    )]
    pub order: Account<'info, Order>,

    #[account(
        init,
        seeds = [b"order-new", order.buyer.as_ref(), order.order_id.as_ref()],
        bump,
        payer = signer,
        space = 8 + OrderNew::INIT_SPACE
    )]
    pub order_new: Account<'info, OrderNew>,

    #[account(mut)]
    pub signer: Signer<'info>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct MigrationOrderRename<'info> {
    #[account(
        mut,
        close = signer
    )]
    pub order_new: Account<'info, OrderNew>,

    #[account(
        init,
        seeds = [b"order", order_new.buyer.as_ref(), order_new.order_id.as_ref()],
        bump,
        payer = signer,
        space = 8 + Order::INIT_SPACE
    )]
    pub order: Account<'info, Order>,

    #[account(mut)]
    pub signer: Signer<'info>,

    pub system_program: Program<'info, System>,
}
