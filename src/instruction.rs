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
    
}