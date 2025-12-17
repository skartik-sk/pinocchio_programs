#![no_std]

extern crate alloc;
use alloc::vec::Vec;

use pinocchio::{ProgramResult, account_info::AccountInfo, cpi::{invoke, slice_invoke}, entrypoint, instruction::{AccountMeta, Instruction}, nostd_panic_handler, program_error::ProgramError, pubkey::Pubkey, sysvars::{Sysvar, rent::Rent, slot_hashes::log}};
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
      log!("doing cpi into lever status");
     

      match accounts.split_first(){
        Some((leveracc,rest))=>{
          
            let accounts_meta: Vec<AccountMeta> = rest.iter()
                .map(|ai| AccountMeta::from(ai))
                .collect();
            let acckey:Vec<&AccountInfo>=rest.iter()
            .map(|a|a ).collect();
           let ix= Instruction{
                program_id:leveracc.key(),
                data:instruction_data,
                accounts: &accounts_meta
            };

            slice_invoke(&ix, acckey.as_slice())?;

        }
        _=>return Err(ProgramError::NotEnoughAccountKeys)
      }


       Ok(())
}
