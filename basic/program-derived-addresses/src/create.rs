use pinocchio::{ProgramResult, account_info::AccountInfo, instruction::{AccountMeta, Seed}, program_error::{INCORRECT_PROGRAM_ID, ProgramError}, pubkey::{Pubkey, find_program_address}, sysvars::{Sysvar, rent::Rent}};
use pinocchio_system::instructions::{CreateAccount, CreateAccountWithSeed};

use crate::state::PageVisits;



pub fn create_state(
     program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
)->ProgramResult{
    let [owner, pda,_]= accounts else {
        return Err(ProgramError::NotEnoughAccountKeys)
    };
    let seeds = [   Seed::from(b"page_visite") ,Seed::from(owner.key())];
    let (_,bump)=find_program_address(&[    b"page_viste",owner.key()] ,program_id);
    let rent = Rent::get()?.minimum_balance(PageVisits::SPACE);
 CreateAccount{
            from:owner,
            to:pda,
            lamports:rent,
            space:5,
            owner:program_id,
            
        }.invoke_signed([owner]) ?;
 pda.new(bump)?;
    Ok(())
}