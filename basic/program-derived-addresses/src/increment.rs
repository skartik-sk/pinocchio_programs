use pinocchio::{ProgramResult, account_info::AccountInfo, program_error::ProgramError, pubkey::Pubkey};

pub fn increment_state(
    accounts: &[AccountInfo],
)->ProgramResult{
    let [pda] = accounts else {
           return Err(ProgramError::NotEnoughAccountKeys);
       };
   
       let mut pda_byte = pda.try_borrow_mut_data()?;
   
       let mut page_visits = u32::from_le_bytes(
           pda_byte[0..4]
               .try_into()
               .map_err(|_| ProgramError::InvalidAccountData)?,
       );
   
       page_visits += 1;
   
       pda_byte[0..4].copy_from_slice(&page_visits.to_le_bytes());

    Ok(())

    
}