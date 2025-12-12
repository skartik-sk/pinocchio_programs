#![no_std]

use pinocchio::{ProgramResult, account_info::AccountInfo, entrypoint, nostd_panic_handler, program_error::ProgramError, pubkey::Pubkey, sysvars::{ Sysvar, rent::Rent, slot_hashes::log}};
use pinocchio_log::log;
use pinocchio_system::instructions::CreateAccount;

entrypoint!(process_instruction);
nostd_panic_handler!();

pub static ID:Pubkey=[94,95,60,240,19,241,67,58,116,199,174,23,141,53,86,173,172,146,104,251,201,45,71,92,165,218,163,154,97,197,81,98];
fn process_instruction(
    _program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
       log!("Processing instruction {}", instruction_data);
       log!("crate.io {}", _program_id);
       match instruction_data.split_first(){
           Some((0,_))=>create_counter(accounts),
           Some((1,_))=>increment_counter(accounts),
           _=>Err(ProgramError::InvalidInstructionData)
           
       }?;
       Ok(())

}

pub struct Counter{
    pub count:u64
}

fn create_counter(accounts: &[AccountInfo])->ProgramResult{
   let [counter,payer,sys]=accounts else {
       return Err(ProgramError::NotEnoughAccountKeys)
   };
   
   if !payer.is_signer(){
       return Err(ProgramError::IllegalOwner)
   }

   
   CreateAccount{
       from:payer,
       to:counter,
       lamports:Rent::get()?.minimum_balance(8),
       space:8,
       owner:&ID
       
       
   }.invoke()?;
   
   counter.try_borrow_mut_data()?.fill(0);
   Ok(())
}

fn increment_counter(accounts: &[AccountInfo])->ProgramResult{
    let [counter]=accounts else {
        return Err(ProgramError::NotEnoughAccountKeys)
    };
    if !counter.is_writable(){
        return Err(ProgramError::AccountBorrowFailed)
    }
    
    let mut count =   counter.try_borrow_mut_data()?;
    let count_byte =count[0..8].try_into().map_err(|_| ProgramError::InvalidInstructionData)?;
    let counter = u64::from_le_bytes(count_byte);
    
        // Increment the counter
        let new_counter = counter + 2;
    
        // Write the new counter value back
        count[0..8].copy_from_slice(&new_counter.to_le_bytes());
    
    Ok(())
}
