#![no_std]
 
use pinocchio::{ProgramResult, account_info::AccountInfo, entrypoint, nostd_panic_handler, program_error::ProgramError, pubkey::{Pubkey, find_program_address}};

entrypoint!(process_instruction);
nostd_panic_handler!();
 

pub mod withdraw;
pub use withdraw::*;
pub mod deposit;
pub use deposit::*;
// 22222222222222222222222222222222222222222222
// [0x0a, 0x5e, 0xfb, 0xee, 0xa4, 0x78, 0x76, 0xfc, 0x67, 0xed, 0x48, 0x62, 0xf0, 0x31, 0x00, 0xcc, 0x1a, 0x04, 0xad, 0xd6, 0xe9, 0x20, 0x31, 0xc6, 0xc8, 0xdb, 0x0b, 0x20, 0x28, 0x1d, 0x49, 0x46, 0xa4, 0xfe, 0x54, 0x0c, 0x6e, 0x10, 0x86, 0xf3, 0x7f, 0x19, 0x3d, 0xbe, 0xbf, 0x6d, 0x8d, 0xa6, 0xb8, 0x3c, 0x66, 0x11, 0x02, 0xe4, 0x2a, 0xe3, 0x87, 0xc8, 0x73, 0xde, 0x2c, 0x33, 0xc9, 0xaf]
pub const ID: Pubkey = [
    0x0f, 0x1e, 0x6b, 0x14, 0x21, 0xc0, 0x4a, 0x07,
    0x04, 0x31, 0x26, 0x5c, 0x19, 0xc5, 0xbb, 0xee,
    0x19, 0x92, 0xba, 0xe8, 0xaf, 0xd1, 0xcd, 0x07,
    0x8e, 0xf8, 0xaf, 0x70, 0x47, 0xdc, 0x11, 0xf7,
];
 
fn process_instruction(
    _program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    match instruction_data.split_first() {
        Some((Deposit::DISCRIMINATOR, data)) => Deposit::try_from((data, accounts))?.process(),
        Some((Withdraw::DISCRIMINATOR, _)) => Withdraw::try_from(accounts)?.process(),
        _ => Err(ProgramError::InvalidInstructionData),
    }
}

    // use shank::ShankInstruction;



    // #[derive(ShankInstruction)]
    //  enum _MyProgramInstruction{
    //     #[account(0, writable, signer, name = "owner_acc", desc = "Fee payer account")]
    //     #[account(1, writable, signer, name = "vault_acc", desc = "vault account")]
    //     #[account(2, name = "system_program", desc = "System program account")]
    //     Deposit,
        
    //     #[account(0, writable, signer, name = "owner_acc", desc = "Fee payer account")]
    //     #[account(1, writable, name = "vault_acc", desc = "vault account")]
    //     Withdraw,
    // }

