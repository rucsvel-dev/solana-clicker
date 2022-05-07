use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::pubkey::Pubkey;

use crate::types::USER_STATE_SEED;

#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct UserState {
    pub click_balance: u64,
    pub value_per_click: u32,
    pub cost_to_upgrade_v1: u32,
    pub cost_to_upgrade_v2: u32,
}

impl UserState {
    pub fn get_pubkey(user: &Pubkey, program_id: &Pubkey) -> Pubkey {
        Pubkey::create_with_seed(user, USER_STATE_SEED, program_id).unwrap()
    }

    pub fn is_ok_pubkey(user: &Pubkey, user_state_pubkey: &Pubkey, program_id: &Pubkey) -> bool {
        user_state_pubkey.to_bytes() == Self::get_pubkey(user, program_id).to_bytes()
    }
}