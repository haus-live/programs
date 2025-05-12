use crate::WithdrawTips;
use crate::CErrorCode;

use anchor_lang::prelude::*;
use anchor_lang::solana_program::system_instruction;

pub fn withdraw_tips(ctx: Context<WithdrawTips>) -> Result<()> {
    msg!("withdrawing tips");

    let current_time = Clock::get().unwrap().unix_timestamp;
    let event = &ctx.accounts.event;

    require!(event.end_timestamp < current_time, CErrorCode::EventEnded);
    require!(event.authority == ctx.accounts.authority.key(), CErrorCode::InvalidOwner);

    // Transfer SOL to the event authority
    let transfer_instruction = system_instruction::transfer(
        &event.key(),
        &ctx.accounts.authority.key(),
        event.get_lamports()
    );
    anchor_lang::solana_program::program::invoke(
        &transfer_instruction,
        &[
            event.to_account_info(),
            ctx.accounts.authority.to_account_info(),
            ctx.accounts.system_program.to_account_info()
        ],
    )?;

    msg!("tips withdrawn");

    Ok(())
}
