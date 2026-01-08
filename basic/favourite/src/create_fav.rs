use pinocchio::{
    ProgramResult, account_info::AccountInfo, instruction::{Seed, Signer}, program_error::ProgramError, pubkey::{Pubkey, find_program_address}, sysvars::{Sysvar, rent::Rent}
};
use pinocchio_log::log;
use pinocchio_system::instructions::CreateAccount;

use crate::state::Favorites;

pub fn create_fav_fn(program_id: &Pubkey, accounts: &[AccountInfo], data: &[u8]) -> ProgramResult {
    let [pda, payer, _] = accounts else {
        return Err(ProgramError::NotEnoughAccountKeys);
    };
    if data.len() < Favorites::SIZE {
        return Err(ProgramError::InvalidInstructionData);
    }
    if !pda.try_borrow_data()?.is_empty() {
        return Err(ProgramError::AccountAlreadyInitialized);
    }
    let sizee = Rent::get()?.minimum_balance(Favorites::SIZE);
    let (dir_pda , dir_bump)=find_program_address(&["my_fav".as_bytes(),payer.key().as_ref()], program_id);
    if pda.key().ne(&dir_pda){
        log!("key not matched ");
        return Err(ProgramError::InvalidSeeds);
    }
    let bump_byte =&[dir_bump];
    let seeds = [Seed::from("my_fav".as_bytes()), Seed::from(payer.key().as_ref()), Seed::from(bump_byte)];
    let signers = Signer::from(&seeds);
    CreateAccount {
        from: payer,
        to: pda,
        lamports: sizee,
        space: Favorites::SIZE as u64,
        owner: program_id,
    }
    .invoke_signed(&[signers])
    .unwrap();

    let mut pda_data = pda.try_borrow_mut_data()?;
    pda_data.copy_from_slice(data);

    Ok(())
}
