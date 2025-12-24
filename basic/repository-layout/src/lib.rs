

#![no_std]


#[cfg(not(feature = "no-entrypoint"))]
mod entrypoint;

pub mod error;
pub mod instruction;
pub mod state;

pub static ID:[u8;32]= [
    
    206,146,35,58,28,96,16,232,22,107,54,149,26,6,222,19,136,11,227,242,111,146,171,216,254,133,74,227,168,87,6,52
];


//pinocchio_pubkey::declare_id!("ENrRns55VechXJiq4bMbdx7idzQh7tvaEJoYeWxRNe7Y");