use pinocchio::{ProgramResult, account_info::AccountInfo, instruction::{AccountMeta, Seed, Signer}, log, msg, program_error::{INCORRECT_PROGRAM_ID, ProgramError}, pubkey::{Pubkey, find_program_address}, sysvars::{Sysvar, rent::Rent}};
use pinocchio_log::log;
use pinocchio_system::instructions::{CreateAccount, CreateAccountWithSeed};

use crate::state::PageVisits;
pub fn create_state(
     program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
)->ProgramResult{
    let [payer , user, pda,_]= accounts else {
        return Err(ProgramError::NotEnoughAccountKeys)
    };
let bump =    &instruction_data[4..5];

 let seeds = [
     Seed::from(PageVisits::SEED.as_bytes()),
     Seed::from(user.key().as_ref()),
     Seed::from(bump),
 ];

 let signers = Signer::from(&seeds);
    
    let rent = Rent::get()?.minimum_balance(PageVisits::SPACE);
        CreateAccount{
            from:payer,
            to:pda,
            lamports:rent,
            space:PageVisits::SPACE as u64,
            owner:program_id,
        }.invoke_signed(&[signers]) ?;
 pda.try_borrow_mut_data()?.copy_from_slice(&instruction_data);
    Ok(())
}



    