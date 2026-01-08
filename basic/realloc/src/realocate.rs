use pinocchio::{
    account_info::AccountInfo,
    program_error::ProgramError,
    sysvars::{rent::Rent, Sysvar},
    ProgramResult,
};
use pinocchio_system::instructions::Transfer;

use crate::state::{EnhancedAddressInfo, WorkInfo};

pub fn reallocate_without_zero_init(
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    let [to_acc, payer, _] = accounts else {
        return Err(ProgramError::NotEnoughAccountKeys);
    };

    let account_span = EnhancedAddressInfo::LEN;
    let lamports_required = (Rent::get()?).minimum_balance(account_span);

    let diff = lamports_required - *to_acc.try_borrow_lamports()?;

    Transfer {
        from: payer,
        to: to_acc,
        lamports: diff,
    }
    .invoke()?;

    to_acc.resize(account_span)?;

    let mut to_acc_data = to_acc.try_borrow_mut_data()?;
    to_acc_data[25..37].copy_from_slice(instruction_data);

    Ok(())
}

pub fn reallocate_zero_init(accounts: &[AccountInfo], data: &[u8]) -> ProgramResult {
    let [to_acc] = accounts else {
        return Err(ProgramError::NotEnoughAccountKeys);
    };

    let account_span = WorkInfo::LEN;

    to_acc.resize(account_span)?;

    let mut to_acc_data = to_acc.try_borrow_mut_data()?;
    to_acc_data.copy_from_slice(data);

    Ok(())
}