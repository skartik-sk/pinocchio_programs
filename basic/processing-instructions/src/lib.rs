#![no_std]

use pinocchio::{ProgramResult, account_info::AccountInfo, entrypoint, nostd_panic_handler, program_error::ProgramError, pubkey::Pubkey, sysvars::slot_hashes::log};
use pinocchio_log::log;

entrypoint!(process_instruction);
nostd_panic_handler!();


fn process_instruction(
    _program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
       log!("Processing instruction {}", instruction_data);
       log!("crate.io {}", _program_id);
       let instruction_data_object = InstructionData::try_from(instruction_data).unwrap();
       log!("Welcome to the park, {}!", &instruction_data_object.name);
           if instruction_data_object.height > 5 {
               log!("You are tall enough to ride this ride. Congratulations.");
           } else {
               log!("You are NOT tall enough to ride this ride. Sorry mate.");
           };

       Ok(())
}

#[repr(C)]
struct InstructionData {
    pub name: [u8 ;40],//40
    pub height:u32,//4
}

impl TryFrom<&[u8]> for InstructionData {
    type Error = ProgramError;

    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
       if(value.len()<44){
          return Err( ProgramError::InvalidInstructionData);
       }
       let name:[u8;40]= value[0..40].try_into().expect("Error parsing name");
       let height :u32= u32::from_le_bytes(value[40..44].try_into().expect("Error parsing heigh"));
       Ok(Self { name, height })
    }
}


