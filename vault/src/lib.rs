#![no_std]
 
use pinocchio::{ProgramResult, account_info::AccountInfo, entrypoint, nostd_panic_handler, program_error::ProgramError, pubkey::{Pubkey, find_program_address}};

entrypoint!(process_instruction);
nostd_panic_handler!();
 

pub mod withdraw;
pub use withdraw::*;
pub mod deposit;
pub use deposit::*;
// 22222222222222222222222222222222222222222222
//C74nvDyYRN9KbvY44G8ucnzXgjFXWLHyG5y58UTqaNEa
pub const ID: Pubkey = [
    0xa4, 0xfe, 0x54, 0x0c, 0x6e, 0x10, 0x86, 0xf3,
        0x7f, 0x19, 0x3d, 0xbe, 0xbf, 0x6d, 0x8d, 0xa6,
        0xb8, 0x3c, 0x66, 0x11, 0x02, 0xe4, 0x2a, 0xe3,
        0x87, 0xc8, 0x73, 0xde, 0x2c, 0x33, 0xc9, 0xaf,
        
    
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

