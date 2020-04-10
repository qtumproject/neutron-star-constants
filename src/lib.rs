#![no_std]
extern crate num_enum;
use num_enum::TryFromPrimitive;
 
//Testbench constants are included here as well, though they are not technically part of neutronstar

pub const NEUTRON_INTERRUPT:u8 = 0x40;
pub const EXIT_INTERRUPT:u8 = 0xF0;
pub const TESTBENCH_INTERRUPT:u8 = 0x50;

/// The system calls available using the NEUTRON_INTERRUPT 
#[derive(Debug, Eq, PartialEq, TryFromPrimitive)]
#[repr(u32)]
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
    /// How much gas has been used up to this point in execution
    GasUsed,
    IsCreate = 0xF000
}
/// The system calls available using the TESTBENCH_INTERRUPT 
#[derive(Debug, Eq, PartialEq, TryFromPrimitive)]
#[repr(u32)]
pub enum TestbenchSyscalls{
    Invalid = 0,
    /// Logs an error message
    LogError = 1,
    /// Logs an info message
    LogInfo,
    /// Logs a debug message
    LogDebug,
    /// Prints a message to the console. Not guaranteed to exist in all circumstances
    /// and designed primarily for use in testing
    Print,
    /// Disables all diagnostics messages excluding crashes.
    /// Primarily designed for use in testing
    DisableDiagnostics,
}

#[derive(Debug, Default, Copy, Clone, Eq, PartialEq, Hash)]
#[repr(C)]
pub struct NeutronShortAddress{
    pub version: u32,
    pub data: [u8; 20]
}

pub struct NeutronFullAddress<'a>{
    pub version: u32,
    pub data: &'a [u8]
}

pub const EXEC_FLAG_CREATE: u32 = 1;

#[derive(Debug, Default, Copy, Clone)]
//#[repr(packed)]
pub struct NeutronExecContext{
    pub flags: u64,
    pub sender: NeutronShortAddress,
    pub gas_limit: u64,
    pub value_sent: u64,
    pub origin: NeutronShortAddress,
    pub self_address: NeutronShortAddress,
    pub nest_level: u32
}






