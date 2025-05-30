use crate::WithdrawTips;
use crate::CErrorCode;

use anchor_lang::prelude::*;

pub fn withdraw_tips(ctx: Context<WithdrawTips>) -> Result<()> {
    msg!("withdrawing tips");

    let current_time = Clock::get().unwrap().unix_timestamp;
    let event = &ctx.accounts.event;

    require!(event.end_timestamp < current_time, CErrorCode::EventEnded);
    require!(event.authority == ctx.accounts.authority.key(), CErrorCode::InvalidOwner);
    require!(event.realtime_asset == ctx.accounts.realtime_asset.key(), CErrorCode::EventNotFound);

    // Transfer SOL to the event authority
    let init_balance = event.get_lamports();
    event.sub_lamports(init_balance)?;
    ctx.accounts.authority.add_lamports(init_balance)?;

    msg!("tips withdrawn");

    Ok(())
}
