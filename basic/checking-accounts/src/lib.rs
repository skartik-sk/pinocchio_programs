#![no_std]

use pinocchio::{
    account_info::AccountInfo, entrypoint, nostd_panic_handler, program_error::ProgramError,
    pubkey::Pubkey, sysvars::slot_hashes::log, ProgramResult,
};
use pinocchio_log::log;

entrypoint!(process_instruction);
nostd_panic_handler!();

pub static ID: Pubkey = [
    172, 83, 11, 121, 200, 119, 209, 42, 103, 188, 187, 212, 145, 62, 76, 239, 252, 20, 11, 95,
    222, 30, 225, 34, 42, 184, 4, 108, 14, 118, 145, 116,
];

fn process_instruction(
    _program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    log!("Processing instruction {}", instruction_data);
    log!("crate.io {}", _program_id);
let [ account_key, program_key]=accounts else {
    return Err(ProgramError::NotEnoughAccountKeys)
};
if !account_key.is_signer() {
     return Err(ProgramError::InvalidAccountOwner)
}
if !account_key.is_owned_by(&pinocchio_system::ID) {
     return Err(ProgramError::InvalidAccountOwner)
}
if !account_key.is_writable() {
     return Err(ProgramError::InvalidAccountData)
}if account_key.lamports().eq(&0) {
     return Err(ProgramError::InsufficientFunds)
}
if !program_key.key().eq(&ID) {
     return Err(ProgramError::InvalidArgument)
}

    Ok(())
}
