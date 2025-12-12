#![no_std]

use pinocchio::{ProgramResult, account_info::AccountInfo, entrypoint, nostd_panic_handler, program_error::ProgramError, pubkey::Pubkey, sysvars::{Sysvar, rent::Rent, slot_hashes::log}};
use pinocchio_log::log;
use pinocchio_system::instructions::CreateAccount;

entrypoint!(process_instruction);
nostd_panic_handler!();

pub static ID:Pubkey=[
  42,165,143,7,168,12,33,122,131,101,49,8,48,61,142,209,8,125,123,223,11,50,71,35,149,156,79,17,151,2,35,82
];
fn process_instruction(
    _program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
       log!("Processing instruction {}", instruction_data);
       log!("crate.io {}", _program_id);
       match instruction_data.split_first() {
           Some((0,data))=> create_account(
               accounts,data
           ),
           Some((1,_))=> close_account(accounts),

           _=>Err(ProgramError::InvalidInstructionData)
       }
}

pub fn create_account( accounts: &[AccountInfo],
data: &[u8],)->ProgramResult{

    let [userDataAdd, owner, system] = accounts else {
                 return Err(ProgramError::NotEnoughAccountKeys);
             };

             if !owner.is_signer() {
                 return Err(ProgramError::IllegalOwner);
             }
             if userDataAdd.lamports().ne(&0) {
                 return Err(ProgramError::AccountAlreadyInitialized);
             }

             if system.key().ne(&pinocchio_system::ID) {
                 return Err(ProgramError::InvalidAccountData);
             }
             log!("data length{}", data.len());
             if data.len() < UserData::LEN {
                 return Err(ProgramError::InvalidInstructionData);
             }
             let lamports_req = Rent::get()?.minimum_balance(UserData::LEN);
             CreateAccount {
                 from: &owner,
                 to: userDataAdd,
                 lamports: lamports_req,
                 space: UserData::LEN as u64,
                 owner: &ID,
             }
             .invoke()?;

             // .invoke_signed(&[Signer::from(&[Seed::from(owner.key().as_ref())])])?;
             log!("{}", lamports_req);
             userDataAdd.try_borrow_mut_data()?.copy_from_slice(data);
    Ok(())
}
pub fn close_account( accounts: &[AccountInfo])->ProgramResult{
let [target_account, payer, system] = accounts else {
             return Err(ProgramError::NotEnoughAccountKeys);
};
                let diff = target_account.lamports();

                *target_account.try_borrow_mut_lamports()? -= diff;
                *payer.try_borrow_mut_lamports()? += diff;

                target_account.resize(0)?;

                unsafe {
                    target_account.assign(system.key());
                }
    Ok(())
}




#[repr(C)]
pub struct UserData {
    pub name: [u8; 40],
    pub house_number: [u8; 10],
    pub street: [u8; 150],
    pub city: [u8; 50],
}

impl UserData {
    const LEN: usize = 40 + 10 + 150 + 50;
}
