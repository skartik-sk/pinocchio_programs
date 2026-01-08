#![no_std]

use pinocchio::{ProgramResult, account_info::AccountInfo, entrypoint, nostd_panic_handler, program_error::ProgramError, pubkey::Pubkey, sysvars::slot_hashes::log};
use pinocchio_log::log;

entrypoint!(process_instruction);
nostd_panic_handler!();
pub mod state;
pub mod create;
pub mod increment;
use create::*;
use increment::*;
fn process_instruction(
    _program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
       log!("Processing instruction {}", instruction_data);
       log!("crate.io {}", _program_id);
       match instruction_data.split_first() {
           Some((0,data))=>{create_state(_program_id, accounts, data).unwrap()},
           Some((1,_))=>{increment_state(accounts).unwrap()},
           _=>return Err(ProgramError::InvalidInstructionData)
       }
       Ok(())
}
