#![allow(unexpected_cfgs)]
use anchor_lang::prelude::*;
use anchor_lang::solana_program::system_instruction;
use mpl_core::{
    ID as MPL_CORE_ID,
    accounts::BaseCollectionV1, 
    instructions::CreateV2CpiBuilder, 
};
use session_keys::{SessionError, SessionToken, session_auth_or, Session};


declare_id!("8SjSBampBM2asLdQeJoAZpxJxpcbBEGG5q9ADRCAFxr5");

#[program]
pub mod payment_handler {
    use super::*;

    pub fn create_event(ctx: Context<Event>, args: CreateEventArgs) -> Result<()> {
        msg!("create_event instruction invoked");
        let collection = match &ctx.accounts.collection {
            Some(collection) => Some(collection.to_account_info()),
            None => None,
        };
        let authority = match &ctx.accounts.authority {
            Some(authority) => Some(authority.to_account_info()),
            None => None,
        };
        let owner = match &ctx.accounts.owner {
            Some(owner) => Some(owner.to_account_info()),
            None => None,
        };
        let update_authority = match &ctx.accounts.update_authority {
            Some(update_authority) => Some(update_authority.to_account_info()),
            None => None,
        };
        CreateV2CpiBuilder::new(&ctx.accounts.mpl_core_program.to_account_info())
            .asset(&ctx.accounts.asset.to_account_info())
            .collection(collection.as_ref())
            .authority(authority.as_ref())
            .payer(&ctx.accounts.payer.to_account_info())
            .owner(owner.as_ref())
            .update_authority(update_authority.as_ref())
            .system_program(&ctx.accounts.system_program.to_account_info())
            .name(args.name)
            .uri(args.uri)
            .invoke()?;
        Ok(())
    }

    #[session_auth_or(
        ctx.accounts.payment_record.authority.key() == ctx.accounts.payer.key(),
        SessionError::InvalidToken
    )]
    pub fn pay(ctx: Context<Pay>, amount: u64) -> Result<()> {
        let transfer_instruction = system_instruction::transfer(
            &ctx.accounts.payer.key(),
            &ctx.accounts.vault.key(),
            amount,
        );

        anchor_lang::solana_program::program::invoke(
            &transfer_instruction,
            &[
                ctx.accounts.payer.to_account_info(),
                ctx.accounts.vault.to_account_info(),
                ctx.accounts.system_program.to_account_info(),
            ],
        )?;

        let payment_record = &mut ctx.accounts.payment_record;
        payment_record.authority = ctx.accounts.payer.key();  // !
        payment_record.total_paid.checked_add(amount).unwrap();

        let central_tracker = &mut ctx.accounts.central_tracker;
        if payment_record.total_paid > central_tracker.highest_payment {
            central_tracker.highest_payer = ctx.accounts.payer.key();
            central_tracker.highest_payment = payment_record.total_paid;
        }

        Ok(())
    }
}

#[derive(Accounts, Session)]
pub struct Pay<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,
    #[account(
        init_if_needed,
        payer = payer,
        space = 8,
        seeds = [b"vault"],
        bump,
    )]
    pub vault: Account<'info, Vault>,
    #[session(
        signer = payer,
        authority = payer.key() 
    )]
    pub session_token: Option<Account<'info, SessionToken>>,
    #[account(
        init_if_needed,
        payer = payer,
        space = 8 + 8, // Discriminator + u64 for total_paid
        seeds = [b"payment_record", payer.key().as_ref()],
        bump,
    )]
    pub payment_record: Account<'info, PaymentRecord>,
    #[account(
        init_if_needed,
        payer = payer,
        space = 8 + 32 + 8, // Discriminator + Pubkey + u64
        seeds = [b"central_tracker"],
        bump,
    )]
    pub central_tracker: Account<'info, CentralTracker>,

    // #[account(mut)]
    // pub authority: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
#[derive(Default)]
pub struct Vault {}

#[account]
#[derive(Default)]
pub struct PaymentRecord {
    pub authority: Pubkey,
    pub total_paid: u64,
}

#[account]
#[derive(Default)]
pub struct CentralTracker {
    pub highest_payer: Pubkey,
    pub highest_payment: u64,
}

#[derive(Accounts)]
pub struct Event<'info> {
    #[account(mut)]
    pub asset: Signer<'info>,
    #[account(mut)]
    pub collection: Option<Account<'info, BaseCollectionV1>>,
    pub authority: Option<Signer<'info>>,
    #[account(mut)]
    pub payer: Signer<'info>,
    /// CHECK: this account will be checked by the mpl_core program
    pub owner: Option<UncheckedAccount<'info>>,
    /// CHECK: this account will be checked by the mpl_core program
    pub update_authority: Option<UncheckedAccount<'info>>,
    pub system_program: Program<'info, System>,
    #[account(address = MPL_CORE_ID)]
    /// CHECK: this account is checked by the address constraint
    pub mpl_core_program: UncheckedAccount<'info>,
}


#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct CreateEventArgs {
    name: String,
    uri: String,
}

pub fn is_zero_account(account_info: &AccountInfo) -> bool {
    let account_data: &[u8] = &account_info.data.borrow();
    let len = account_data.len();
    let mut is_zero = true;
    for i in 0..len - 1 {
        if account_data[i] != 0 {
            is_zero = false;
        }
    }
    is_zero
}

pub fn bump(seeds: &[&[u8]], program_id: &Pubkey) -> u8 {
    let (_found_key, bump) = Pubkey::find_program_address(seeds, program_id);
    bump
}
