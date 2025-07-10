use anchor_lang::prelude::*;

#[error_code]
pub enum BerrieStakingError {
    #[msg("Invalid amount")]
    InvalidAmount,
    #[msg("Invalid event")]
    InvalidEvent,
}
