use super::memory::Memory;

pub struct Cpu {
    ee_registers: [u128; 32],
    memory: Memory,
}