use mollusk_svm::result::{Check, ProgramResult};
use mollusk_svm::{program, Mollusk};

use vault::{DepositInstructionData, ID};
use solana_sdk::account::Account;
use solana_sdk::instruction::{AccountMeta, Instruction};
use solana_sdk::native_token::LAMPORTS_PER_SOL;
use solana_sdk::pubkey;
use solana_sdk::pubkey::Pubkey;

pub const PROGRAM: Pubkey = Pubkey::new_from_array(ID);

pub const RENT: Pubkey = pubkey!("SysvarRent111111111111111111111111111111111");

pub const PAYER: Pubkey = pubkey!("6WcEP42ha6bMS5eteTZu84C95kN3kettBL2JwwM4Acu5");

pub fn mollusk() -> Mollusk {
    Mollusk::new(&PROGRAM, "target/deploy/vault")
}
pub trait DataLen {
    const LEN: usize;
}

pub fn to_bytes<T: DataLen>(data: &T) -> &[u8] {
    unsafe { core::slice::from_raw_parts(data as *const T as *const u8, T::LEN) }
}


#[test]
fn test_deposit() {
    let mollusk = mollusk();
    
    
    let (system_prgram, system_account) = program::keyed_account_for_system_program();
    let (vault_pda, bump) =
        Pubkey::find_program_address(&["vault".as_bytes(), &PAYER.to_bytes()], &PROGRAM);
    
    let payer_acc = Account::new(6 * LAMPORTS_PER_SOL, 0, &system_prgram);
    let vault_acc = Account::new(0, 0, &system_prgram);
    
    let ix_account = vec![
        AccountMeta::new(PAYER, true),
        AccountMeta::new(vault_pda, false),
              AccountMeta::new(system_prgram, false),
    ];


    let mut ser_ix_data = vec![0];
    ser_ix_data.extend_from_slice(&DepositInstructionData{  amount: 5 * LAMPORTS_PER_SOL }.amount.to_le_bytes());


    let instruction = Instruction::new_with_bytes(PROGRAM, &ser_ix_data, ix_account);

    // let tx_accounts = &vec![
    //     (PAYER, payer_acc.clone()),
    //     (vault_pda, vault_acc.clone()),
    //           (system_prgram, system_account.clone()),
    // ];
     let tx_accounts = &vec![
        (PAYER, payer_acc.clone()),
        (vault_pda, vault_acc.clone()),

              (system_prgram, system_account.clone()),
    ];

    let init_res =
            mollusk.process_and_validate_instruction(&instruction, tx_accounts, &[
                 Check::account(&vault_pda).lamports(5 * LAMPORTS_PER_SOL).build(),
            ]);

    assert!(init_res.program_result == ProgramResult::Success);
}



#[test]
fn test_withdraw() {
    let mollusk = mollusk();

    let (system_prgram, system_account) = program::keyed_account_for_system_program();

    let (vault_pda,_) =
        Pubkey::find_program_address(&["vault".as_bytes(), &PAYER.to_bytes()], &PROGRAM);

    let payer_acc = Account::new(9, 0, &system_prgram);
    let vault_acc = Account::new(1, 0, &system_prgram);

    let ix_account = vec![
        AccountMeta::new(PAYER, true),
        AccountMeta::new(vault_pda, false),
        AccountMeta::new(system_prgram, false),
    ];

    let mut ix_data = vec![1];

    let instruction = Instruction::new_with_bytes(PROGRAM, &ix_data, ix_account);

    let tx_accounts = &vec![
        (PAYER, payer_acc.clone()),
        (vault_pda, vault_acc.clone()),
        (system_prgram, system_account.clone()),
    ];

    let update_res =
        mollusk.process_and_validate_instruction(&instruction, tx_accounts, &[Check::success()]);

    assert!(update_res.program_result == ProgramResult::Success);
    assert!(update_res.get_account(&PAYER).unwrap().lamports == (9 + 1) );
}


