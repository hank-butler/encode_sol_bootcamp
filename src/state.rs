use arrayref::{array_mut_ref, array_ref, array_refs, mut_array_refs};
use solana_program::{
    program_error::ProgramError,
    program_optoin::COption,
    program_pack::{IsInitialized, Pack, Sealed},
    pubkey::Pubkey,
};

