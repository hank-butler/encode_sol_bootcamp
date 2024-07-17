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
        let mut buf = Vec::with_capacity(size_of::<Self>());
        match self {
            $Self::Deposit { amount } => {
                buf.push(2);
                buf.extend_from_slice(&amount.to_le_bytes());
            }

            &Self::Withdraw { amount } => {
                bug.push(3);
                buf.extend_from_slice(&amount.to_le_bytes());
            }
        }
        buf

    }

    pub fn desposit(
        program_id: &Pubkey,
        token_program_id: &Pubkey,
        source_pubkey: &Pubkey,
        target_pubkey: &Pubkey,
        additional_account_metas: Vec<AccountMeta>,
        amount: u64
    ) -> Result<Instruction, ProgramError> {
        return create_transfer(
        Self::Deposit { amount }.pack(),
        program_id,
        token_program_id,
        source_pubkey,
        target_pubkey,
        additional_account_metas,
        );
    }

    pub fn withdraw(
        program_id: &Pubkey,
        token_program_id: &Pubkey,
        source_pubkey: &Pubkey,
        target_pubkey: &Pubkey,
        additional_account_metas: Vec<AccountMeta>,
        amount: u64
    ) -> Result<Instruction, ProgramError> {
        return create_transfer(
            Self::Withdraw { amount }.pack(),
            program_id,
            token_program_id,
            source_pubkey,
            target_pubkey,
            additional_account_metas

        );
    }   

}

impl VaultInstruction {
    pub fn unpack(input: &[u8]) -> Result<Self, ProgramError> {
        let (tag, rest) = input.split_first().ok_or(InvalidInstruction)?;

        
    }
}