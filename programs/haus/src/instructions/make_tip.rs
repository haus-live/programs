use crate::MakeTip;
use crate::MakeTipArgs;
use crate::CErrorCode;
// use crate::NftVerifierError;

use anchor_lang::prelude::*;
use anchor_lang::solana_program::system_instruction;
// use mpl_token_metadata::accounts::Metadata as MetadataAccount;

pub fn make_tip(ctx: Context<MakeTip>, args: MakeTipArgs) -> Result<()> {
    msg!("making tip");

    let current_time = Clock::get().unwrap().unix_timestamp;
    let event = &mut ctx.accounts.event;

    // Check timestamps of the event
    require!(current_time >= event.begin_timestamp, CErrorCode::EventNotStarted);
    require!(current_time <= event.end_timestamp, CErrorCode::EventEnded);

    // let token_account = &ctx.accounts.token_account;
    // let mint = &ctx.accounts.mint;
    // let metadata_account = &ctx.accounts.metadata_account;
    // let expected_collection_mint = &ctx.accounts.expected_collection_mint;

    // // Verify token account ownership and amount
    // require!(token_account.owner == ctx.accounts.authority.key(), NftVerifierError::InvalidOwner);
    // require!(token_account.mint == mint.key(), NftVerifierError::InvalidMint);
    // require!(token_account.amount == 1, NftVerifierError::InvalidAmount);

    // let (metadata_pda, _bump) = MetadataAccount::find_pda(&mint.key());
    // require!(metadata_pda == metadata_account.key(), NftVerifierError::InvalidMetadataAccount);

    // // Deserialize metadata
    // let metadata = MetadataAccount::try_from(metadata_account)?;
    
    // // Verify collection
    // match metadata.collection {
    //     Some(collection) => {
    //         require!(collection.verified, NftVerifierError::UnverifiedCollection);
    //         require!(collection.key == expected_collection_mint.key(), NftVerifierError::InvalidCollection);
    //     },
    //     None => return Err(NftVerifierError::NoCollectionData.into()),
    // }

    // Update users tipping account and retrieve the total tipped amount
    let tipping_calculator = &mut ctx.accounts.tipping_calculator;
    let authority_total_tipped_amount = tipping_calculator.process_tip(&args.amount);

    // Update the tipping leader pubkey and amount
    if event.tipping_leader.is_none() || *authority_total_tipped_amount > event.tipping_leader_total {
        event.tipping_leader = Some(ctx.accounts.signer.key());
        event.tipping_leader_total = *authority_total_tipped_amount;
    } else if event.tipping_leader == Some(ctx.accounts.signer.key()) {
        event.tipping_leader_total = *authority_total_tipped_amount;
    }

    // Transfer SOL to the event account
    let transfer_instruction = system_instruction::transfer(
        &ctx.accounts.signer.key(),
        &event.key(),
        args.amount,
    );
    anchor_lang::solana_program::program::invoke(
        &transfer_instruction,
        &[
            ctx.accounts.signer.to_account_info(),
            event.to_account_info(),
            ctx.accounts.system_program.to_account_info(),
        ],
    )?;

    msg!(
        "Payment of {} lamports made by {} to event: {}, new total: {}", 
        args.amount, 
        ctx.accounts.signer.key(), 
        ctx.accounts.event.key(), 
        *authority_total_tipped_amount
    );

    msg!("tip made");

    Ok(())
}
