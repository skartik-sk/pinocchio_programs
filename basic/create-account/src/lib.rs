#![no_std]

use pinocchio::{ProgramResult, account_info::AccountInfo, entrypoint, nostd_panic_handler, program_error::ProgramError, pubkey::Pubkey, sysvars::{Sysvar, rent::Rent, slot_hashes::log}};
use pinocchio_log::log;
use pinocchio_system::instructions::CreateAccount;

entrypoint!(process_instruction);
nostd_panic_handler!();


fn process_instruction(
    _program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
       log!("Processing instruction {}", instruction_data);
       log!("crate.io {}", _program_id);
let [owner, newacc,_]= accounts else{
    return Err(ProgramError::NotEnoughAccountKeys)
};
let minlamport = Rent::get()?.minimum_balance(0);
       CreateAccount{
        from:owner,
        to:newacc,
        lamports:minlamport,
        space:0,
        owner:&pinocchio_system::ID
       }.invoke()?;

       Ok(())
}
