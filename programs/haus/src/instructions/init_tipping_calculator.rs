use crate::InitTippingCalculator;
use crate::CErrorCode;

use anchor_lang::prelude::*;

pub fn init_tipping_calculator(ctx: Context<InitTippingCalculator>) -> Result<()> {
    let event = &ctx.accounts.event;
    let current_time = Clock::get().unwrap().unix_timestamp;
    require!(current_time >= event.begin_timestamp, CErrorCode::EventNotStarted);
    require!(current_time <= event.end_timestamp, CErrorCode::EventEnded);
    let tipping_calculator = &mut ctx.accounts.tipping_calculator;
    tipping_calculator.total_tipped_amount = 0;
    tipping_calculator.authority = ctx.accounts.signer.key();
    Ok(())
}
