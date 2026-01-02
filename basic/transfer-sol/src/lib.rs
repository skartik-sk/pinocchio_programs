#![no_std]

use pinocchio::{ProgramResult, account_info::AccountInfo, entrypoint, nostd_panic_handler, program_error::ProgramError, pubkey::Pubkey, sysvars::slot_hashes::log};
use pinocchio_log::log;
use pinocchio_system::instructions::Transfer;

entrypoint!(process_instruction);
nostd_panic_handler!();


fn process_instruction(
    _program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
       log!("Processing instruction {}", instruction_data);
       log!("crate.io {}", _program_id);
       match instruction_data.split_first() {
           Some((0,data))=>process_transfer_program(accounts,data)?,//only for progrom owner account 
           Some((1,data))=>process_trasfer_cpi(accounts,data)?,
           _=>return Err(ProgramError::InvalidInstructionData)
           
       }
       Ok(())
}

pub fn process_transfer_program(
    accounts: &[AccountInfo],
    data: &[u8],
)->ProgramResult{
if data.len()<8 {
    return Err(ProgramError::InvalidInstructionData)
}
let [sender,reciver]= accounts else {
    return Err(ProgramError::NotEnoughAccountKeys);
};
    let amount = u64::from_be_bytes(data[0..8].try_into().map_err(|a| ProgramError::InvalidInstructionData)?);
    log!("befroer {}, {} and amount {}", sender.lamports(), reciver.lamports(), amount );
    *sender.try_borrow_mut_lamports()? -=amount;
    *reciver.try_borrow_mut_lamports()? +=amount;
    log!("after{}, {}", sender.lamports(), reciver.lamports() );
    Ok(())
}

pub fn process_trasfer_cpi(
    accounts: &[AccountInfo],
    data: &[u8],
)->ProgramResult{
    if data.len()<8 {
        return Err(ProgramError::InvalidInstructionData)
    }
    let [sender,reciver,_system]= accounts else {
        return Err(ProgramError::NotEnoughAccountKeys);
    };
    let amount = u64::from_be_bytes(data[0..8].try_into().map_err(|a| ProgramError::InvalidInstructionData)?);
    
    Transfer{
        from:sender,
        to:reciver,
        lamports:amount
    }.invoke()?;
    Ok(())
    
}