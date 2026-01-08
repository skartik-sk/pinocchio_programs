pub struct AddressInfo {
    pub name: [u8; 8],
    pub house_number: u8,
    pub street: [u8; 8],
    pub city: [u8; 8],
}

impl AddressInfo {
    pub const LEN: usize = 25;
}


pub struct EnhancedAddressInfoExtender {
    pub state: [u8; 8],
    pub zip: u32,
}

pub struct EnhancedAddressInfo {
    pub name: [u8; 8],
    pub house_number: u8,
    pub street: [u8; 8],
    pub city: [u8; 8],
    pub state: [u8; 8],
    pub zip: u32,
}

impl EnhancedAddressInfo {
    pub const LEN: usize = 37;
}

pub struct WorkInfo {
    pub name: [u8; 8],
    pub position: [u8; 8],
    pub company: [u8; 8],
    pub years_employed: u8,
}

impl WorkInfo {
    pub const LEN: usize = 25;
}