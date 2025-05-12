#![allow(unexpected_cfgs)]
/**
Create Event:
- Create Core Asset Collection 
- ticket price 
- event type? 

*/

use anchor_lang::prelude::*;
use anchor_lang::prelude::Pubkey;
use mpl_core::ID as MPL_CORE_ID;
use std::mem;
// use::mpl_token_metadata::ID as MPL_TOKEN_METADATA_ID;
// use::anchor_lang::solana_program::native_token;

// use mpl_core::instructions::{
//     CreateV2CpiBuilder, 
//     TransferV1CpiBuilder,
// };

use anchor_spl::token::{Mint, TokenAccount};

use mpl_token_metadata::accounts::{MasterEdition, Metadata as MetadataAccount};
use mpl_token_metadata::ID as MPL_TOKEN_METADATA_ID;

pub mod errors;

pub use errors::ErrorCode as CErrorCode;

declare_id!("EvNT111111111111111111111111111111111111111");

#[program]
pub mod event_factory {
    use anchor_lang::solana_program::system_instruction;

    use crate::instruction::Tip;

    use super::*;

    // pub fn create_event(
    //     ctx: Context<CreateEvent>,
    //     args: CreateEventArgs,
    // ) -> Result<()> {
        
    // }

    pub fn tip(ctx: Context<TippingCtx>, args: TippingArgs) -> Result<()> {
        let current_time = Clock::get().unwrap().unix_timestamp;
        let event = &mut ctx.accounts.event;

        // Check timestamps of the event
        require!(current_time >= event.begin_timestamp, CErrorCode::EventNotStarted);
        require!(current_time <= event.end_timestamp, CErrorCode::EventEnded);

        // TODO: Check token gated access to the event

        // Update users tipping account and retrieve the total tipped amount
        let tipping_calculator = &mut ctx.accounts.tipping_calculator;
        let authority_total_tipped_amount = tipping_calculator.process_tip(&args.amount);

        // Update the tipping leader pubkey and amount
        if event.tipping_leader.is_none() || *authority_total_tipped_amount > event.tipping_leader_total {
            event.tipping_leader = Some(ctx.accounts.authority.key());
            event.tipping_leader_total = *authority_total_tipped_amount;
        } else if event.tipping_leader == Some(ctx.accounts.authority.key()) {
            event.tipping_leader_total = *authority_total_tipped_amount;
        }

        let transfer_instruction = system_instruction::transfer(
            &ctx.accounts.authority.key(),
            &event.key(),
            args.amount,
        );
        anchor_lang::solana_program::program::invoke(
            &transfer_instruction,
            &[
                ctx.accounts.authority.to_account_info(),
                event.to_account_info(),
                ctx.accounts.system_program.to_account_info(),
            ],
        )?;

        msg!(
            "Payment of {} lamports made by {} to event: {}, new total: {}", 
            args.amount, 
            ctx.accounts.payer.key(), 
            ctx.accounts.event.key(), 
            *authority_total_tipped_amount
        );

        Ok(())
    }

    pub fn test(ctx: Context<TestCtx>, expected_collection: Pubkey) -> Result<()> {
        // ctx.accounts.mint.
        let (metadata_account, _) = MetadataAccount::find_pda(&ctx.accounts.mint.key());
        match metadata_account {
            expected_collection => {
                return Ok(());
            },
            _ => {
                return Err(CErrorCode::NoTicket.into());
            }
        }
        // Ok(())
    }
}

#[derive(Accounts)]
pub struct TippingCtx<'info> {
    #[account(
        mut,
        seeds = [b"event", authority.key().as_ref()],
        bump
    )]
    pub event: Account<'info, Event>,
    #[account(
        init_if_needed,
        payer = payer,
        space = 8 + mem::size_of::<TippingCalculator>(),
        seeds = [b"tipping_calculator", event.key().as_ref(), authority.key().as_ref()],
        bump
    )]
    pub tipping_calculator: Account<'info, TippingCalculator>,
    #[account(mut)]
    pub payer: Signer<'info>,
    pub authority: AccountInfo<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct TippingArgs {
    pub amount: u64,
}

#[derive(Accounts)]
pub struct TestCtx<'info> {
    /// The mint account of the NFT
    pub mint: Account<'info, Mint>,
    #[account(
        seeds = [b"metadata", MPL_TOKEN_METADATA_ID.as_ref(), mint.key().as_ref()],
        bump,
        seeds::program = MPL_TOKEN_METADATA_ID
    )]
    pub metadata: AccountInfo<'info>,
}

#[derive(AnchorSerialize, AnchorDeserialize, Debug)]
pub struct CreateEventArgs {
    name: String,
    uri: String,
    begin_timestamp: u64,

}

#[derive(Accounts, Debug)]
pub struct VerifyTicket<'info> {
    #[account(signer)]
    pub signer: Signer<'info>,

}

#[derive(Accounts)]
pub struct CreateEvent<'info> {
    #[account(mut)]
    pub asset: Signer<'info>,
    #[account(mut)]
    pub authority: Signer<'info>,
    #[account(mut)]
    pub payer: Signer<'info>,
    #[account(
        init,
        payer = payer,
        space = 8 + mem::size_of::<Event>(),
        seeds = [b"event", authority.key().as_ref()],
        bump
    )]
    pub event: Account<'info, Event>,
    pub system_program: Program<'info, System>,
    /// CHECK: Metaplex Core program
    #[account(address = MPL_CORE_ID)]
    pub mpl_core_program: UncheckedAccount<'info>,
}

#[account]
pub struct Event {
    /// The creator of the event
    pub authority: Pubkey,
    /// The Real Time Asset (Metaplex Core) representing the event
    pub realtime_asset: Pubkey,
    /// Start time of the event
    pub begin_timestamp: i64,
    /// End time of the event
    pub end_timestamp: i64,              
    /// The user with the highest total tipped amount
    pub tipping_leader: Option<Pubkey>,
    /// The higher total tipped amount
    pub tipping_leader_total: u128,       
    /// Minimum total tipped amount needed to claim the assets' ownership 
    pub reserve_price: u64,
    /// Mint address of the ticket collection
    pub mint_authority: Pubkey,
}

#[account]
pub struct TippingCalculator {
    /// Total tips made by the user
    pub total_tipped_amount: u128,
}

impl TippingCalculator {
    /// Increments the total tipped by the given amount
    pub fn process_tip(&mut self, amount: &u64) -> &u128 {
        self.total_tipped_amount += *amount as u128;
        &self.total_tipped_amount
    }
}

// todo: check event timestamps 15m, 30m, 45m, 1h 

pub fn bump(seeds: &[&[u8]], program_id: &Pubkey) -> u8 {
    let (_found_key, bump) = Pubkey::find_program_address(seeds, program_id);
    bump
}

