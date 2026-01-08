use core::str::from_utf8;

use bytemuck::from_bytes;
use pinocchio::{ProgramResult, account_info::AccountInfo, program_error::{ProgramError}, pubkey::Pubkey};
use pinocchio_log::log;

use crate::state::Favorites;

pub fn get_fav_fn(
    accounts: &[AccountInfo])->ProgramResult{
        let [pda, owner]=accounts else {
            return Err(ProgramError::NotEnoughAccountKeys);
        };
        
        log!("get_fav for {}", pda.key());
        
        if pda.data_is_empty(){
            log!("ERROR: PDA data is empty");
            return Err(ProgramError::InvalidAccountData);
        }
        
        let total_bytes = pda.try_borrow_data()?;
        
        if total_bytes.len() < Favorites::SIZE {
            log!("ERROR: Account data too small. Expected 112 bytes, got {}", total_bytes.len());
            return Err(ProgramError::InvalidAccountData);
        }
        
        let received_bytes = &total_bytes[0..32];
        
        log!("Checking owner. Expected: {}, Found in data: {}", owner.key(), received_bytes);
        
        if received_bytes != owner.key().as_ref() {
            log!("ERROR: Owner mismatch!");
            return Err(ProgramError::InvalidAccountOwner);
        }
        
        let data: &Favorites = from_bytes(&total_bytes[0..112]);
        log!("Successfully deserialized data");
        
        let number = data.number;
        log!("Number: {}", number);

        let color = from_utf8(&data.color).map_err(|_| ProgramError::InvalidAccountData)?;
        log!("Color: {}", color);
        
        let hobby1 = from_utf8(&data.hobbies[0]).map_err(|_| ProgramError::InvalidAccountData)?;
        let hobby2 = from_utf8(&data.hobbies[1]).map_err(|_| ProgramError::InvalidAccountData)?;
        let hobby3 = from_utf8(&data.hobbies[2]).map_err(|_| ProgramError::InvalidAccountData)?;
        let hobby4 = from_utf8(&data.hobbies[3]).map_err(|_| ProgramError::InvalidAccountData)?;
        
        log!("Hobbies: {} {} {} {}", hobby1, hobby2, hobby3, hobby4);
        
        log!(
            "User {}'s favorite number is {}, favorite color is: {}, and their hobbies are {} {} {} {}",
            owner.key(),
            number,
            color,
            hobby1,
            hobby2,
            hobby3,
            hobby4,
        );
        
    Ok(())
}