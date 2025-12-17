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
       match instruction_data.split_first(){
        Some((0,data))=>initilize(   _program_id,accounts,data),
        Some((1,data))=>lever(_program_id, accounts),

        _=> return Err(ProgramError::InvalidInstructionData)
       }?;

       Ok(())
}


pub struct PowerName{
    pub name:[u8; 39]
}

pub struct PowerStatus{
    pub is_on:[u8; 1]
}

pub fn initilize(
      _program_id: &Pubkey,
    accounts: &[AccountInfo],
    data:&[u8],
)->ProgramResult{
    
let [owner, newacc,_]= accounts else{
    return Err(ProgramError::NotEnoughAccountKeys)
};

if data.len()!=40{
    return Err(ProgramError::InvalidInstructionData)
}
let minlamport = Rent::get()?.minimum_balance(40);
       CreateAccount{
        from:owner,
        to:newacc,
        lamports:minlamport,
        space:40,//1+39
        owner:_program_id
       }.invoke()?;
       newacc.try_borrow_mut_data()?.
       copy_from_slice(data);
       Ok(())
}



pub fn lever( _program_id: &Pubkey,
    accounts: &[AccountInfo])->ProgramResult{

let [_owner, PowerAcc,_]= accounts else{
    return Err(ProgramError::NotEnoughAccountKeys)
};



let mut powerstatus = PowerAcc.try_borrow_mut_data()?;
if powerstatus.is_empty() {
    return Err(ProgramError::InvalidInstructionData);
}
match powerstatus[0] {
    0 => powerstatus[0] = 1,
    1 => powerstatus[0] = 0,
    _ => return Err(ProgramError::InvalidInstructionData),
}


Ok(())
    }
