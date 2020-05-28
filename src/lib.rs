#![no_std]
 
//Testbench constants are included here as well, though they are not technically part of neutronstar

/*

Summary of interface:

Note: returning u64 values uses the EAX:EDX "mostly but not quite" standard cdcel convention
Order of registers: EAX, ECX, EDX

-- SCCS functions
Interrupt 0x10: push_sccs (buffer, size)
Interrupt 0x11: pop_sccs (buffer, max_size) -> actual_size: u32
Interrupt 0x12: peek_sccs (buffer, max_size, index) -> actual_size: u32
Interrupt 0x13: swap_sccs (index)
Interrupt 0x14: dup_sccs()
Interrupt 0x15: sccs_item_count() -> size
Interrupt 0x16: sccs_memory_size() -> size
Interrupt 0x17: sccs_memory_remaining() -> size
Interrupt 0x18: sccs_item_limit_remaining() -> size

-- CallSystem functions
Interrupt 0x20: system_call(feature, function) -> error:u32

-- Hypervisor functions
Interrupt 0x80: alloc_memory TBD

-- Context functions
Interrupt 0x90: gas_limit() -> u64
Interrupt 0x91: self_address() -- result on stack as NeutronShortAddress
Interrupt 0x92: origin() -- result on stack as NeutronShortAddress
Interrupt 0x93: origin_long() -- result on stack as NeutronLongAddress
Interrupt 0x94: sender() -- result on stack as NeutronShortAddress
Interrupt 0x95: sender_long() -- result on stack as NeutronLongAddress
Interrupt 0x96: value_sent() -> u64
Interrupt 0x97: nest_level() -> u32
Interrupt 0x98: gas_remaining() -> u64
Interrupt 0x99: execution_type() -> u32

-- System interrupts
Interrupt 0xFE: revert_execution(status) -> noreturn
Interrupt 0xFF: exit_execution(status) -> noreturn


*/


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



