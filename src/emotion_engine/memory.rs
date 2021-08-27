const KiB: usize = 1024;
const MiB: usize = 1024 * KiB;

pub type Address = usize;

enum AddressLocation {
    MainEEMemory(Address),
    IORegisters(Address),
    VU0CodeMemory(Address),
    VU0DataMemory(Address),
    VU1CodeMemory(Address),
    VU1DataMemory(Address),
    GSPrivilegedRegisters(Address),
    IOPMemory(Address),
    BIOSMemory(Address),
    Scratchpad(Address),
}

pub struct Memory {
    physical_ee_main_memory: [u8; 32 * MiB],
    physical_io_registers: [u8; 64 * KiB],
    physical_vu0_code_memory: [u8; 4 * KiB],
    physical_vu0_data_memory: [u8; 4 * KiB],
    physical_vu1_code_memory: [u8; 16 * KiB],
    physical_vu1_data_memory: [u8; 16 * KiB],
    physical_gs_privileged_registers: [u8; 8 * KiB],
    physical_iop_memory: [u8; 2 * MiB],
    physical_bios: [u8; 4 * MiB],
    physical_scratchpad: [u8; 16 * KiB],
    physical_gs_vram: [u8; 4 * MiB],
    physical_spu2_work_ram: [u8; 2 * MiB],
    physical_memory_card: [u8; 8 * MiB],
}

fn translate_virt_address(address: Address) -> Option<AddressLocation> {
    match address {
        0x00000000..=0x01FFFFFF => Some(AddressLocation::MainEEMemory(address)),
        0x20000000..=0x21FFFFFF => Some(AddressLocation::MainEEMemory(address - 0x20000000)),
        0x30100000..=0x31FFFFFF => Some(AddressLocation::MainEEMemory(address - 0x30000000)),
        0x10000000..=0x10018FFF => Some(AddressLocation::IORegisters(address - 0x10000000)),
        0x11000000..=0x11000FFF => Some(AddressLocation::VU0CodeMemory(address - 0x11000000)),
        0x11004000..=0x11004FFF => Some(AddressLocation::VU0DataMemory(address - 0x11004000)),
        0x11008000..=0x1100BFFF => Some(AddressLocation::VU1CodeMemory(address - 0x11008000)),
        0x1100C000..=0x1100FFFF => Some(AddressLocation::VU1DataMemory(address - 0x1100C000)),
        0x12000000..=0x12001FFF => Some(AddressLocation::GSPrivilegedRegisters(address - 0x12000000)),
        0x1C000000..=0x1C1FFFFF => Some(AddressLocation::IOPMemory(address - 0x1C000000)),
        0x1FC00000..=0x1FFFFFFF => Some(AddressLocation::BIOSMemory(address - 0x1FC00000)),
        0x9FC00000..=0x9FFFFFFF => Some(AddressLocation::BIOSMemory(address - 0x9FC00000)),
        0xBFC00000..=0xBFFFFFFF => Some(AddressLocation::BIOSMemory(address - 0xBFC00000)),
        0x70000000..=0x70003FFF => Some(AddressLocation::Scratchpad(address - 0x70000000)),
        _ => None,
    }
}

impl Memory {
    pub fn new() -> Memory {
        Memory {
            physical_ee_main_memory: [0; 32 * MiB],
            physical_io_registers: [0; 64 * KiB],
            physical_vu0_code_memory: [0; 4 * KiB],
            physical_vu0_data_memory: [0; 4 * KiB],
            physical_vu1_code_memory: [0; 16 * KiB],
            physical_vu1_data_memory: [0; 16 * KiB],
            physical_gs_privileged_registers: [0; 8 * KiB],
            physical_iop_memory: [0; 2 * MiB],
            physical_bios: [0; 4 * MiB],
            physical_scratchpad: [0; 16 * KiB],
            physical_gs_vram: [0; 4 * MiB],
            physical_spu2_work_ram: [0; 2 * MiB],
            physical_memory_card: [0; 8 * MiB],
        }
    }



    pub fn read_address(&self, virt_address: Address) -> u8 {
        let address_location = translate_virt_address(virt_address).unwrap();

        match address_location {
            AddressLocation::MainEEMemory(address) => self.physical_ee_main_memory[address],
            AddressLocation::IORegisters(address) => self.physical_iop_memory[address],
            AddressLocation::VU0CodeMemory(address) => self.physical_vu0_code_memory[address],
            AddressLocation::VU0DataMemory(address) => self.physical_vu0_data_memory[address],
            AddressLocation::VU1CodeMemory(address) => self.physical_vu1_code_memory[address],
            AddressLocation::VU1DataMemory(address) => self.physical_vu1_data_memory[address],
            AddressLocation::GSPrivilegedRegisters(address) => self.physical_gs_privileged_registers[address],
            AddressLocation::IOPMemory(address) => self.physical_iop_memory[address],
            AddressLocation::BIOSMemory(address) => self.physical_bios[address],
            AddressLocation::Scratchpad(address) => self.physical_scratchpad[address],
        }
    }

    pub fn write_address(&mut self, virt_address: Address, length: usize, values: &[u8]) {
        let address_location = translate_virt_address(virt_address).unwrap();

        let set_memory: &mut [u8] = match address_location {
            AddressLocation::MainEEMemory(address) => &mut self.physical_ee_main_memory[address..address+length],
            AddressLocation::IORegisters(address) => &mut self.physical_iop_memory[address..address+length],
            AddressLocation::VU0CodeMemory(address) => &mut self.physical_vu0_code_memory[address..address+length],
            AddressLocation::VU0DataMemory(address) => &mut self.physical_vu0_data_memory[address..address+length],
            AddressLocation::VU1CodeMemory(address) => &mut self.physical_vu1_code_memory[address..address+length],
            AddressLocation::VU1DataMemory(address) => &mut self.physical_vu1_data_memory[address..address+length],
            AddressLocation::GSPrivilegedRegisters(address) => &mut self.physical_gs_privileged_registers[address..address+length],
            AddressLocation::IOPMemory(address) => &mut self.physical_iop_memory[address..address+length],
            AddressLocation::BIOSMemory(address) => &mut self.physical_bios[address..address+length],
            AddressLocation::Scratchpad(address) => &mut self.physical_scratchpad[address..address+length],
        };

        for (memory, value) in set_memory.iter_mut().zip(values) {
            *memory = *value;
        }
    }
}