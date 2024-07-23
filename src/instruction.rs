// idea behind yearn finance
// https://medium.com/iearn/yearn-finance-explained-what-are-vaults-and-strategies-96970560432

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

        Ok(match tag {
            0 => {
                let hold = *rest.get(0).unwrap();
                let strategy_program_deposit_instruction_id = *rest.get(0).unwrap();
                let strategy_program_withdraw_instruction_id = *rest.get(1).unwrap();
                Self::InitializeVault {
                    hold: if hold == 1 { true } else { false },
                    strategy_program_deposit_instruction_id,
                    strategy_program_withdraw_instruction_id,
                }
            }
            1 | 2 => {
                let amount = rest
                    .get(..8)
                    .and_then(|slice| slice.try_into().ok())
                    .map(u64::from_le_bytes)
                    .ok_or(InvalidInstruction)?;
            }
            _ => return Err(VaultError::InvalidInstruction.info()),
        })
    }

    fn pack(&self) -> Vec<u8> {
        let mut buf = Vec::with_capacity(size_of::<Self>());
        match self {
            &Self::InitializeVault{
                hold,
                strategy_program_deposit_instruction_id,
                strategy_program_withdraw_instruction_id,
            } => {
                buf.push(0);
                buf.push(hold as u8);
                buf.push(strategy_program_deposit_instruction_id);
                buf.push(strategy_program_withdraw_instruction_id);
            }
            &Self::Deposit { amount } => {
                buf.push(1);
                buf.extend_from_slice(&amount.to_le_bytes());
            }
            &Self::Withdraw { amount } => {
                buf.push(2);
                buf.extend_from_slice(&amount.to_le_bytes());
            }
        }
         buf
    }

    pub fn initialize_vault(
        vault_program_id: &Pubkey,
        initializer: &Pubkey,
        vault_storage_account: &Pubkey,
        lx_token_account: &Pubkey,
        llx_token_mint_id: &Pubkey,
        token_program: &Pubkey,
        strategy_program: &Pubkey,
        hold: bool,
        x_token_account: COption<Pubkey>,
        strategy_program_deposit_instruction_id: u8,
        strategy_program_withdraw_instruction_id: u8,
    ) -> Result<Instruction, ProgramError> {
        let mut accounts = vec![
            AccountMeta::new_readonly(*initializer, true),
            AccountMeta::new(*vault_storage_account, false),
            AccountMeta::new(*lx_token_account, false),
            AccountMeta::new(*llx_token_mint_id, false),
            AccountMeta::new_readonly(*token_program, false),
            AccountMeta::new_readonly(*strategy_program, false),
            AccountMeta::new_readonly(sysvar::rent::id(), false),
        ];
        assert_eq!(hold, x_token_account.is_some());
        if hold {
            accounts.push(AccountMeta::new(x_token_account.unwrap(), false));
        }

        let data = VaultInstruction::InitializeVault {
            hold,
            strategy_program_deposit_instruction_id,
            strategy_program_withdraw_instruction_id,
        }
        .pack();
        Ok(Instruction {
            program_id: *vault_program_id,
            accounts,
            data
        })
    }
    pub fn deposit(
        vault_program_id: &Pubkey,
        token_program_id: &Pubkey,
        source_pubkey: &Pubkey,
        target_pubkey: &Pubkey,
        additional_account_metas: Vec<AccountMeta>,
        amount: u64,
    ) -> Result<Instruction, ProgramError> {
        return create_transfer(
            Self::Deposit { amount }.pack(),
            vault_program_id,
            token_program_id,
            source_pubkey,
            target_pubkey,
            additional_account_metas,
        );
    }

    pub fn withdraw(
        vault_program_id: &Pubkey,
        token_program_id: &Pubkey,
        source_pubkey: &Pubkey,
        target_pubkey: &Pubkey,
        additional_account_metas: Vec<AccountMeta>,
        amount: u64,
    ) -> Result<Instruction, ProgramError> {
        return create_transfer(
            Self::Withdraw { amount }.pack(),
            vault_program_id,
            token_program_id,
            source_pubkey,
            target_pubkey,
            additional_account_metas
        );
    }
    
}

pub fn create_transfer(
    data: Vec<u8>,
    vault_program_id: &Pubkey,
    token_program_id: &Pubkey,
    source_pubkey: &Pubkey,
    target_pubkey: &Pubkey,
    additional_account_metas: Vec<AccountMeta>
) -> Result<Instruction, ProgramError> {
    let mut accounts = vec![
        AccountMeta::new_readonly(*token_program_id, false),
        AccountMeta::new(*source_pubkey, false),
        AccountMeta::new(*target_pubkey, false),
    ];
    accounts.extend(additional_account_metas);

    Ok(Instruction {
        program_id: *vault_program_id,
        accounts,
        data
    })
}