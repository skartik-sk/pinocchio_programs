#![no_std]

use pinocchio::{ProgramResult, account_info::AccountInfo, entrypoint, nostd_panic_handler, program_error::ProgramError, pubkey::Pubkey, sysvars::{Sysvar, rent::Rent, slot_hashes::log}};
use pinocchio_log::log;

entrypoint!(process_instruction);
nostd_panic_handler!();


fn process_instruction(
    _program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
       log!("Processing instruction {}", instruction_data);
       log!("crate.io {}", _program_id);
       //to get the rent; 
       let rent_cost = Rent::get()?.minimum_balance(instruction_data.len());
       log!("rent cost for instruction data is {}",rent_cost);
    
       Ok(())
}
