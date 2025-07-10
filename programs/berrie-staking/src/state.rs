use crate::constants::TOTAL_UNLOCK_DURATION;
use anchor_lang::prelude::*;

#[account]
#[derive(Debug, InitSpace)]
pub struct Global {
    pub deposited_amount: u64,
    pub staked_amount: u64,
    pub cml_reward_per_token: f64,
}

impl Global {
    pub fn update_cml_reward_per_token(&mut self, amount: u64) {
        if self.staked_amount != 0 {
            let reward_per_token = amount as f64 / self.staked_amount as f64;
            self.cml_reward_per_token += reward_per_token;
        }
    }
}

#[account]
#[derive(Debug, InitSpace)]
pub struct Stake {
    pub user: Pubkey,
    pub staked_amount: u64,
    pub claimed_amount: u64,
    pub stake_id: i64,
    pub staked_at: i64,
    pub cml_reward_per_token: f64,
}

impl Stake {
    pub fn initialize(&mut self, user: Pubkey, stake_id: i64, amount: u64, curr_time: i64) {
        self.user = user;
        self.staked_amount = amount;
        self.stake_id = stake_id;
        self.staked_at = curr_time;
    }

    pub fn get_total_amount(&self, global: &Global) -> u64 {
        let total_amount =
            (global.cml_reward_per_token - self.cml_reward_per_token) * self.staked_amount as f64;

        total_amount as u64
    }

    pub fn get_unclaimed_amount(&self, global: &Global, curr_time: i64) -> u64 {
        let staked_duration = curr_time - self.staked_at;

        let total_amount = self.get_total_amount(global) as f64;

        let unlocked_amount: u64 = (total_amount
            * (staked_duration).min(TOTAL_UNLOCK_DURATION) as f64
            / TOTAL_UNLOCK_DURATION as f64) as u64;

        unlocked_amount - self.claimed_amount
    }
}

#[account]
#[derive(Debug, InitSpace)]
pub struct Event {
    pub user: Pubkey,
    pub amount: u64,
    pub timestamp: i64,
    pub action: Action,
}

#[derive(Debug, InitSpace, Clone, AnchorSerialize, AnchorDeserialize, PartialEq)]
pub enum Action {
    Stake,
    Unstake,
    Claim,
}

impl Event {
    pub fn initialize(&mut self, user: Pubkey, amount: u64, timestamp: i64, action: Action) {
        self.user = user;
        self.amount = amount;
        self.timestamp = timestamp;
        self.action = action;
    }

    pub fn update(&mut self, amount: u64) {
        self.amount += amount;
    }
}
