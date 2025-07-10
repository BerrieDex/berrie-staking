use anchor_lang::prelude::*;
use instructions::{
    admin::deposit_token::*,
    user::{claim_token::*, stake_token::*, unstake_token::*},
};

pub mod constants;
pub mod error;
pub mod instructions;
pub mod state;

declare_id!("5FGWD8UZL1X67MV2NuSTS3aRLNMdwk4VdB5CXe3QcT3C");

#[program]
pub mod berrie_staking {
    use super::*;

    // Admin instructions

    pub fn deposit_token(ctx: Context<DepositToken>, amount: u64) -> Result<()> {
        deposit_token_handler(ctx, amount)
    }

    // User instructions

    pub fn stake_token(
        ctx: Context<StakeToken>,
        stake_id: i64,
        event_id: i64,
        amount: u64,
    ) -> Result<()> {
        stake_token_handler(ctx, stake_id, event_id, amount)
    }

    pub fn unstake_token(ctx: Context<UnstakeToken>, stake_id: i64, event_id: i64) -> Result<()> {
        unstake_token_handler(ctx, stake_id, event_id)
    }

    pub fn claim_token(ctx: Context<ClaimToken>, stake_id: i64, event_id: i64) -> Result<()> {
        claim_token_handler(ctx, stake_id, event_id)
    }
}
