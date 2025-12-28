use pinocchio::program_error::ProgramError;
#[repr(C)]
pub struct PageVisits {
    pub page_visits: u32,
 pub bump: u8,
}
impl PageVisits {
pub const SPACE:usize= 4+1;
pub const SEED : &'static str = "page_visits";
fn new (data:u8)->Self{
    PageVisits { page_visits:0, bump:data }
}
fn increment(&mut self){
    self.page_visits.checked_add(1).ok_or(ProgramError::InvalidArgument);
}
}