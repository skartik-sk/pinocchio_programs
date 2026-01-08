#![no_std]

use pinocchio::{ProgramResult, account_info::AccountInfo, entrypoint, nostd_panic_handler, program_error::ProgramError, pubkey::Pubkey, sysvars::slot_hashes::log};
use pinocchio_log::log;

use crate::{create::create_new_account, init::init_rent_vault};

entrypoint!(process_instruction);
nostd_panic_handler!();

pub mod create;
pub mod state;
pub mod init;
fn process_instruction(
    _program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
       log!("Processing instruction {}", instruction_data);
       log!("crate.io {}", _program_id);
        match instruction_data.split_first() {
        Some((0, data)) => {init_rent_vault(_program_id, accounts, data)?;},
        Some((1, _)) => {create_new_account(_program_id, accounts)?;},
        _ => return  Err(ProgramError::InvalidInstructionData),
    }
       Ok(())
}
