use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{account_info::{next_account_info, AccountInfo}, entrypoint::ProgramResult, msg, log::sol_log_compute_units, program_error::ProgramError, program::invoke, pubkey::Pubkey, sysvar::{rent::Rent, Sysvar}, system_instruction};
use solana_program::program::invoke_signed;

use crate::instruction::SolanaClickerInstructions;
use crate::state::UserState;
use crate::types::USER_STATE_SEED;
use crate::error::ClickerError;

pub struct Processor {}

impl Processor {
    pub fn process_instruction(
        program_id: &Pubkey,
        accounts: &[AccountInfo],
        instruction_data: &[u8],
    ) -> ProgramResult {
        let instruction = SolanaClickerInstructions::try_from_slice(instruction_data)
            .map_err(|_| ProgramError::InvalidInstructionData)?;
        let accounts_iter = &mut accounts.iter();

        match instruction {
            SolanaClickerInstructions::InitUser => {
                msg!("Instruction: Init user start");
                let user = next_account_info(accounts_iter)?;
                let user_state_ai = next_account_info(accounts_iter)?;
                let system_program = next_account_info(accounts_iter)?;

                let user_state_space = UserState { click_balance: 0, value_per_click: 1, cost_to_upgrade_v1: 10, cost_to_upgrade_v2: 20 };
                let space = user_state_space.try_to_vec()?.len();
                msg!("Instruction: Init user space {:?}", space);
                let (_, bump) = Pubkey::find_program_address(&[user.key.as_ref(), USER_STATE_SEED.as_bytes()], program_id);
                invoke_signed(
                    &system_instruction::create_account(
                        user.key,
                        user_state_ai.key,
                        Rent::get()?.minimum_balance(space),
                        space as u64,
                        program_id,
                    ),
                    &[user.clone(), user_state_ai.clone(), system_program.clone()],
                    &[&[user.key.as_ref(), USER_STATE_SEED.as_bytes(), &[bump]]],
                )?;

                let mut user_state: UserState = UserState::try_from_slice(&user_state_ai.data.borrow())?;
                user_state.click_balance = 0;
                user_state.value_per_click = 1;
                user_state.cost_to_upgrade_v1 = 10;
                user_state.cost_to_upgrade_v2 = 20;
                user_state.serialize(&mut *user_state_ai.data.borrow_mut())?;
                msg!("Instruction: Init user done");
            }
            SolanaClickerInstructions::Click => {
                msg!("Instruction: Click started");
                let user = next_account_info(accounts_iter)?;
                let user_state_account = next_account_info(accounts_iter)?;
                let mut user_state: UserState = UserState::try_from_slice(&user_state_account.data.borrow())?;
                user_state.click_balance += user_state.value_per_click as u64;
                user_state.serialize(&mut *user_state_account.data.borrow_mut())?;
                msg!("Instruction: Click done");
            }
            SolanaClickerInstructions::UpgradeValuePerClick { variation } => {
                msg!("Instruction: UpgradeValuePerClick started {:?}", variation);
                let user = next_account_info(accounts_iter)?;
                let user_state_account = next_account_info(accounts_iter)?;
                let mut user_state: UserState = UserState::try_from_slice(&user_state_account.data.borrow())?;
                match variation {
                    0 => {
                        if user_state.cost_to_upgrade_v1 as u64 > user_state.click_balance {
                            return Err(ClickerError::NotEnoughToUpgrade.into())
                        }
                        user_state.click_balance -= user_state.cost_to_upgrade_v1 as u64;
                        user_state.value_per_click += 1;
                        user_state.cost_to_upgrade_v1 *= 2;
                    }
                    1 => {
                        if user_state.cost_to_upgrade_v2 as u64 > user_state.click_balance {
                            return Err(ClickerError::NotEnoughToUpgrade.into())
                        }
                        user_state.click_balance -= user_state.cost_to_upgrade_v2 as u64;
                        user_state.value_per_click += 2;
                        user_state.cost_to_upgrade_v2 *= 2;
                    }
                    _ => {}
                }
                user_state.serialize(&mut *user_state_account.data.borrow_mut())?;
                msg!("Instruction: UpgradeValuePerClick done");
            }
            SolanaClickerInstructions::TransferClicks { value } => {
                msg!("Instruction: TransferClicks started {:?}", value);
                let user = next_account_info(accounts_iter)?;
                let user_state_account = next_account_info(accounts_iter)?;
                let user_to_transfer = next_account_info(accounts_iter)?;
                let user_to_transfer_state_account = next_account_info(accounts_iter)?;

                let mut user_state: UserState = UserState::try_from_slice(&user_state_account.data.borrow())?;
                let mut user_to_transfer_state: UserState = UserState::try_from_slice(&user_to_transfer_state_account.data.borrow())?;

                if value > user_state.click_balance {
                    return Err(ClickerError::NotEnoughToTransfer.into())
                }

                user_state.click_balance -= value;
                user_state.value_per_click += 1;
                user_to_transfer_state.click_balance += value;

                user_state.serialize(&mut *user_state_account.data.borrow_mut())?;
                user_to_transfer_state.serialize(&mut *user_to_transfer_state_account.data.borrow_mut())?;
                msg!("Instruction: TransferClicks done");
            }
        }
        Ok(())
    }
}
