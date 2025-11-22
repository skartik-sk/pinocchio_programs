use core::u8;

use pinocchio::{ProgramResult, account_info::{AccountInfo, Ref}, entrypoint, instruction::{Seed, Signer}, nostd_panic_handler, program_error::ProgramError, pubkey::{Pubkey, find_program_address}, sysvars::{Sysvar, clock::Clock, instructions::{Instructions, IntrospectedInstruction}}};
use pinocchio_secp256r1_instruction::{Secp256r1Instruction, Secp256r1Pubkey};
use pinocchio_system::instructions::Transfer;
 
pub struct WithdrawAccounts<'a> {
    pub owner: &'a AccountInfo,
    pub vault: &'a AccountInfo,
      pub instructions: &'a AccountInfo,
   
}
 
impl<'a> TryFrom<&'a [AccountInfo]> for WithdrawAccounts<'a> {
    type Error = ProgramError;
 
    fn try_from(accounts: &'a [AccountInfo]) -> Result<Self, Self::Error> {
        let [owner, vault, instructions, _] = accounts else {
            return Err(ProgramError::NotEnoughAccountKeys);
        };
 
        // Basic Accounts Checks
        if !owner.is_signer() {
            return Err(ProgramError::InvalidAccountOwner);
        }
 
        if !vault.is_owned_by(&pinocchio_system::ID) {
            return Err(ProgramError::InvalidAccountOwner);
        }
 
        if vault.lamports().eq(&0) {
            return Err(ProgramError::InvalidAccountData);
        }
 

 
        Ok(Self { owner, vault, instructions })
    }
}

pub struct WithdrawInstructionData {
 pub bump: [u8; 1],
}
impl<'a> TryFrom<&'a [u8]> for WithdrawInstructionData {
    type Error = ProgramError;
    fn try_from(data: &'a [u8]) -> Result<Self, Self::Error> {
        Ok(Self {
                    bump: [*data.first().ok_or(ProgramError::InvalidInstructionData)?],
                })
    }
}

pub struct Withdraw<'a> {
    pub accounts: WithdrawAccounts<'a>,
    pub instruction_data: WithdrawInstructionData,
}
 
impl<'a> TryFrom<(&'a [u8],&'a [AccountInfo])> for Withdraw<'a> {
    type Error = ProgramError;
 
    fn try_from((data,accounts): (&'a [u8],&'a [AccountInfo])) -> Result<Self, Self::Error> {
        let accounts = WithdrawAccounts::try_from(accounts)?;
        let instruction_data = WithdrawInstructionData::try_from(data)?;
 
        Ok(Self { accounts, instruction_data })
    }
}
 
impl<'a> Withdraw<'a> {
    pub const DISCRIMINATOR: &'a u8 = &1;
 
    pub fn process(&mut self) -> ProgramResult {
        // Deserialize our instructions
               let instructions  = Instructions::try_from(self.accounts.instructions)?;
               // Get instruction directly after this one
               let ix = instructions.get_instruction_relative(1)?;
               // Get Secp256r1 instruction
               let secp256r1_ix =  Secp256r1Instruction::try_from(ix.get_instruction_data()).unwrap() ;
               // Enforce that we only have one signature
               if secp256r1_ix.num_signatures() != 1 {
                   return Err(ProgramError::InvalidInstructionData);
               }
               // Enforce that the signer of the first signature is our PDA owner
               let signer: Secp256r1Pubkey = *secp256r1_ix.get_signer(0).unwrap();
               // Check that our fee payer is the correct 
               let (payer, expiry) = secp256r1_ix
                   .get_message_data(0).unwrap()
                   .split_at_checked(32)
                   .ok_or(ProgramError::InvalidInstructionData)?;
               if self.accounts.owner.key().ne(payer) {
                   return Err(ProgramError::InvalidAccountOwner);
               }
               // Get current timestamp
               let now = Clock::get()?.unix_timestamp;
               // Get signature expiry timestamp
               let expiry = i64::from_le_bytes(
                   expiry
                       .try_into()
                       .map_err(|_| ProgramError::InvalidInstructionData)?
               );
               if now > expiry {
                   return Err(ProgramError::InvalidInstructionData);
               }
        
        // Create PDA signer seeds
        let seeds = [
            Seed::from(b"vault"),
            Seed::from(signer[..1].as_ref()),
                Seed::from(signer[1..].as_ref()),
            Seed::from(&self.instruction_data.bump),
        ];
        let signers = [Signer::from(&seeds)];
 
        // Transfer all lamports from vault to owner
        Transfer {
            from: self.accounts.vault,
            to: self.accounts.owner,
            lamports: self.accounts.vault.lamports(),
        }
        .invoke_signed(&signers)?;
 
        Ok(())
    }
}