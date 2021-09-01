use super::super::cpu::*;

#[inline(always)]
pub fn ADD(cpu: &mut Cpu, rd: u8, rs: u8, rt: u8) {
    match cpu.read_ee_register_32(rs).checked_add(cpu.read_ee_register_32(rt)) {
        Some(value) => cpu.write_ee_register_32(rd, value),
        None => cpu.throw_exception(Exception::IntegerOverflow),
    }
}

#[inline(always)]
pub fn ADDI(cpu: &mut Cpu, rt: u8, rs: u8, imm: u16) {
    match cpu.read_ee_register_32(rs).checked_add(imm as u32) {
        Some(value) => cpu.write_ee_register_32(rt, value),
        None => cpu.throw_exception(Exception::IntegerOverflow),
    }
}

#[inline(always)]
pub fn ADDIU(cpu: &mut Cpu, rt: u8, rs: u8, imm: u16) {
    cpu.write_ee_register_32(rt, cpu.read_ee_register_32(rs) + imm as u32);
}

#[inline(always)]
pub fn ADDU(cpu: &mut Cpu, rd: u8, rs: u8, rt: u8) {
    cpu.write_ee_register_32(rd, cpu.read_ee_register_32(rs) + cpu.read_ee_register_32(rt));
}

#[inline(always)]
pub fn AND(cpu: &mut Cpu, rd: u8, rs: u8, rt: u8) {
    cpu.write_ee_register_32(rd, cpu.read_ee_register_32(rs) & cpu.read_ee_register_32(rt));
}

#[inline(always)]
pub fn ANDI(cpu: &mut Cpu, rt: u8, rs: u8, imm: u16) {
    cpu.write_ee_register_32(rt, cpu.read_ee_register_32(rs) & imm as u32);
}

/*
#[inline(always)]
pub fn BEQ(cpu: &mut Cpu, rs: u8, rt: u8, off: u16) {
    let delay_instruction = cpu.fetch_instruction(cpu.pc + 4);
    let condition = cpu.read_ee_register_64(rs) == cpu.read_ee_register_64(rt);
    cpu.execute_delay_slot();
    if condition {
        cpu.pc += (off as u32) << 2;
    }
}

#[inline(always)]
pub fn BEQL(cpu: &mut Cpu, rs: u8, rt: u8, off: u16) {
    let condition = cpu.read_ee_register_64(rs) == cpu.read_ee_register_64(rt);
    if condition {
        cpu.execute_delay_slot();
        cpu.pc += (off as u32) << 2;
    }
}

#[inline(always)]
pub fn BGEZ(cpu: &mut Cpu, rs: u8, off: u16) {
    let condition = cpu.read_ee_register_64(rs) >= 0;
    cpu.execute_delay_slot();
    if condition {
        cpu.pc += (off as u32) << 2;
    }
}

#[inline(always)]
pub fn BGEZAL(cpu: &mut Cpu, rs: u8, off: u16) {
    let condition = cpu.read_ee_register_64(rs) >= 0;
    cpu.execute_delay_slot();
    cpu.write_ee_register_64(RA_REG, (cpu.pc + 8) as u64);
    if condition {
        cpu.pc += (off as u32) << 2;
    }
}

#[inline(always)]
pub fn BGEZALL(cpu: &mut Cpu, rs: u8, off: u16) {
    let condition = cpu.read_ee_register_64(rs) >= 0;
    cpu.write_ee_register_64(RA_REG, (cpu.pc + 8) as u64);
    if condition {
        cpu.execute_delay_slot();
        cpu.pc += (off as u32) << 2;
    }
}

#[inline(always)]
pub fn BGEZL(cpu: &mut Cpu, rs: u8, off: u16) {
    let condition = cpu.read_ee_register_64(rs) >= 0;
    if condition {
        cpu.execute_delay_slot();
        cpu.pc += (off as u32) << 2;
    }
}

#[inline(always)]
pub fn BGTZ(cpu: &mut Cpu, rs: u8, off: u16) {
    let condition = cpu.read_ee_register_64(rs) > 0;
    cpu.execute_delay_slot();
    if condition {
        cpu.pc += (off as u32) << 2;
    }
}

#[inline(always)]
pub fn BGTZL(cpu: &mut Cpu, rs: u8, off: u16) {
    let condition = cpu.read_ee_register_64(rs) > 0;
    if condition {
        cpu.execute_delay_slot();
        cpu.pc += (off as u32) << 2;
    }
}

#[inline(always)]
pub fn BLEZ(cpu: &mut Cpu, rs: u8, off: u16) {
    let condition = cpu.read_ee_register_64(rs) <= 0;
    cpu.execute_delay_slot();
    if condition {
        cpu.pc += (off as u32) << 2;
    }
}

#[inline(always)]
pub fn BLEZL(cpu: &mut Cpu, rs: u8, off: u16) {
    let condition = cpu.read_ee_register_64(rs) <= 0;
    if condition {
        cpu.execute_delay_slot();
        cpu.pc += (off as u32) << 2;
    }
}

#[inline(always)]
pub fn BLTZ(cpu: &mut Cpu, rs: u8, off: u16) {
    let condition = cpu.read_ee_register_64(rs) < 0;
    cpu.execute_delay_slot();
    if condition {
        cpu.pc += (off as u32) << 2;
    }
}

#[inline(always)]
pub fn BLTZAL(cpu: &mut Cpu, rs: u8, off: u16) {
    let condition = cpu.read_ee_register_64(rs) < 0;
    cpu.execute_delay_slot();
    cpu.write_ee_register_64(RA_REG, (cpu.pc + 8) as u64);
    if condition {
        cpu.pc += (off as u32) << 2;
    }
}

#[inline(always)]
pub fn BLTZALL(cpu: &mut Cpu, rs: u8, off: u16) {
    let condition = cpu.read_ee_register_64(rs) < 0;
    cpu.write_ee_register_64(RA_REG, (cpu.pc + 8) as u64);
    if condition {
        cpu.execute_delay_slot();
        cpu.pc += (off as u32) << 2;
    }
}

#[inline(always)]
pub fn BLTZL(cpu: &mut Cpu, rs: u8, off: u16) {
    let condition = cpu.read_ee_register_64(rs) < 0;
    if condition {
        cpu.execute_delay_slot();
        cpu.pc += (off as u32) << 2;
    }
}

#[inline(always)]
pub fn BNE(cpu: &mut Cpu, rs: u8, off: u16) {
    let condition = cpu.read_ee_register_64(rs) != 0;
    cpu.execute_delay_slot();
    if condition {
        cpu.pc += (off as u32) << 2;
    }
}

#[inline(always)]
pub fn BNEL(cpu: &mut Cpu, rs: u8, off: u16) {
    let condition = cpu.read_ee_register_64(rs) != 0;
    if condition {
        cpu.execute_delay_slot();
        cpu.pc += (off as u32) << 2;
    }
}
*/

