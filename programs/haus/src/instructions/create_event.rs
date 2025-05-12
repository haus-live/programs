use crate::CreateEvent;
use crate::CreateEventArgs;
use crate::CErrorCode;
use crate::constants;

use anchor_lang::prelude::*;
use mpl_core::instructions::CreateV2CpiBuilder;

pub fn create_event(ctx: Context<CreateEvent>, args: CreateEventArgs) -> Result<()> {
    msg!("creating event");

    let event_duration = args.end_timestamp - args.begin_timestamp;
    
    require!(
        event_duration > 0 && 
        event_duration % constants::FIFTEEN_MINUTES_IN_SECONDS == 0 &&
        event_duration < constants::MAX_EVENT_DURATION_IN_SECONDS, 
        CErrorCode::EventDurationInvalid
    );

    let event = &mut ctx.accounts.event;

    event.authority = ctx.accounts.authority.key();
    event.realtime_asset = ctx.accounts.realtime_asset.key();
    event.begin_timestamp = args.begin_timestamp;
    event.end_timestamp = args.end_timestamp;
    event.ticket_collection = args.ticket_collection;
    event.reserve_price = args.reserve_price;
    event.event_type = args.event_type;
    // TODO: maybe use .. operator to shorten the code above

    CreateV2CpiBuilder::new(&ctx.accounts.mpl_core_program.to_account_info())
        .asset(&ctx.accounts.realtime_asset.to_account_info())
        .collection(None.as_ref())
        .authority(Some(ctx.accounts.event.to_account_info().as_ref()))
        .payer(ctx.accounts.authority.as_ref())
        .owner(Some(&ctx.accounts.authority.as_ref())) 
        .system_program(&ctx.accounts.system_program.to_account_info())
        .name(args.name)
        .uri(args.uri)
        .invoke()?;

    msg!("event created");

    Ok(())
}
