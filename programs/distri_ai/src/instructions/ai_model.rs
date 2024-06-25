use anchor_lang::prelude::*;
use crate::errors::DistriAIError;
use crate::state::ai_model::*;

/// Creates an AI model account.
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
     // Assigning provided details to the new AI model account
    ai_model.owner = ctx.accounts.owner.key();
    ai_model.name = name;
    ai_model.framework = framework;
    ai_model.license = license;
    ai_model.type1 = type1;
    ai_model.type2 = type2;
    ai_model.tags = tags;
    // Setting creation and update timestamps to the current time
    let now_ts = Clock::get()?.unix_timestamp;
    ai_model.create_time = now_ts;
    ai_model.update_time = now_ts;

    // Emitting an event to log the creation of the AI model
    emit!(AiModelEvent {
        owner: ai_model.owner,
        name: ai_model.name.clone(),
    });
    Ok(())
}

/// Removes an AI model.
pub fn remove_ai_model(ctx: Context<RemoveAiModel>) -> Result<()> {
    let ai_model = &mut ctx.accounts.ai_model;

    // Emit an event carrying the AI model's owner and name to log the removal action.
    emit!(AiModelEvent {
        owner: ai_model.owner,
        name: ai_model.name.clone(),
    });
    Ok(())
}

#[derive(Accounts)]
#[instruction(name: String)]
// Defines the `CreateAiModel` structure for creating a new AI model account within a Solana program.
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

     // The signer (owner) account, which must be mutable to fund the new account creation.
    #[account(mut)]
    pub owner: Signer<'info>,

     // The system program, used for basic account operations on Solana.
    pub system_program: Program<'info, System>,
}

// Defines a structure `RemoveAiModel` for handling the logic of removing an AI model.
#[derive(Accounts)]
pub struct RemoveAiModel<'info> {
    #[account(
        mut,
        has_one = owner,
        close = owner
    )]
    pub ai_model: Account<'info, AiModel>,

     // Declares the `owner` account, which is a signer and signifies ownership rights for the model operations.
    #[account(mut)]
    pub owner: Signer<'info>,
}

// Defines an event structure `AiModelEvent` to log events related to AI models.
#[event]
pub struct AiModelEvent {
    pub owner: Pubkey,
    pub name: String,
}
