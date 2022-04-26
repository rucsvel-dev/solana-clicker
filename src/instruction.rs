use borsh::{BorshDeserialize, BorshSerialize};

#[derive(BorshDeserialize, BorshSerialize)]
pub enum SolanaClickerInstructions {
    InitUser,
    Click,
    UpgradeValuePerClick { variation: u8 }
}