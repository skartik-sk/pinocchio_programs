use pinocchio::{ProgramResult, account_info::AccountInfo, instruction::Seed, program_error::{INCORRECT_PROGRAM_ID, ProgramError}, pubkey::{Pubkey, find_program_address}, sysvars::{Sysvar, rent::Rent}};
use pinocchio_system::instructions::CreateAccountWithSeed;

use crate::state::PageVisits;



pub fn create_state(
     program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
)->ProgramResult{
    let [owner, _]= accounts else {
        return Err(ProgramError::NotEnoughAccountKeys)
    };
    let seeds = [    Seed::from(b"page_viste"),Seed::from(owner.key())];
    let (pda,bump)=find_program_address(&seeds ,program_id);
    let rent = Rent::get()?.minimum_balance(PageVisits::SPACE);
    Seed::from(value)
    CreateAccountWithSeed{
        from:owner,
        to:pda,
        seed:seeds,
        lamports:rent,
        space:5,
        owner:program_id,
        
    };
    
    Ok(())
}