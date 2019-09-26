#![no_std]

//Testbench constants are included here as well, though they are not technically part of neutronstar

pub const NEUTRON_INTERRUPT:u8 = 0x40;
pub const EXIT_INTERRUPT:u8 = 0xF0;
pub const TESTBENCH_INTERRUPT:u8 = 0x50;

/// The system calls available using the NEUTRON_INTERRUPT 
pub enum NeutronSyscalls{
    /// Pushes a value to the SCCS
    PushSCCS,
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
    DepthOfSCCS
}
/// The system calls available using the TESTBENCH_INTERRUPT 
pub enum TestbenchSyscalls{
    /// Logs an error message
    LogError,
    /// Logs an info message
    LogInfo,
    /// Logs a debug message
    LogDebug
}
