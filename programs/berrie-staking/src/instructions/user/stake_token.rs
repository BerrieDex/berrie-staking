use crate::{
    constants::{seeds, TOKEN_PUBKEY},
    error::BerrieStakingError,
    state::{Action, Event, Global, Stake},
};
use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token_2022::Token2022,
    token_interface::{transfer_checked, Mint, TokenAccount, TransferChecked},
};

#[derive(Accounts)]
#[instruction(stake_id: i64, event_id: i64)]
pub struct StakeToken<'info> {
    #[account(
        init,
        seeds = [
            seeds::STAKE_SEED,
            user.key().as_ref(),
            stake_id.to_le_bytes().as_ref(),
        ],
        payer = user,
        space = 8 + Stake::INIT_SPACE,
        bump,
    )]
    pub stake: Box<Account<'info, Stake>>,
    #[account(
        init,
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
        mut,
        seeds = [seeds::GLOBAL_SEED],
        bump,
    )]
    pub global: Box<Account<'info, Global>>,
    #[account(mut)]
    pub user: Signer<'info>,

    #[account(address = TOKEN_PUBKEY)]
    pub token_mint: Box<InterfaceAccount<'info, Mint>>,

    #[account(
        init,
        payer = user,
        associated_token::mint = token_mint,
        associated_token::authority = stake,
        associated_token::token_program = token_program,
    )]
    pub associated_stake: Box<InterfaceAccount<'info, TokenAccount>>,
    #[account(
        mut,
        associated_token::mint = token_mint,
        associated_token::authority = user,
        associated_token::token_program = token_program,
    )]
    pub associated_user: Box<InterfaceAccount<'info, TokenAccount>>,

    pub associated_token_program: Program<'info, AssociatedToken>,
    pub token_program: Program<'info, Token2022>,
    pub system_program: Program<'info, System>,
}

pub fn stake_token_handler(
    ctx: Context<StakeToken>,
    stake_id: i64,
    _event_id: i64,
    amount: u64,
) -> Result<()> {
    let stake = &mut ctx.accounts.stake;
    let global = &mut ctx.accounts.global;
    let user = ctx.accounts.user.key();

    let user_balance = ctx.accounts.associated_user.amount;
    let stake_amount = amount.min(user_balance);

    require!(stake_amount > 0, BerrieStakingError::InvalidAmount);

    let curr_time = Clock::get()?.unix_timestamp;
    stake.initialize(user, stake_id, stake_amount, curr_time);

    transfer_checked(
        CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            TransferChecked {
                from: ctx.accounts.associated_user.to_account_info(),
                to: ctx.accounts.associated_stake.to_account_info(),
                authority: ctx.accounts.user.to_account_info(),
                mint: ctx.accounts.token_mint.to_account_info(),
            },
        ),
        stake_amount,
        ctx.accounts.token_mint.decimals,
    )?;

    global.staked_amount += stake_amount;
    stake.cml_reward_per_token = global.cml_reward_per_token;

    let event = &mut ctx.accounts.event;
    event.initialize(user, stake_amount, curr_time, Action::Stake);

    Ok(())
}
