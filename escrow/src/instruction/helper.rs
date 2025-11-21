use pinocchio::instruction::{Seed, Signer};
use pinocchio::msg;
use pinocchio::sysvars::Sysvar;
use pinocchio::sysvars::rent::Rent;
use pinocchio::{
    account_info::AccountInfo, program_error::ProgramError, pubkey::find_program_address,
};
use pinocchio_associated_token_account::instructions::Create;
use pinocchio_system::instructions::CreateAccount;

use crate::Escrow;
use crate::ID;

pub struct SignerAccount;
impl SignerAccount {
    pub fn check(account: &AccountInfo) -> Result<(), ProgramError> {
        if !account.is_signer() {
            return Err(ProgramError::InvalidAccountOwner);
        }
        Ok(())
    }
}

pub struct MintInterface;
impl MintInterface {
    pub fn check(account: &AccountInfo) -> Result<(), ProgramError> {
        if !account.is_owned_by(&pinocchio_token::ID) {
            return Err(ProgramError::IllegalOwner);
        }
     

        if account.data_len() != pinocchio_token::state::Mint::LEN {
            return Err(ProgramError::InvalidAccountData);
        }

        Ok(())
    }
}

pub struct TokenAccount;

impl TokenAccount {
    pub fn check(account: &AccountInfo) -> Result<(), ProgramError> {
        if !account.is_owned_by(&pinocchio_token::ID) {
            return Err(ProgramError::IllegalOwner);
        }

        if account
            .data_len()
            .ne(&pinocchio_token::state::TokenAccount::LEN)
        {
            return Err(ProgramError::InvalidAccountData);
        }

        Ok(())
    }
}

fn check_token_program(token_prog: &AccountInfo) -> Result<(), ProgramError> {
    if token_prog.key() != &pinocchio_token::ID {
        return Err(ProgramError::IncorrectProgramId);
    }
    Ok(())
}

fn check_system_program(system_prog: &AccountInfo) -> Result<(), ProgramError> {
    if system_prog.key() != &pinocchio_system::ID {
        return Err(ProgramError::IncorrectProgramId);
    }
    Ok(())
}

pub struct AssociatedTokenAccount;
impl AssociatedTokenAccount {
    pub fn check(
        account: &AccountInfo,
        authority: &AccountInfo,
        mint: &AccountInfo,
        token_program: &AccountInfo,
    ) -> Result<(), ProgramError> {
        TokenAccount::check(account)?;
        MintInterface::check(mint)?;
        check_token_program(token_program)?;

        let derivedata = find_program_address(
            &[authority.key(), token_program.key(), mint.key()],
            &pinocchio_associated_token_account::ID,
        );
        
        if derivedata.0.ne(account.key()) {
            return Err(ProgramError::InvalidAccountData);
        }
        Ok(())
    }

    pub fn init(
        account: &AccountInfo,
        mint: &AccountInfo,
        payer: &AccountInfo,
        owner: &AccountInfo,
        system_program: &AccountInfo,
        token_program: &AccountInfo,
    ) -> Result<(), ProgramError> {
        MintInterface::check(mint)?;
        SignerAccount::check(payer)?;
        check_system_program(system_program)?;
        check_token_program(token_program)?;

        Create {
            funding_account: payer,
            account: account,
            wallet: owner,
            mint: mint,
            system_program: system_program,
            token_program: token_program,
        }
        .invoke()?;

        Ok(())
    }

    pub fn init_if_needed( account: &AccountInfo,
        mint: &AccountInfo,
        payer: &AccountInfo,
        owner: &AccountInfo,
        system_program: &AccountInfo,
        token_program: &AccountInfo,)-> Result<(), ProgramError> {
match Self::check(account, owner, mint, token_program) {
    Ok(_)=> Ok(()),
    Err(_)=> Self::init(account, mint, payer, owner, system_program, token_program),

}

        }
}

pub struct ProgramAccount;

impl ProgramAccount {
    pub fn check(account: &AccountInfo) -> Result<(), ProgramError> {
        if !account.is_owned_by(&ID) {
            return Err(ProgramError::IllegalOwner);
        }
        if account.data_len() != Escrow::LEN {
            return Err(ProgramError::InvalidAccountData);
        }
        Ok(())
    }

    pub fn init<'a, T: Sized>(
        payer: &AccountInfo,
        account: &AccountInfo,
        seeds: &[Seed<'a>],
        space: usize,
    ) -> Result<(), ProgramError> {
        let lamports = Rent::get()?.minimum_balance(space);

        let signer = [Signer::from(seeds)];

        CreateAccount {
            from: payer,
            to: account,
            lamports,
            space: space as u64,
            owner: &crate::ID,
        }
        .invoke_signed(&signer)?;
        Ok(())
    }

    pub fn close(account: &AccountInfo, refund_account: &AccountInfo)->Result<(), ProgramError> {
        let source_lamports = account.lamports();
*refund_account.try_borrow_mut_lamports()? += source_lamports;
    *account.try_borrow_mut_lamports()? = 0;
         account.resize(0)?;
account.close()?;
Ok(())
    }
}
