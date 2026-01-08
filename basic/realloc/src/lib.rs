#![no_std]

use pinocchio::{ProgramResult, account_info::AccountInfo, entrypoint, nostd_panic_handler, program_error::ProgramError, pubkey::Pubkey, sysvars::slot_hashes::log};
use pinocchio_log::log;

use crate::{create::create_address_info, realocate::{reallocate_without_zero_init, reallocate_zero_init}};

entrypoint!(process_instruction);
nostd_panic_handler!();

pub mod realocate;
pub mod create;
pub mod state;
fn process_instruction(
    _program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
       log!("Processing instruction {}", instruction_data);
       log!("crate.io {}", _program_id);
       match instruction_data.split_first() {
        Some((0, data)) => {create_address_info(_program_id, accounts, data)?;},
        Some((1, data)) =>{ reallocate_without_zero_init(accounts, data)?;},
        Some((2, data)) =>{ reallocate_zero_init(accounts, data)?;},
        _ =>return  Err(ProgramError::InvalidInstructionData),
    }
    Ok(())
}
