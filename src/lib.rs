#![no_std]
 
#[derive(Debug, Default, Copy, Clone, Eq, PartialEq, Hash)]
pub struct NeutronAddress{
    pub version: u32,
    pub data: [u8; 20]
}

impl NeutronAddress{
    pub fn from_data(data: &[u8]) -> NeutronAddress{
        let mut address = NeutronAddress::default();
        for i in 0..4{
            address.version = address.version | ((data[i] as u32) << (i * 8)); 
        }
        address.data.copy_from_slice(&data[4..24]);
        address
    }
}

pub const EXEC_FLAG_CREATE: u32 = 1;



