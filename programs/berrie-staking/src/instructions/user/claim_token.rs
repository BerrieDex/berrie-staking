use crate::{
    constants::{seeds, MIN_STAKE_DURATION, TOKEN_PUBKEY},
    error::BerrieStakingError,
    state::{Action, Event, Global, Stake},
};
use anchor_lang::prelude::*;
use anchor_spl::{
    token_2022::Token2022,
    token_interface::{transfer_checked, Mint, TokenAccount, TransferChecked},
};

#[derive(Accounts)]
#[instruction(stake_id: i64, event_id: i64)]
pub struct ClaimToken<'info> {
    #[account(
        mut,
        seeds = [
            seeds::STAKE_SEED,
            user.key().as_ref(),
            stake_id.to_le_bytes().as_ref(),
        ],
        bump,
    )]
    pub stake: Box<Account<'info, Stake>>,
    #[account(
        init_if_needed,
        seeds = [
            seeds::EVENT_SEED,
            user.key().as_ref(),
            event_id.to_le_bytes().as_ref(),
        ],
        payer = user,
        space = 8 + Event::INIT_SPACE,
        bump,
    )]
    pub event: Box<Account<'info, Event>>,
    #[account(
        seeds = [seeds::GLOBAL_SEED],
        bump,
    )]
    pub global: Box<Account<'info, Global>>,

    #[account(mut)]
    pub user: Signer<'info>,

    #[account(address = TOKEN_PUBKEY)]
    pub token_mint: Box<InterfaceAccount<'info, Mint>>,

    #[account(
        mut,
        associated_token::mint = token_mint,
        associated_token::authority = global,
        associated_token::token_program = token_program,
    )]
    pub associated_global: Box<InterfaceAccount<'info, TokenAccount>>,
    #[account(
        mut,
        associated_token::mint = token_mint,
        associated_token::authority = user,
        associated_token::token_program = token_program,
    )]
    pub associated_user: Box<InterfaceAccount<'info, TokenAccount>>,

    pub token_program: Program<'info, Token2022>,
    pub system_program: Program<'info, System>,
}

pub fn claim_token_handler(
    ctx: Context<ClaimToken>,
    _staked_id: i64,
    _event_id: i64,
) -> Result<()> {
    let stake = &mut ctx.accounts.stake;
    let global = &ctx.accounts.global;

    let curr_time = Clock::get()?.unix_timestamp;

    if curr_time - stake.staked_at < MIN_STAKE_DURATION {
        return Ok(());
    }

    let unclaimed_amount = stake.get_unclaimed_amount(&global, curr_time);

    if unclaimed_amount == 0 {
        return Ok(());
    }

    let signer_seeds: &[&[&[u8]]] = &[&[seeds::GLOBAL_SEED, &[ctx.bumps.global]]];

    transfer_checked(
        CpiContext::new_with_signer(
            ctx.accounts.token_program.to_account_info(),
            TransferChecked {
                from: ctx.accounts.associated_global.to_account_info(),
                to: ctx.accounts.associated_user.to_account_info(),
                authority: global.to_account_info(),
                mint: ctx.accounts.token_mint.to_account_info(),
            },
            signer_seeds,
        ),
        unclaimed_amount,
        ctx.accounts.token_mint.decimals,
    )?;

    stake.claimed_amount += unclaimed_amount;

    let user = ctx.accounts.user.key();

    let event = &mut ctx.accounts.event;

    if event.user.eq(&Pubkey::default()) {
        event.initialize(user, unclaimed_amount, curr_time, Action::Claim);
    } else if event.action.eq(&Action::Claim) {
        event.update(unclaimed_amount);
    } else {
        return err!(BerrieStakingError::InvalidEvent);
    }

    Ok(())
}
