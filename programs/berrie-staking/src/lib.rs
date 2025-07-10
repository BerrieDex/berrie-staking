use anchor_lang::prelude::*;
use instructions::{
    admin::deposit_token::*,
    user::{claim_token::*, stake_token::*, unstake_token::*},
};

#[allow(unused_imports)]
use solana_security_txt::security_txt;

pub mod constants;
pub mod error;
pub mod instructions;
pub mod state;

declare_id!("5FGWD8UZL1X67MV2NuSTS3aRLNMdwk4VdB5CXe3QcT3C");

#[cfg(not(feature = "no-entrypoint"))]
security_txt! {
    name: "Berrie Staking Program",
    project_url: "https://berr.ie/",
    contacts: "twitter:BerrieOrg",
    policy: "https://berrie.gitbook.io/berrie/privacy-policy",
    preferred_languages: "en",
    source_code: "https://github.com/BerrieDex/Staking/"
}


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
