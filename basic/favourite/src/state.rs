use bytemuck::{Pod, Zeroable};
use pinocchio::pubkey::Pubkey;
#[repr(C)]
#[derive(Pod,Zeroable,Clone, Copy)]
pub struct Favorites {
    pub owner:Pubkey,
    pub number: u64,
    pub color: [u8; 8],
    pub hobbies: [[u8; 16]; 4],
}
impl Favorites {
    pub const SIZE:usize= 32+8+8+(16*4); 
}