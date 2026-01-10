#![no_std]

use pinocchio::{ProgramResult, account_info::AccountInfo, entrypoint, nostd_panic_handler, program_error::ProgramError, pubkey::Pubkey, sysvars::slot_hashes::log};
use pinocchio_log::log;
use pinocchio_system::create_account_with_minimum_balance;

entrypoint!(process_instruction);
nostd_panic_handler!();
#[repr(C)]
#[derive(Pod,Zeroable,Clone, Copy)]
struct CreateTokenArgs{
    pub token_title: [u8;10],
        pub token_symbol: [u8;4],
        pub token_uri: [u8;100],
        pub token_decimals: u8,
}
impl CreateTokenArgs {
    const SIZE:usize = 10+4+100+1;
}

fn process_instruction(
    _program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
       log!("Processing instruction {}", instruction_data);
       log!("crate.io {}", _program_id);
       create_account_with_minimum_balance(account, space, owner, payer, rent_sysvar);
       
       Ok(())
}
