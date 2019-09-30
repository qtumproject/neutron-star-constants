#![no_std]

extern crate struct_deser;
#[macro_use]
extern crate struct_deser_derive;
use num_derive::FromPrimitive;    
//Testbench constants are included here as well, though they are not technically part of neutronstar

pub const NEUTRON_INTERRUPT:u8 = 0x40;
pub const EXIT_INTERRUPT:u8 = 0xF0;
pub const TESTBENCH_INTERRUPT:u8 = 0x50;

/// The system calls available using the NEUTRON_INTERRUPT 
#[derive(FromPrimitive)]
pub enum NeutronSyscalls{
    Invalid = 0,
    /// Pushes a value to the SCCS
    PushSCCS = 1,
    /// Pops a value from the SCCS into a fixed size buffer
    PopSCCS,
    /// Peeks the value from the top of the SCCS into a fixed size buffer
    PeekSCCS,
    /// Pops a value from the SCCS into a dynamically allocated buffer
    PopSCCSAlloc,
    /// Peeks the value from the top of the SCCS into a dynamically allocated buffer
    PeekSCCSAlloc,
    /// Peeks the size of the value from the top of the SCCS
    PeekSCCSSize,
    /// Removes the item from the top of the SCCS without retreiving the value
    DropSCCS,
    /// How many items are in the SCCS
    DepthOfSCCS,
    IsCreate = 0xF000
}
/// The system calls available using the TESTBENCH_INTERRUPT 
#[derive(FromPrimitive)]
pub enum TestbenchSyscalls{
    Invalid = 0,
    /// Logs an error message
    LogError = 1,
    /// Logs an info message
    LogInfo,
    /// Logs a debug message
    LogDebug
}

#[derive(StructDeser, Debug, Default, Copy, Clone, Eq, PartialEq)]
#[repr(C)]
pub struct NeutronShortAddress{
    #[le]
    pub version: u32,
    pub data: [u8; 20]
}

pub struct NeutronFullAddress<'a>{
    pub version: u32,
    pub data: &'a [u8]
}

pub const EXEC_FLAG_CREATE: u32 = 1;

#[derive(StructDeser, Debug, Default, Copy, Clone)]
//#[repr(packed)]
pub struct NeutronExecContext{
    #[le]
    pub flags: u64,
    pub sender: NeutronShortAddress,
    #[le]
    pub gas_limit: u64,
    #[le]
    pub value_sent: u64,
    pub origin: NeutronShortAddress,
    pub self_address: NeutronShortAddress,
    #[le]
    pub nest_level: u32
}






