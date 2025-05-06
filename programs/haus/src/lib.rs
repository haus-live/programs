#![allow(unexpected_cfgs)]

use anchor_lang::prelude::*;
use anchor_lang::solana_program::system_instruction;
use mpl_core::{
    ID as MPL_CORE_ID,
};
use mpl_core::instructions::{
    CreateV2CpiBuilder, 
    TransferV1CpiBuilder
};
// use session_keys::{SessionError, SessionToken, session_auth_or, Session};

declare_id!("EvNT111111111111111111111111111111111111111");

#[program]
pub mod event_fund {
    use super::*;

    /// Initializes an event with begin and end timestamps and a payment threshold.
    pub fn initialize_event(
        ctx: Context<InitializeEvent>,
        name: String,
        uri: String,
        begin_timestamp: i64,
        end_timestamp: i64,
        threshold: u64,
    ) -> Result<()> {
        require!(begin_timestamp < end_timestamp, ErrorCode::InvalidTimestamps);

        let event = &mut ctx.accounts.event;
        // event.authority = authority.key();  // todo: authority = payer?
        event.authority = ctx.accounts.authority.key();
        event.asset = ctx.accounts.asset.key();
        event.begin_timestamp = begin_timestamp;
        event.end_timestamp = end_timestamp;
        event.leader = None;
        event.leader_total = 0;
        event.threshold = threshold;

        // Create Metaplex Core asset (simplified for this example)
        CreateV2CpiBuilder::new(&ctx.accounts.mpl_core_program.to_account_info())
            .asset(&ctx.accounts.asset.to_account_info())
            .authority(Some(ctx.accounts.authority.as_ref()))
            .payer(ctx.accounts.payer.as_ref())
            .name(name)
            .uri(uri)
            .invoke()?;

        msg!("Event initialized with asset: {}, time range: {} to {}, and threshold: {}", event.asset, begin_timestamp, end_timestamp, threshold);
        Ok(())
    }

    // /// Processes a payment using session keys for gasless transactions.
    // #[session_auth_or(
    //     ctx.accounts.payment_record.authority.key() == ctx.accounts.payer.key(),
    //     SessionError::InvalidToken
    // )]
    pub fn make_payment(ctx: Context<MakePayment>, amount: u64) -> Result<()> {
        let current_time = Clock::get().unwrap().unix_timestamp;
        let event = &mut ctx.accounts.event;

        require!(current_time >= event.begin_timestamp, ErrorCode::EventNotStarted);
        require!(current_time <= event.end_timestamp, ErrorCode::EventEnded);

        let payment = &mut ctx.accounts.payment;
        payment.total += amount;

        let transfer_instruction = system_instruction::transfer(
            &ctx.accounts.payer.key(),
            &event.key(),
            amount,
        );

        // Transfer SOL
        anchor_lang::solana_program::program::invoke(
            &transfer_instruction,
            &[
                ctx.accounts.payer.to_account_info(),
                event.to_account_info(),
                ctx.accounts.system_program.to_account_info(),
            ],
        )?;

        // Update leader if necessary
        let user_total = payment.total;
        if event.leader.is_none() || user_total > event.leader_total {
            event.leader = Some(ctx.accounts.payer.key());
            event.leader_total = user_total;
        } else if event.leader == Some(ctx.accounts.payer.key()) {
            event.leader_total = user_total;
        }

        msg!("Payment of {} lamports made by {} to event: {}, new total: {}", amount, ctx.accounts.payer.key(), ctx.accounts.event.key(), user_total);
        Ok(())
    }

    /// Transfers the core asset to the leader if they meet the threshold, otherwise to the authority.
    pub fn transfer_asset(ctx: Context<TransferAsset>) -> Result<()> {
        let event = &ctx.accounts.event;
        let current_time = Clock::get().unwrap().unix_timestamp;

        require!(current_time > event.end_timestamp, ErrorCode::EventNotEnded);

        let recipient = if event.leader.is_some() && event.leader_total >= event.threshold {
            event.leader.unwrap()
        } else {
            event.authority
        };

        require!(recipient.key() == ctx.accounts.authority.key(), ErrorCode::SignerNoAuthority);

        // Transfer the asset using Metaplex Core's transfer instruction
        TransferV1CpiBuilder::new(&ctx.accounts.mpl_core_program.to_account_info())
            .asset(&ctx.accounts.asset.to_account_info())
            .authority(Some(&ctx.accounts.authority.to_account_info()))
            .new_owner(&ctx.accounts.authority.to_account_info())
            .invoke()?;

        msg!("Asset {} transferred to: {}", event.asset, recipient);
        Ok(())
    }

    // /// Withdraws funds from the event account to the authority after the event has ended.
    // pub fn withdraw(ctx: Context<Withdraw>) -> Result<()> {
    //     let event = &mut ctx.accounts.event;
    //     let authority = &ctx.accounts.authority;
    //     let current_time = Clock::get().unwrap().unix_timestamp;

