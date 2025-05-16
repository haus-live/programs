use crate::CreateEvent;
use crate::CreateEventArgs;
use crate::CErrorCode;
use crate::constants;

use anchor_lang::prelude::*;
use mpl_core::instructions::CreateV1CpiBuilder;

pub fn create_event(ctx: Context<CreateEvent>, args: CreateEventArgs) -> Result<()> {
    msg!("creating event");

    let event_duration = args.end_timestamp - args.begin_timestamp;
    
    require!(
        event_duration > 0 && 
        event_duration % constants::FIFTEEN_MINUTES_IN_SECONDS == 0 &&
        event_duration <= constants::MAX_EVENT_DURATION_IN_SECONDS, 
        CErrorCode::EventDurationInvalid
    );

    let event = &mut ctx.accounts.event;
    event.authority = ctx.accounts.authority.key();
    event.realtime_asset = ctx.accounts.realtime_asset.key();
    event.begin_timestamp = args.begin_timestamp;
    event.end_timestamp = args.end_timestamp;
    event.ticket_collection = args.ticket_collection;
    event.reserve_price = args.reserve_price;
    event.art_category = args.art_category;
    event.chunk_uploader = args.chunk_uploader;

    CreateV1CpiBuilder::new(&ctx.accounts.mpl_core_program.to_account_info())
        .asset(&ctx.accounts.realtime_asset.to_account_info())
        .authority(Some(ctx.accounts.event.to_account_info().as_ref())) // Event is the authority
        .update_authority(Some(ctx.accounts.event.to_account_info().as_ref()))
        .owner(Some(ctx.accounts.event.to_account_info().as_ref()))
        .payer(ctx.accounts.authority.as_ref()) // Authority (signer) is the payer
        .system_program(&ctx.accounts.system_program.to_account_info())
        .name(args.name)
        .uri(args.uri)
        .invoke_signed(&[&[
            constants::EVENT_SEED,
            ctx.accounts.realtime_asset.key().as_ref(),
            &[ctx.bumps.event]]]
        )?;

    msg!("event created");

    Ok(())
}
