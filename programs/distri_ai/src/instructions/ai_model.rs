use anchor_lang::prelude::*;
use crate::errors::DistriAIError;
use crate::state::ai_model::*;
use crate::state::statistics::*;

pub fn create_ai_model(
    ctx: Context<CreateAiModel>,
    name: String,
    framework: u8,
    license: u8,
    type1: u8,
    type2: u8,
    tags: String,
) -> Result<()> {
    require_gte!(
        AiModel::NAME_MAX_LENGTH,
        name.len(),
        DistriAIError::StringTooLong
    );
    require_gte!(
        AiModel::TAGS_MAX_LENGTH,
        tags.len(),
        DistriAIError::StringTooLong
    );

    let ai_model = &mut ctx.accounts.ai_model;
    ai_model.owner = ctx.accounts.owner.key();
    ai_model.name = name;
    ai_model.framework = framework;
    ai_model.license = license;
    ai_model.type1 = type1;
    ai_model.type2 = type2;
    ai_model.tags = tags;
    let now_ts = Clock::get()?.unix_timestamp;
    ai_model.create_time = now_ts;
    ai_model.update_time = now_ts;

    emit!(AiModelEvent {
        owner: ai_model.owner,
        name: ai_model.name.clone(),
    });
    Ok(())
}

pub fn remove_ai_model(ctx: Context<RemoveAiModel>) -> Result<()> {
    let ai_model = &mut ctx.accounts.ai_model;

    emit!(AiModelEvent {
        owner: ai_model.owner,
        name: ai_model.name.clone(),
    });
    Ok(())
}

#[derive(Accounts)]
#[instruction(name: String)]
pub struct CreateAiModel<'info> {
    #[account(
        init,
        seeds = [
            b"ai_model",
            owner.key().as_ref(),
            &anchor_lang::solana_program::hash::hash(name.as_bytes()).to_bytes(),
        ],
        bump,
        payer = owner,
        space = 8 + AiModel::INIT_SPACE
    )]
    pub ai_model: Account<'info, AiModel>,

    #[account(mut)]
    pub owner: Signer<'info>,

    #[account(
        init_if_needed,
        seeds = [b"statistics", owner.key().as_ref()],
        bump,
        payer = owner,
        space = 8 + Statistics::INIT_SPACE
    )]
    pub statistics_owner: Account<'info, Statistics>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct RemoveAiModel<'info> {
    #[account(
        mut,
        has_one = owner,
        close = owner
    )]
    pub ai_model: Account<'info, AiModel>,

    #[account(mut)]
    pub owner: Signer<'info>,
}

#[event]
pub struct AiModelEvent {
    pub owner: Pubkey,
    pub name: String,
}
