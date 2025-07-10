use crate::{
    constants::{seeds, ADMIN_PUBKEY, TOKEN_PUBKEY},
    state::Global,
};
use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token_2022::Token2022,
    token_interface::{transfer_checked, Mint, TokenAccount, TransferChecked},
};

#[derive(Accounts)]
pub struct DepositToken<'info> {
    #[account(
        init_if_needed,
        seeds = [seeds::GLOBAL_SEED],
        payer = admin,
        space = 8 + Global::INIT_SPACE,
        bump,
    )]
    pub global: Box<Account<'info, Global>>,
    #[account(
        mut,
        address = ADMIN_PUBKEY,
    )]
    pub admin: Signer<'info>,

    #[account(address = TOKEN_PUBKEY)]
    pub token_mint: Box<InterfaceAccount<'info, Mint>>,

    #[account(
        init_if_needed,
        payer = admin,
        associated_token::mint = token_mint,
        associated_token::authority = global,
        associated_token::token_program = token_program,
    )]
    pub associated_global: Box<InterfaceAccount<'info, TokenAccount>>,
    #[account(
        mut,
        associated_token::mint = token_mint,
        associated_token::authority = admin,
        associated_token::token_program = token_program,
    )]
    pub associated_admin: Box<InterfaceAccount<'info, TokenAccount>>,

    pub associated_token_program: Program<'info, AssociatedToken>,
    pub token_program: Program<'info, Token2022>,
    pub system_program: Program<'info, System>,
}

pub fn deposit_token_handler(ctx: Context<DepositToken>, amount: u64) -> Result<()> {
    let global = &mut ctx.accounts.global;
    global.deposited_amount += amount;

    if amount > 0 {
        global.update_cml_reward_per_token(amount);

        transfer_checked(
            CpiContext::new(
                ctx.accounts.token_program.to_account_info(),
                TransferChecked {
                    from: ctx.accounts.associated_admin.to_account_info(),
                    to: ctx.accounts.associated_global.to_account_info(),
                    authority: ctx.accounts.admin.to_account_info(),
                    mint: ctx.accounts.token_mint.to_account_info(),
                },
            ),
            amount,
            ctx.accounts.token_mint.decimals,
        )?;
    }

    Ok(())
}
