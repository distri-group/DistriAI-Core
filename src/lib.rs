use anchor_lang::prelude::*;
use instructions::*;

pub mod errors;
pub mod instructions;
pub mod state;

// This is your program's public key and it will update
// automatically when you build the project.
declare_id!("HF4aT6sho2zTySB8nEeN5ThMvDGtGVRrH3jeBvxFNxit");

#[program]
mod distri_ai {
    use super::*;

    pub fn add_machine(ctx: Context<AddMachine>, uuid: [u8; 16], metadata: String) -> Result<()> {
        instructions::machine::add_machine(ctx, uuid, metadata)
    }

    pub fn remove_machine(ctx: Context<RemoveMachine>) -> Result<()> {
        instructions::machine::remove_machine(ctx)
    }

    pub fn make_offer(
        ctx: Context<MakeOffer>,
        price: u64,
        max_duration: u32,
        disk: u32,
    ) -> Result<()> {
        instructions::machine::make_offer(ctx, price, max_duration, disk)
    }

    pub fn cancel_offer(ctx: Context<CancelOffer>) -> Result<()> {
        instructions::machine::cancel_offer(ctx)
    }

    pub fn place_order(
        ctx: Context<PlaceOrder>,
        order_id: [u8; 16],
        duration: u32,
        metadata: String,
    ) -> Result<()> {
        instructions::order::place_order(ctx, order_id, duration, metadata)
    }

    pub fn renew_order(ctx: Context<RenewOrder>, duration: u32) -> Result<()> {
        instructions::order::renew_order(ctx, duration)
    }

    pub fn order_completed(
        ctx: Context<OrderCompleted>,
        metadata: String,
        score: u8,
    ) -> Result<()> {
        instructions::order::order_completed(ctx, metadata, score)
    }

    pub fn order_failed(ctx: Context<OrderFailed>, metadata: String) -> Result<()> {
        instructions::order::order_failed(ctx, metadata)
    }

    pub fn remove_order(ctx: Context<RemoveOrder>) -> Result<()> {
        instructions::order::remove_order(ctx)
    }
}
