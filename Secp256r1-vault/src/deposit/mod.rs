use pinocchio::{ProgramResult, account_info::AccountInfo, entrypoint, msg, nostd_panic_handler, program_error::ProgramError, pubkey::{Pubkey, find_program_address}};
use pinocchio_secp256r1_instruction::Secp256r1Pubkey;
use pinocchio_system::instructions::Transfer;


pub struct DepositAccounts<'a> { 
    pub owner: &'a AccountInfo,
    pub vault: &'a AccountInfo,
}
 
impl<'a> TryFrom<&'a [AccountInfo]> for DepositAccounts<'a> {
    type Error = ProgramError;
 
    fn try_from(accounts: &'a [AccountInfo]) -> Result<Self, Self::Error> {
        let [owner, vault, _] = accounts else {
            return Err(ProgramError::NotEnoughAccountKeys);
        };
 
        // Accounts Checks
        if !owner.is_signer() {
            return Err(ProgramError::InvalidAccountOwner);
        }
 
        if !vault.is_owned_by(&pinocchio_system::ID) {
            return Err(ProgramError::InvalidAccountOwner);
        }
 
        if vault.lamports().ne(&0) {
            return Err(ProgramError::InvalidAccountData);
        }
 //removing vault_key generation because public key will be given in instruction data. 
       
 
        // Return the accounts
        Ok(Self { owner, vault })
    }
}


#[repr(C)]
pub struct DepositInstructionData {
    pub pubkey: Secp256r1Pubkey,
    pub amount: u64,
}
 
impl<'a> TryFrom<&'a [u8]> for DepositInstructionData {
    type Error = ProgramError;
 
    fn try_from(data: &'a [u8]) -> Result<Self, Self::Error> {
        if data.len() != 41 { //33+8 
            msg!("reach here 1");
            return Err(ProgramError::InvalidInstructionData);
        }
        let (pub_key_byte, amount_byte) = data.split_at(size_of::<Secp256r1Pubkey>());
        let pubkey = Secp256r1Pubkey::try_from(pub_key_byte).unwrap();
        let amount = u64::from_le_bytes(amount_byte.try_into().unwrap());
        
        // Instruction Checks
            msg!("reach here 3");
        if amount.eq(&0) {
            return Err(ProgramError::InvalidInstructionData);
        }
        msg!("reach here 3");
        Ok(Self {
            pubkey,
            amount })
    }
}



pub struct Deposit<'a> {
    pub accounts: DepositAccounts<'a>,
    pub instruction_data: DepositInstructionData,
}
 
impl<'a> TryFrom<(&'a [u8], &'a [AccountInfo])> for Deposit<'a> {
    type Error = ProgramError;
 
    fn try_from((data, accounts): (&'a [u8], &'a [AccountInfo])) -> Result<Self, Self::Error> {
        let accounts = DepositAccounts::try_from(accounts)?;
        let instruction_data = DepositInstructionData::try_from(data)?;
 
        Ok(Self {
            accounts,
            instruction_data,
        })
    }
}
 
impl<'a> Deposit<'a> {
    pub const DISCRIMINATOR: &'a u8 = &0;
 
    pub fn process(&mut self) -> ProgramResult {
        
        let (vault_key, _) = find_program_address(
                    &[
                        b"vault",
                        &self.instruction_data.pubkey[..1],
                        &self.instruction_data.pubkey[1..33]
                    ],
                    &crate::ID
                );
                if vault_key.ne(self.accounts.vault.key()) {
                    return Err(ProgramError::InvalidAccountOwner);
                           }
        Transfer {
            from: self.accounts.owner,
            to:self.accounts.vault,
            lamports: self.instruction_data.amount,
        }
        .invoke()?;
 
        Ok(())
    }
}