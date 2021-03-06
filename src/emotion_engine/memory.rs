pub const KiB: usize = 1024;
pub const MiB: usize = 1024 * KiB;

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
    ee_main_memory: Box<[u8]>,
    io_registers: Box<[u8]>,
    vu0_code_memory: Box<[u8]>,
    vu0_data_memory: Box<[u8]>,
    vu1_code_memory: Box<[u8]>,
    vu1_data_memory: Box<[u8]>,
    gs_privileged_registers: Box<[u8]>,
    iop_memory: Box<[u8]>,
    bios: Box<[u8]>,
    scratchpad: Box<[u8]>,
    gs_vram: Box<[u8]>,
    spu2_work_ram: Box<[u8]>,
    memory_card: Box<[u8]>,
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
    pub fn new(bios: &[u8; 4 * MiB]) -> Memory {
        Memory {
            ee_main_memory: vec![0; 32 * MiB].into_boxed_slice(),
            io_registers: vec![0; 64 * KiB].into_boxed_slice(),
            vu0_code_memory: vec![0; 4 * KiB].into_boxed_slice(),
            vu0_data_memory: vec![0; 4 * KiB].into_boxed_slice(),
            vu1_code_memory: vec![0; 16 * KiB].into_boxed_slice(),
            vu1_data_memory: vec![0; 16 * KiB].into_boxed_slice(),
            gs_privileged_registers: vec![0; 8 * KiB].into_boxed_slice(),
            iop_memory: vec![0; 2 * MiB].into_boxed_slice(),
            bios: Box::new(*bios),
            scratchpad: vec![0; 16 * KiB].into_boxed_slice(),
            gs_vram: vec![0; 4 * MiB].into_boxed_slice(),
            spu2_work_ram: vec![0; 2 * MiB].into_boxed_slice(),
            memory_card: vec![0; 8 * MiB].into_boxed_slice(),
        }
    }



    pub fn read_address(&self, virt_address: Address) -> u8 {
        let address_location = translate_virt_address(virt_address).unwrap();

        match address_location {
            AddressLocation::MainEEMemory(address) => self.ee_main_memory[address],
            AddressLocation::IORegisters(address) => self.iop_memory[address],
            AddressLocation::VU0CodeMemory(address) => self.vu0_code_memory[address],
            AddressLocation::VU0DataMemory(address) => self.vu0_data_memory[address],
            AddressLocation::VU1CodeMemory(address) => self.vu1_code_memory[address],
            AddressLocation::VU1DataMemory(address) => self.vu1_data_memory[address],
            AddressLocation::GSPrivilegedRegisters(address) => self.gs_privileged_registers[address],
            AddressLocation::IOPMemory(address) => self.iop_memory[address],
            AddressLocation::BIOSMemory(address) => self.bios[address],
            AddressLocation::Scratchpad(address) => self.scratchpad[address],
        }
    }

    pub fn write_address(&mut self, virt_address: Address, length: usize, values: &[u8]) {
        let address_location = translate_virt_address(virt_address).unwrap();

        let set_memory: &mut [u8] = match address_location {
            AddressLocation::MainEEMemory(address) => &mut self.ee_main_memory[address..address+length],
            AddressLocation::IORegisters(address) => &mut self.iop_memory[address..address+length],
            AddressLocation::VU0CodeMemory(address) => &mut self.vu0_code_memory[address..address+length],
            AddressLocation::VU0DataMemory(address) => &mut self.vu0_data_memory[address..address+length],
            AddressLocation::VU1CodeMemory(address) => &mut self.vu1_code_memory[address..address+length],
            AddressLocation::VU1DataMemory(address) => &mut self.vu1_data_memory[address..address+length],
            AddressLocation::GSPrivilegedRegisters(address) => &mut self.gs_privileged_registers[address..address+length],
            AddressLocation::IOPMemory(address) => &mut self.iop_memory[address..address+length],
            AddressLocation::BIOSMemory(address) => &mut self.bios[address..address+length],
            AddressLocation::Scratchpad(address) => &mut self.scratchpad[address..address+length],
        };

        for (memory, value) in set_memory.iter_mut().zip(values) {
            *memory = *value;
        }
    }
}