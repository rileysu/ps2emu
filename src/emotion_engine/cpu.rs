use super::{instructions::{Instruction, NOP}, memory::{Memory, MiB}};

pub const SP_REG: u8 = 29;
pub const RA_REG: u8 = 31;

pub enum Exception {
    IntegerOverflow,
    Breakpoint(u32),
}

pub struct Cpu {
    pub ee_registers: [u128; 32],
    pub pc: u32,
    pub hi: u64,
    pub lo: u64,
    pub hi1: u64,
    pub lo1: u64,
    pub sa: u32,
    pub memory: Memory,
}

impl Cpu {
    pub fn new(bios: &[u8; 4 * MiB], pc: u32) -> Cpu {
        println!("Hello Debug!");

        Cpu {
            ee_registers: [0; 32],
            pc,
            hi: 0,
            lo: 0,
            hi1: 0,
            lo1: 0,
            sa: 0,
            memory: Memory::new(bios),
        }
    }

    #[inline(always)]
    pub fn fetch_instruction(address: u32) ->  Instruction {
        NOP
    }

    #[inline(always)]
    pub fn read_ee_register(&self, register: u8) -> u128 {
        self.ee_registers[register as usize]
    }

    #[inline(always)]
    pub fn write_ee_register(&mut self, register: u8, value: u128) {
        if register != 0 {
            self.ee_registers[register as usize] = value;
        }
    }

    #[inline(always)]
    pub fn read_ee_register_32(&self, register: u8) -> u32 {
        self.ee_registers[register as usize] as u32
    }

    #[inline(always)]
    pub fn write_ee_register_32(&mut self, register: u8, value: u32) {
        if register != 0 {
            self.ee_registers[register as usize] = value as u128;
        }
    }

    #[inline(always)]
    pub fn read_ee_register_64(&self, register: u8) -> u64 {
        self.ee_registers[register as usize] as u64
    }

    #[inline(always)]
    pub fn write_ee_register_64(&mut self, register: u8, value: u64) {
        if register != 0 {
            self.ee_registers[register as usize] = value as u128;
        }
    }

    #[inline(always)]
    pub fn throw_exception(&mut self, exception: Exception) {
        //idk
    }

    #[inline(always)]
    pub fn execute_instruction(&mut self, instruction: Instruction) {
        
    }

    fn execute(&mut self, address: usize) {
        self.write_ee_register(SP_REG, address as u128);

        loop {
            
        }
    }
}

#[cfg(test)]
pub mod test {
    use super::Cpu;
    use super::super::memory::MiB;

    pub fn create_mock_cpu() -> Cpu {
        Cpu::new(&[0u8; 4 * MiB], 0)
    }
}