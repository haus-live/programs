#![allow(unexpected_cfgs)]

use std::mem;

use anchor_lang::prelude::*;
use anchor_lang::prelude::Pubkey;

use anchor_spl::token::{Token, TokenAccount};

use mpl_token_metadata::ID as MPL_TOKEN_METADATA_ID;
use mpl_core::ID as MPL_CORE_ID;

pub mod errors;
pub mod utils;
pub mod constants;
pub mod instructions;

pub use errors::{
    ErrorCode as CErrorCode,
    NftVerifierError
};

declare_id!("8SjSBampBM2asLdQeJoAZpxJxpcbBEGG5q9ADRCAFxr5");

#[program]
pub mod haus {
    use super::*;

    pub fn make_tip(ctx: Context<MakeTip>, args: MakeTipArgs) -> Result<()> {
        instructions::make_tip(ctx, args)
    }

    pub fn create_event(ctx: Context<CreateEvent>, args: CreateEventArgs) -> Result<()> {
        instructions::create_event(ctx, args)
    }

    pub fn claim_realtime_asset(ctx: Context<ClaimRealtimeAsset>) -> Result<()> {
        instructions::claim_realtime_asset(ctx)
    }

    pub fn withdraw_tips(ctx: Context<WithdrawTips>) -> Result<()> {
        instructions::withdraw_tips(ctx)
    }
}

// <withdraw_tips>
#[derive(Accounts)]
pub struct WithdrawTips<'info> {
    #[account(
        mut,
        seeds = [constants::EVENT_SEED, authority.key().as_ref()],
        bump,
        has_one = authority
    )]
    pub event: Account<'info, Event>,
    /// CHECK: authority
    #[account(mut)]
    pub authority: Signer<'info>,
    pub system_program: Program<'info, System>
}
// </withdraw_tips>

// <create_event>
#[account]
pub struct Event {
    /// CHECK: event authority
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
    pub reserve_price: u128,
    /// Ticket collection (Metaplex Token Metadata)
    pub ticket_collection: Pubkey,
    /// Event type (category)
    pub art_category: ArtCategory,
}

#[derive(Accounts)]
pub struct CreateEvent<'info> {
    #[account(mut)]
    pub realtime_asset: Signer<'info>,
    /// CHECK: authority
    #[account(mut)]
    pub authority: Signer<'info>,
    #[account(
        init_if_needed,
        payer = authority,
        space = 8 + mem::size_of::<Event>(),
        seeds = [constants::EVENT_SEED, authority.key().as_ref()],
        bump
    )]
    pub event: Account<'info, Event>,
    pub system_program: Program<'info, System>,
    /// CHECK: Metaplex Core program
    #[account(address = MPL_CORE_ID)]
    pub mpl_core_program: UncheckedAccount<'info>,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub enum ArtCategory {
    StandupComedy,
    PerformanceArt,
    PoetrySlam,
    OpenMicImprov,
    LivePainting,
    CreatingWorkshop,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct CreateEventArgs {
    name: String,
    uri: String,
    begin_timestamp: i64,
    end_timestamp: i64,
    reserve_price: u128,
    ticket_collection: Pubkey,
    art_category: ArtCategory,
}
// </create_event>

// <claim_realtime_asset>
#[derive(Accounts)]
pub struct ClaimRealtimeAsset<'info> {
    #[account(
        seeds = [constants::EVENT_SEED, authority.key().as_ref()],
        bump
    )]
    pub event: Account<'info, Event>,
    /// CHECK: This is the Metaplex Core asset account
    #[account(mut)]
    pub asset: UncheckedAccount<'info>,
    #[account(mut)]
    pub authority: Signer<'info>,
    /// CHECK: Metaplex Core program
    #[account(address = MPL_CORE_ID)]
    pub mpl_core_program: UncheckedAccount<'info>,
}
// </claim_realtime_asset>

// <make_tip>
#[derive(Accounts)]
pub struct MakeTip<'info> {
    #[account(
        mut,
        seeds = [constants::EVENT_SEED, authority.key().as_ref()],
        bump
    )]
    pub event: Account<'info, Event>,
    #[account(
        init_if_needed,
        payer = payer,
        space = 8 + mem::size_of::<TippingCalculator>(),
        seeds = [
            constants::TIPPING_CALCULATOR_SEED, 
            event.key().as_ref(), 
            authority.key().as_ref()
        ],
        bump
    )]
    pub tipping_calculator: Account<'info, TippingCalculator>,
    #[account(mut)]
    pub payer: Signer<'info>,
    /// CHECK: authority
    #[account(mut)]
    pub authority: AccountInfo<'info>,
    // #[account(
    //     constraint = token_account.owner == authority.key(),
    //     constraint = token_account.mint == mint.key(),
    //     constraint = token_account.amount == 1
    // )]
    // pub token_account: Account<'info, TokenAccount>,
    // /// CHECK: mint account
    // pub mint: AccountInfo<'info>,
    // #[account(
    //     owner = MPL_TOKEN_METADATA_ID
    // )]
    // /// CHECK: metadata account
    // pub metadata_account: AccountInfo<'info>,
    // /// CHECK: expected collection mint
    // pub expected_collection_mint: AccountInfo<'info>,
    // pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}

#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct MakeTipArgs {
    pub amount: u64,
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
// </make_tip>

// TODO: research composite accounts
