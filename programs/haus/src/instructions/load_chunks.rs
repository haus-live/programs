use crate::LoadChunks;
use crate::LoadChunksArgs;
use crate::constants;

use crate::CErrorCode;

use anchor_lang::prelude::*;
use mpl_core::instructions::UpdateV1CpiBuilder;

pub fn load_chunks(ctx: Context<LoadChunks>, args: LoadChunksArgs) -> Result<()> {
    msg!("loading chunks to core asset uri");

    let event = &ctx.accounts.event;
    require!(event.chunk_uploader == ctx.accounts.authority.key(), CErrorCode::InvalidOwner);
    require!(event.realtime_asset == ctx.accounts.realtime_asset.key(), CErrorCode::EventNotFound);

    UpdateV1CpiBuilder::new(&ctx.accounts.mpl_core_program.to_account_info())
        .asset(&ctx.accounts.realtime_asset.to_account_info())
        .payer(&ctx.accounts.authority.to_account_info())
        .authority(Some(ctx.accounts.event.to_account_info().as_ref()))
        .new_uri(args.uri)
        .system_program(&ctx.accounts.system_program.to_account_info())
        .invoke_signed(&[&[
            constants::EVENT_SEED,
            ctx.accounts.realtime_asset.key().as_ref(),
            &[ctx.bumps.event]
        ]])?;

    Ok(())
}
