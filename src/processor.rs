use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult,
    msg,
    program::{invoke, invoke_signed},
    program_error::ProgramError,
    program_option::COption,
    program_pack::{IsInitialized, Pack},
    pubkey::Pubkey,
    sysvar::{rent::Rent, Sysvar},
  };
  
  use crate::{error::VaultError, instruction::VaultInstruction, state::Vault};
  
  pub struct Processor;
  impl Processor {
    pub fn process(
    }
  
    fn process_initialize_vault(
    }
  
    fn process_transfer(
      program_id: &Pubkey,
      accounts: &[AccountInfo],
      amount: u64,
      is_deposit: bool,
    ) -> ProgramResult {
      msg!("Transferring");
      let account_info_iter = &mut accounts.iter();
  
      let token_program = next_account_info(account_info_iter)?;
      let source_token_account = next_account_info(account_info_iter)?;
      let target_token_account = next_account_info(account_info_iter)?;
  
      // Additional account metas:
      let source_authority = next_account_info(account_info_iter)?;
      let storage_account = next_account_info(account_info_iter)?;
  
      let storage_info = Vault::unpack_unchecked(&storage_account.data.borrow())?;
      if !storage_info.is_initialized() {
        msg!("Storage not configured!");
        return Err(VaultError::InvalidInstruction.into());
      }
  
      // Charge fees
      if is_deposit {
        // TODO(001): implement.
        msg!("Mint lX tokens to client account");
      } else {
        // TODO(002): implement.
        msg!("Transfer & burn lX tokens from client");
      }
  
      // Check if this is a HODL Vault; if so, we deposit & withdraw from 
      if storage_info.hodl {
        let x_token_account = next_account_info(account_info_iter)?;
        msg!("Calling the token program to transfer tokens");
        if is_deposit {
          let transfer_to_vault_ix = spl_token::instruction::transfer(
            token_program.key,
            source_token_account.key,
            x_token_account.key,
            &source_authority.key,
            &[&source_authority.key],
            amount,
          )?;
          msg!("Depositing to hodl account");
          invoke(
            &transfer_to_vault_ix,
            &[
              source_token_account.clone(),
              x_token_account.clone(),
              source_authority.clone(),
              token_program.clone(),
            ],
          )?;
        } else {
          let (pda, bump_seed) = Pubkey::find_program_address(&[b"vault"], program_id);
          let transfer_to_client_ix = spl_token::instruction::transfer(
            token_program.key,
            x_token_account.key,
            target_token_account.key,
            &pda,
            &[&pda],
            amount,
          )?;
          msg!("Withdrawing from hodl account");
          invoke_signed(
            &transfer_to_client_ix,
            &[
              x_token_account.clone(),
              target_token_account.clone(),
              source_authority.clone(),
              token_program.clone(),
            ],
            &[&[&b"vault"[..], &[bump_seed]]],
          )?;
        }
      }
      else {
        if is_deposit {
          // TODO(003): implement.
          msg!("Depositing into strategy");
        } else {
          // TODO(003): implement.
          msg!("Withdrawing from strategy");
        }
      }
      Ok(())
    }
