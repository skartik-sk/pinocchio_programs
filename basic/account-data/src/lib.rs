#![no_std]

use pinocchio::instruction::{Seed, Signer};
use pinocchio::sysvars::rent::Rent;
use pinocchio::sysvars::slot_hashes::log;
use pinocchio::sysvars::Sysvar;
use pinocchio::{
    account_info::AccountInfo, entrypoint, nostd_panic_handler, program_error::ProgramError,
    pubkey::Pubkey, ProgramResult,
};
use pinocchio_log::log;
use pinocchio_system::instructions::CreateAccount;

entrypoint!(process_instruction);
nostd_panic_handler!();

pub static ID: Pubkey = [
    18, 18, 215, 173, 200, 251, 2, 59, 163, 32, 204, 202, 13, 117, 206, 178, 239, 229, 113, 7, 140,
    213, 252, 187, 193, 44, 236, 248, 139, 173, 47, 53,
];
fn process_instruction(
    _program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    match instruction_data.split_first() {
        Some((0, data)) => {
            //checkes

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
        _ => Err(ProgramError::InvalidInstructionData),
    }
}

///Saving an address with name, house number, street and city in an account
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
