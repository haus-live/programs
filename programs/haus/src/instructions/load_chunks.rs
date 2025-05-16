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
        .new_uri(args.uri)
        .invoke_signed(&[&[
            constants::EVENT_SEED,
            ctx.accounts.realtime_asset.key().as_ref(),
            &[ctx.bumps.event]
        ]])?;

    Ok(())
}