    //     require!(current_time > event.end_timestamp, ErrorCode::EventNotEnded);

    //     let rent = Rent::get().unwrap();
    //     let event_data_len = event.to_account_info().data_len();
    //     let rent_exempt = rent.minimum_balance(event_data_len);

    //     let event_lamports = event.to_account_info().lamports();
    //     if event_lamports <= rent_exempt {
    //         return Err(ErrorCode::InsufficientFunds.into());
    //     }

    //     let withdraw_amount = event_lamports - rent_exempt;

    //     // Transfer SOL from event to authority
    //     let seeds = &[&[b"event", authority.key().as_ref(), &[ctx.bumps.event]]];
    //     let signer = &[&seeds[..]];

    //     let cpi_accounts = anchor_lang::system_program::Transfer {
    //         from: event.to_account_info(),
    //         to: authority.to_account_info(),
    //     };
    //     let cpi_program = ctx.accounts.system_program.to_account_info();
    //     anchor_lang::system_program::transfer(
    //         CpiContext::new_with_signer(cpi_program, cpi_accounts, signer),
    //         withdraw_amount,
    //     )?;

    //     msg!("Withdrew {} lamports from event to authority", withdraw_amount);
    //     Ok(())
    // }
}

#[derive(Accounts)]
pub struct InitializeEvent<'info> {
    /// CHECK: This is the Metaplex Core asset account (created by Metaplex)
    #[account(mut)]
    pub asset: Signer<'info>,
    #[account(mut)]
    pub authority: Signer<'info>,
    #[account(mut)]
    pub payer: Signer<'info>,
    #[account(
        init,
        payer = payer,
        space = 8 + 32 + 32 + 8 + 8 + 1 + 32 + 8 + 8, // Discriminator + authority + asset + begin_timestamp + end_timestamp + leader (Option<Pubkey>) + leader_total + threshold
        seeds = [b"event", authority.key().as_ref()],
        bump
    )]
    pub event: Account<'info, Event>,
    pub system_program: Program<'info, System>,
    /// CHECK: Metaplex Core program
    #[account(address = MPL_CORE_ID)]
    pub mpl_core_program: UncheckedAccount<'info>,
}

#[derive(Accounts)]
pub struct MakePayment<'info> {
    #[account(
        mut,
        seeds = [b"event", authority.key().as_ref()],
        bump
    )]
    pub event: Account<'info, Event>,
    #[account(
        init_if_needed,
        payer = payer,
        space = 8 + 8, // Discriminator + total: u64
        seeds = [b"payment", event.key().as_ref(), payer.key().as_ref()],
        bump
    )]
    pub payment: Account<'info, Payment>,
    #[account(mut)]
    pub payer: Signer<'info>,
    /// CHECK: The authority of the event
    pub authority: AccountInfo<'info>,
    pub system_program: Program<'info, System>,
    // /// CHECK: Session key account for gasless transactions
    // pub session_key: Account<'info, SessionKey>,
}

#[derive(Accounts)]
pub struct TransferAsset<'info> {
    #[account(
        mut,
        seeds = [b"event", authority.key().as_ref()],
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

#[derive(Accounts)]
pub struct Withdraw<'info> {
    #[account(
        mut,
        seeds = [b"event", authority.key().as_ref()],
        bump,
        has_one = authority,
    )]
    pub event: Account<'info, Event>,
    #[account(mut)]
    pub authority: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct Event {
    pub authority: Pubkey,      // The creator of the event
    pub asset: Pubkey,         // The Metaplex Core asset representing the event
    pub begin_timestamp: i64,  // Start time of the event
    pub end_timestamp: i64,    // End time of the event
    pub leader: Option<Pubkey>, // The user with the highest total payments
    pub leader_total: u64,     // The highest total payment amount
    pub threshold: u64,        // Minimum payment threshold for asset transfer
}

#[account]
pub struct Payment {
    pub total: u64,            // Total payments made by the user to the event
}

#[error_code]
pub enum ErrorCode {
    #[msg("The begin timestamp must be before the end timestamp.")]
    InvalidTimestamps,
    #[msg("The event has not started yet.")]
    EventNotStarted,
    #[msg("The event has already ended.")]
    EventEnded,
    #[msg("The event has not ended yet.")]
    EventNotEnded,
    #[msg("The signer has no authority over asset.")]
    SignerNoAuthority,
    // #[msg("Insufficient funds in the event account.")]
    // InsufficientFunds,
}

pub fn bump(seeds: &[&[u8]], program_id: &Pubkey) -> u8 {
    let (_found_key, bump) = Pubkey::find_program_address(seeds, program_id);
    bump
}
