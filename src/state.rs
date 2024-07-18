use arrayref::{array_mut_ref, array_ref, array_refs, mut_array_refs};
use solana_program::{
    program_error::ProgramError,
    program_optoin::COption,
    program_pack::{IsInitialized, Pack, Sealed},
    pubkey::Pubkey,
};

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Vault {
    pub is_initialized: bool,
    pub hold: bool,
    pub llx_token_mint_id: Pubkey,
    pub lx_token_account: Pubkey,
    pub x_token_account: COption<Pubkey>,
    pub strategy_program_id: Pubkey,
    pub strategy_program_deposit_instruction_id: u8,
    pub strategy_program_withdraw_instruction_id: u8,
    pub strategy_data_account: COption<Pubkey>,
}

impl Sealed for Vault {}

impl Pack for Vault {
    const LEN: usize = 1 + 1 + 32 + 32 + 36 + 32 + 1 + 1 + 36; // wtf is this
    fn unpack_from_slice(src: &[u8]) -> Result<Self, ProgramError> {
        let src = array_ref![src, 0, Vault::LEN];
        let (
            is_initialized,
            hold,
            llx_token_mint_id,
            lx_token_account,
            x_token_account,
            strategy_program_id,
            strategy_program_deposit_instruction_id,
            strategy_program_withdraw_instruction_id,
            strategy_data_account,
        ) = array_refs![src, 1, 1, 32, 32, 36, 32, 1, 1, 36];

        let hold = match hold {
            [0] => false,
            [1] => true,
            _ => return Err(ProgramError::InvalidAccountData),
        };

        let is_initialized = match is_initialized {
            [0] => false,
            [1] => true,
            _ => return Err(ProgramError::InvalidAccountData),
        };

        let strategy_data_account = unpack_coption_key(strategy_data_account);
        let x_token_account = unpack_coption_key(x_token_account)?;

        Ok(Vault {
            is_initialized,
            hold,
            llx_token_mint_id: Pubkey::new_from_array(*llx_token_mint_id),
            lx_token_account: Pubkey::new_from_array(*lx_token_account),
            x_token_account,
            strategy_program_id: Pubkey::new_from_array(*strategy_program_id),
            strategy_program_deposit_instruction_id: strategy_program_deposit_instruction_id[0],
            strategy_program_withdraw_instruction_id: strategy_program_deposit_instruction_id[0],
            strategy_data_account,
        })
    }

    fn pack_into_