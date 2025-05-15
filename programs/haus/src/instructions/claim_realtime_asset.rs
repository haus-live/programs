use crate::constants;
use crate::ClaimRealtimeAsset;
use crate::CErrorCode;

use anchor_lang::prelude::*;
use mpl_core::instructions::TransferV1CpiBuilder;

// Transfers the realtime asset to the leader if they meet the threshold, otherwise to the authority.
pub fn claim_realtime_asset(ctx: Context<ClaimRealtimeAsset>) -> Result<()> {
    msg!("claiming realtime asset");

    let event = &ctx.accounts.event;
    let current_time = Clock::get().unwrap().unix_timestamp;

    require!(current_time > event.end_timestamp, CErrorCode::EventNotEnded);

    let recipient = if event.tipping_leader.is_some() && event.tipping_leader_total >= event.reserve_price {
        event.tipping_leader.unwrap()
    } else {
        event.authority
    };

    // TODO: remove this line, transfer to the recipient anyways (?)
    require!(recipient.key() == ctx.accounts.authority.key(), CErrorCode::InvalidOwner);

    // TODO: event is the authority

    // Transfer the asset using Metaplex Core's transfer instruction
    TransferV1CpiBuilder::new(&ctx.accounts.mpl_core_program.to_account_info())
        .asset(&ctx.accounts.realtime_asset.to_account_info())
        .authority(Some(&ctx.accounts.event.to_account_info()))
        .new_owner(&ctx.accounts.authority.to_account_info())
        .payer(&ctx.accounts.authority.to_account_info())
        .invoke_signed(&[&[
            constants::EVENT_SEED,
            ctx.accounts.realtime_asset.key().as_ref(),
            &[ctx.bumps.event]
        ]])?;

    msg!("Asset {} transferred to: {}", event.realtime_asset, recipient);
    msg!("realtime asset claimed");

    Ok(())
}