#[inline(always)]
pub fn BREAK(cpu: &mut Cpu, code: u32) {
    cpu.throw_exception(Exception::Breakpoint(code));
}

#[inline(always)]
pub fn DADD(cpu: &mut Cpu, rd: u8, rs: u8, rt: u8) {
    match cpu.read_ee_register_64(rs).checked_add(cpu.read_ee_register_64(rt)) {
        Some(value) => cpu.write_ee_register_64(rd, value),
        None => cpu.throw_exception(Exception::IntegerOverflow),
    }
}

#[inline(always)]
pub fn DADDI(cpu: &mut Cpu, rt: u8, rs: u8, imm: u16) {
    match cpu.read_ee_register_64(rs).checked_add(imm as u64) {
        Some(value) => cpu.write_ee_register_64(rt, value),
        None => cpu.throw_exception(Exception::IntegerOverflow),
    }
}

#[inline(always)]
pub fn DADDIU(cpu: &mut Cpu, rt: u8, rs: u8, imm: u16) {
    cpu.write_ee_register_64(rt, cpu.read_ee_register_64(rs) + imm as u64);
}

#[inline(always)]
pub fn DADDU(cpu: &mut Cpu, rd: u8, rs: u8, rt: u8) {
    cpu.write_ee_register_64(rd, cpu.read_ee_register_64(rs) + cpu.read_ee_register_64(rt));
}

#[inline(always)]
pub fn DIV(cpu: &mut Cpu, rs: u8, rt: u8) {
    let lhs = cpu.read_ee_register_32(rs) as i32;
    let rhs = cpu.read_ee_register_32(rt) as i32;

    cpu.lo = (lhs / rhs) as u64;
    cpu.hi = (lhs & rhs) as u64;
}

#[inline(always)]
pub fn DIVU(cpu: &mut Cpu, rs: u8, rt: u8) {
    let lhs = cpu.read_ee_register_32(rs);
    let rhs = cpu.read_ee_register_32(rt);

    cpu.lo = (lhs / rhs) as u64;
    cpu.hi = (lhs & rhs) as u64;
}

#[inline(always)]
pub fn DSLL(cpu: &mut Cpu, rd: u8, rt: u8, sa: u8) {
    cpu.write_ee_register_64(rd, cpu.read_ee_register_64(rt) << sa);
}

#[inline(always)]
pub fn DSLL32(cpu: &mut Cpu, rd: u8, rt: u8, sa: u8) {
    cpu.write_ee_register_64(rd, cpu.read_ee_register_64(rt) << (sa + 32));
}

#[inline(always)]
pub fn DSLLV(cpu: &mut Cpu, rd: u8, rt: u8, rs: u8) {
    cpu.write_ee_register_64(rd, cpu.read_ee_register_64(rt) << cpu.read_ee_register_32(rs) & 0x1f);
}

#[inline(always)]
pub fn DSRA(cpu: &mut Cpu, rd: u8, rt: u8, sa: u8) {
    cpu.write_ee_register_64(rd, (cpu.read_ee_register_64(rt) as i64 >> sa) as u64);
}

#[inline(always)]
pub fn DSRA32(cpu: &mut Cpu, rd: u8, rt: u8, sa: u8) {
    cpu.write_ee_register_64(rd, (cpu.read_ee_register_64(rt) as i64 >> (sa + 32)) as u64);
}

#[inline(always)]
pub fn DSRAV(cpu: &mut Cpu, rd: u8, rt: u8, rs: u8) {
    cpu.write_ee_register_64(rd, (cpu.read_ee_register_64(rt) as i64 >> cpu.read_ee_register_32(rs) & 0x1f) as u64);
}

#[cfg(test)]
mod test {
    use super::super::super::cpu::Cpu;
    use super::super::super::cpu::test::create_mock_cpu;
    use super::ADD;

    #[test]
    fn test_add_unsigned() {
        let a = 0;
        let b = 1;

        let c = a + b;

        let mut cpu = create_mock_cpu();
        
        let rd: u8 = 1;
        let rs: u8 = 2;
        let rt: u8 = 3;

        cpu.write_ee_register_32(rs, 1);
        cpu.write_ee_register_32(rt, 2);

        ADD(&mut cpu, rd, rs, rt);

        assert_eq!(cpu.read_ee_register_32(1), 3);
    }
}