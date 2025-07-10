use anchor_lang::{constant, prelude::Pubkey, pubkey};

#[constant]
pub const ADMIN_PUBKEY: Pubkey = pubkey!("Ez9qT5t8wUMSFAN1htaCG1D42Nn4mYNgacCjEQCmw5RA");
#[constant]
pub const TOKEN_PUBKEY: Pubkey = pubkey!("mntAk89WGn1YacVFxzU84tVbn3zFYf1LVXMxMjhpTjC");
#[constant]
pub const MIN_STAKE_DURATION: i64 = 30 * 24 * 60 * 60;
#[constant]
pub const TOTAL_UNLOCK_DURATION: i64 = 200 * 24 * 60 * 60;

pub mod seeds {
    pub const GLOBAL_SEED: &[u8] = b"global";
    pub const STAKE_SEED: &[u8] = b"stake";
    pub const EVENT_SEED: &[u8] = b"event";
}
