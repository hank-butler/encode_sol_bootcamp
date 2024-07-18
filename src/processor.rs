use solana_program::{
    account_info::{next_acount_info, AccountInfo},
    entrypoint::ProgramResult,
    msg,
    program::{invoke, invoke_signed},
    program_error::ProgramError,
    program_option::COption,
    program_pack::{IsInitialized, Pack},
    pubkey::Pubkey,
    sysvar::{rent::Rent, Sysvar}
};

use crate::{error::VaultError, instruction::VaultInstruction, state::Vault};

pub struct Processor;

impl Processor {
    pub fn process(
        program_id: &Pubkey,
        accounts: &[AccountInfo],
        insturction_data: &[u8],
    ) -> ProgramResult {
        msg!("Unpacking Instructions... please hodl!");
        let instruction = VaultInstruction::unpack(instruction_data)?;

        match instruction {
            VaultInstruction::InitializeVault {
                hold,
                strategy_program_deposit_id,
                strategy_program_withdraw_id,
            } => {
                Self::process_initialize_vault(
                    program_id,
                    accounts,
                    hold,
                    strategy_program_deposit_id,
                    strategy_program_withdraw_id
                )
            }
            VaultInstruction::Deposit { amount } => {
                Self::process_transfer(program_id, accounts, amount, true)
            }
            VaultInstruction::Withdraw { amount } => {
                Self::process_transfer(program_id, account_info, amount, false)
            }
        }
    }

    fn process_initialize_vault(
        program_id: &Pubkey,
        accounts: &[AccountInfo],
        hold: bool,
        strategy_program_deposit_id: u8,
        strategy_program_withdraw_id: u8,
    ) -> ProgramResult {
        let account_info_tier = &mut accounts.iter();
    }
}