use crate::error::{Vault, VaultError::InvalidInstruction};
use solana_program::program_error::ProgramError;
use solana_program::{
    instruction::{AccountMeta, Instruction},
    program_option::COption,
    pubkey::Pubkey,
    sysvar,
};

use std::convert::TryInto;
use std::mem::size_of;

pub enum VaultInstruction {

    InitializeVault{
        strategy_program_deposit_instruction_id: u8,
        strategy_program_withdraw_instruction_id: u8,
        hold: bool,
    },

    Deposit {
        amount: u64
    },

    Withdraw {
        amount: u64,
    }

}

pub enum StrategyInstruction {

    Deposit {
        amount: u64
    },

    Withdraw {
        amount: u64
    },
}

impl StrategyInstruction {
    pub fn unpack(input: &[u8]) -> Result<Self, ProgramError> {
        let (tag, rest) = input.split_first().ok_or(InvalidInstruction)?;

        Ok(match tag {
            0 | 1 => {
                let amount = rest
                    .get(..8)
                    .and_then(|slice| slice.try_into().ok())
                    .map(u64::from_le_bytes)
                    .ok_or(InvalidInstruction)?;
                match tag {
                    1 => Self::Deposit { amount },
                    2 => Self::Withdraw { amount },
                    _ => return Err(VaultError::InvalidInstruction.into()),
                }
            }
            _ => return Err(ValutError::InvalidInstruction.into()),       
        })
    }
    
    fn pack(&self) -> Vec<u8> {
        
    }
}

