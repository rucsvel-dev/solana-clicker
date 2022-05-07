use solana_program::program_error::ProgramError;
use thiserror::Error;

#[derive(Error, Debug, Copy, Clone)]
pub enum ClickerError {
    #[error("Not enough clicks to upgrade")]
    NotEnoughToUpgrade,

    #[error("Not enough clicks to transfer to user")]
    NotEnoughToTransfer,
}

impl From<ClickerError> for ProgramError {
    fn from(e: ClickerError) -> Self {
        ProgramError::Custom(e as u32)
    }
}