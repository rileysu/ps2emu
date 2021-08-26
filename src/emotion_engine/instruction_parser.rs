use super::instruction::{NOP, Instruction};

enum InstructionType {
    Special {
        opcode: u8,
    },
    Regimm {
        opcode: u8,
    },
    Normal {
        opcode: u8,
    },
}

fn is_special(asm_instruction: &u32) -> bool {
    asm_instruction & 0x3f000000 == 0
}

fn is_regim(asm_instruction: &u32) -> bool {
    asm_instruction & 0x3f000000 == 0x01000000
}

fn get_special_opcode(asm_instruction: &u32) -> u8 {
    (asm_instruction & 0x3f) as u8
}

fn get_regimm_opcode(asm_instruction: &u32) -> u8 {
    (asm_instruction >> (32 - 16) & 0x1f) as u8
}

fn get_normal_opcode(asm_instruction: &u32) -> u8 {
    (asm_instruction >> (32 - 6) & 0x3f) as u8
}

fn parse_ADD(asm_instruction: &u32) -> Instruction {
    Instruction::ADD {
        rd: (asm_instruction >> (32 - 21) & 0x1f) as u8, 
        rs: (asm_instruction >> (32 - 11) & 0x1f) as u8, 
        rt: (asm_instruction >> (32 - 16) & 0x1f) as u8,
    }
}

fn parse_ADDI(asm_instruction: &u32) -> Instruction {
    Instruction::ADDI {
        rs: (asm_instruction >> (32 - 11) & 0x1f) as u8, 
        rt: (asm_instruction >> (32 - 16) & 0x1f) as u8,
        imm: (asm_instruction & 0xffff) as i16
    }
}

fn parse_ADDUI(asm_instruction: &u32) -> Instruction {
    Instruction::ADDUI {
        rs: (asm_instruction >> (32 - 11) & 0x1f) as u8, 
        rt: (asm_instruction >> (32 - 16) & 0x1f) as u8,
        imm: (asm_instruction & 0xffff) as i16
    }
}

fn parse_ADDU(asm_instruction: &u32) -> Instruction {
    Instruction::ADD {
        rd: (asm_instruction >> (32 - 21) & 0x1f) as u8, 
        rs: (asm_instruction >> (32 - 11) & 0x1f) as u8, 
        rt: (asm_instruction >> (32 - 16) & 0x1f) as u8,
    }
}

fn parse_AND(asm_instruction: &u32) -> Instruction {
    Instruction::AND {
        rd: (asm_instruction >> (32 - 21) & 0x1f) as u8, 
        rs: (asm_instruction >> (32 - 11) & 0x1f) as u8, 
        rt: (asm_instruction >> (32 - 16) & 0x1f) as u8,  
    }
}

fn parse_ANDI(asm_instruction: &u32) -> Instruction {
    Instruction::ANDI {
        rs: (asm_instruction >> (32 - 11) & 0x1f) as u8, 
        rt: (asm_instruction >> (32 - 16) & 0x1f) as u8,
        imm: (asm_instruction & 0xffff) as u16,
    }
}

fn parse_BEQ(asm_instruction: &u32) -> Instruction {
    Instruction::BEQ {
        rs: (asm_instruction >> (32 - 11) & 0x1f) as u8,
        rt: (asm_instruction >> (32 - 16) & 0x1f) as u8,
        off: (asm_instruction & 0xffff) as u16,
    }
}

fn parse_BEQL(asm_instruction: &u32) -> Instruction {
    Instruction::BEQL {
        rt: (asm_instruction >> (32 - 11) & 0x1f) as u8,
        rs: (asm_instruction >> (32 - 16) & 0x1f) as u8,
        off: (asm_instruction & 0xffff) as u16,
    }
}

fn parse_BGEZ(asm_instruction: &u32) -> Instruction {
    Instruction::BGEZ {
        rs: (asm_instruction >> (32 - 11) & 0x1f) as u8,
        off: (asm_instruction & 0xffff) as u16,
    }
}

fn parse_BGEZAL(asm_instruction: &u32) -> Instruction {
    Instruction::BGEZAL {
        rs: (asm_instruction >> (32 - 11) & 0x1f) as u8,
        off: (asm_instruction & 0xffff) as u16,
    }
}

fn parse_BGEZALL(asm_instruction: &u32) -> Instruction {
    Instruction::BGEZAL {
        rs: (asm_instruction >> (32 - 11) & 0x1f) as u8,
        off: (asm_instruction & 0xffff) as u16,
    }
}

fn parse_BGEZL(asm_instruction: &u32) -> Instruction {
    Instruction::BGEZAL {
        rs: (asm_instruction >> (32 - 11) & 0x1f) as u8,
        off: (asm_instruction & 0xffff) as u16,
    }
}

fn parse_BGTZ(asm_instruction: &u32) -> Instruction {
    Instruction::BGTZ {
        rs: (asm_instruction >> (32 - 11) & 0x1f) as u8,
        off: (asm_instruction & 0xffff) as u16,
    }
}

fn parse_BGTZL(asm_instruction: &u32) -> Instruction {
    Instruction::BGTZL {
        rs: (asm_instruction >> (32 - 11) & 0x1f) as u8,
        off: (asm_instruction & 0xffff) as u16,
    }
}

fn parse_BLEZ(asm_instruction: &u32) -> Instruction {
    Instruction::BLEZ {
        rs: (asm_instruction >> (32 - 11) & 0x1f) as u8,
        off: (asm_instruction & 0xffff) as u16,
    }
}

fn parse_BLEZL(asm_instruction: &u32) -> Instruction {
    Instruction::BLEZL {
        rs: (asm_instruction >> (32 - 11) & 0x1f) as u8,
        off: (asm_instruction & 0xffff) as u16,
    }
}

fn parse_BLTZ(asm_instruction: &u32) -> Instruction {
    Instruction::BLTZ {
        rs: (asm_instruction >> (32 - 11) & 0x1f) as u8,
        off: (asm_instruction & 0xffff) as u16,
    }
}

fn parse_BLTZAL(asm_instruction: &u32) -> Instruction {
    Instruction::BLTZ {
        rs: (asm_instruction >> (32 - 11) & 0x1f) as u8,
        off: (asm_instruction & 0xffff) as u16,
    }
}

fn parse_BLTZALL(asm_instruction: &u32) -> Instruction {
    Instruction::BLTZ {
        rs: (asm_instruction >> (32 - 11) & 0x1f) as u8,
        off: (asm_instruction & 0xffff) as u16,
    }
}

fn parse_BLTZL(asm_instruction: &u32) -> Instruction {
    Instruction::BLTZL {
        rs: (asm_instruction >> (32 - 11) & 0x1f) as u8,
        off: (asm_instruction & 0xffff) as u16,
    }
}

fn parse_BNE(asm_instruction: &u32) -> Instruction {
    Instruction::BNE {
        rs: (asm_instruction >> (32 - 11) & 0x1f) as u8,
        rt: (asm_instruction >> (32 - 16) & 0x1f) as u8,
        off: (asm_instruction & 0xffff) as u16,
    }
}

fn parse_BNEL(asm_instruction: &u32) -> Instruction {
    Instruction::BNEL {
        rs: (asm_instruction >> (32 - 11) & 0x1f) as u8,
        rt: (asm_instruction >> (32 - 16) & 0x1f) as u8,
        off: (asm_instruction & 0xffff) as u16,
    }
}

fn parse_BREAK(asm_instruction: &u32) -> Instruction {
    Instruction::BREAK {
        code: (asm_instruction >> 6 & 0x000fffff) as u32,
    }
}

fn parse_DADD(asm_instruction: &u32) -> Instruction {
    Instruction::DADD {
        rd: (asm_instruction >> (32 - 21) & 0x1f) as u8, 
        rs: (asm_instruction >> (32 - 11) & 0x1f) as u8, 
        rt: (asm_instruction >> (32 - 16) & 0x1f) as u8,
    }
}

fn parse_DADDI(asm_instruction: &u32) -> Instruction {
    Instruction::DADDI {
        rs: (asm_instruction >> (32 - 11) & 0x1f) as u8, 
        rt: (asm_instruction >> (32 - 16) & 0x1f) as u8,
        imm: (asm_instruction & 0xffff) as i16
    }
}

fn parse_DADDIU(asm_instruction: &u32) -> Instruction {
    Instruction::DADDIU {
        rs: (asm_instruction >> (32 - 11) & 0x1f) as u8, 
        rt: (asm_instruction >> (32 - 16) & 0x1f) as u8,
        imm: (asm_instruction & 0xffff) as i16
    }
}

fn parse_DADDU(asm_instruction: &u32) -> Instruction {
    Instruction::DADD {
        rd: (asm_instruction >> (32 - 21) & 0x1f) as u8, 
        rs: (asm_instruction >> (32 - 11) & 0x1f) as u8, 
        rt: (asm_instruction >> (32 - 16) & 0x1f) as u8,
    }
}

fn parse_DIV(asm_instruction: &u32) -> Instruction {
    Instruction::DIV {
        rs: (asm_instruction >> (32 - 11) & 0x1f) as u8, 
        rt: (asm_instruction >> (32 - 16) & 0x1f) as u8,
    }
}

fn parse_DIVU(asm_instruction: &u32) -> Instruction {
    Instruction::DIVU {
        rs: (asm_instruction >> (32 - 11) & 0x1f) as u8, 
        rt: (asm_instruction >> (32 - 16) & 0x1f) as u8,
    }
}

fn parse_DSLL(asm_instruction: &u32) -> Instruction {
    Instruction::DSLL {
        rt: (asm_instruction >> (32 - 16) & 0x1f) as u8,
        rd: (asm_instruction >> (32 - 21) & 0x1f) as u8, 
        sa: (asm_instruction >> (32 - 26) & 0x1f) as u8, 
    }
}

fn parse_DSLL32(asm_instruction: &u32) -> Instruction {
    Instruction::DSLL32 {
        rt: (asm_instruction >> (32 - 16) & 0x1f) as u8,
        rd: (asm_instruction >> (32 - 21) & 0x1f) as u8, 
        sa: (asm_instruction >> (32 - 26) & 0x1f) as u8, 
    }
}

fn parse_DSLLV(asm_instruction: &u32) -> Instruction {
    Instruction::DSLLV {
        rd: (asm_instruction >> (32 - 21) & 0x1f) as u8, 
        rt: (asm_instruction >> (32 - 16) & 0x1f) as u8,
        rs: (asm_instruction >> (32 - 11) & 0x1f) as u8,
    }
}

fn parse_DSRA(asm_instruction: &u32) -> Instruction {
    Instruction::DSRA {
        rt: (asm_instruction >> (32 - 16) & 0x1f) as u8,
        rd: (asm_instruction >> (32 - 21) & 0x1f) as u8, 
        sa: (asm_instruction >> (32 - 26) & 0x1f) as u8,
    }
}

fn parse_DSRA32(asm_instruction: &u32) -> Instruction {
    Instruction::DSRA32 {
        rt: (asm_instruction >> (32 - 16) & 0x1f) as u8,
        rd: (asm_instruction >> (32 - 21) & 0x1f) as u8, 
        sa: (asm_instruction >> (32 - 26) & 0x1f) as u8,
    }
}

fn parse_DSRAV(asm_instruction: &u32) -> Instruction {
    Instruction::DSLLV {
        rd: (asm_instruction >> (32 - 21) & 0x1f) as u8, 
        rt: (asm_instruction >> (32 - 16) & 0x1f) as u8,
        rs: (asm_instruction >> (32 - 11) & 0x1f) as u8,
    }
}

fn parse_DSRL(asm_instruction: &u32) -> Instruction {
    Instruction::DSRL {
        rt: (asm_instruction >> (32 - 16) & 0x1f) as u8,
        rd: (asm_instruction >> (32 - 21) & 0x1f) as u8, 
        sa: (asm_instruction >> (32 - 26) & 0x1f) as u8,
    }
}

fn parse_DSRL32(asm_instruction: &u32) -> Instruction {
    Instruction::DSRL32 {
        rt: (asm_instruction >> (32 - 16) & 0x1f) as u8,
        rd: (asm_instruction >> (32 - 21) & 0x1f) as u8, 
        sa: (asm_instruction >> (32 - 26) & 0x1f) as u8,
    }
}

fn parse_DSRLV(asm_instruction: &u32) -> Instruction {
    Instruction::DSLLV {
        rd: (asm_instruction >> (32 - 21) & 0x1f) as u8, 
        rt: (asm_instruction >> (32 - 16) & 0x1f) as u8,
        rs: (asm_instruction >> (32 - 11) & 0x1f) as u8,
    }
}

fn parse_DSUB(asm_instruction: &u32) -> Instruction {
    Instruction::DSUB {
        rd: (asm_instruction >> (32 - 21) & 0x1f) as u8, 
        rs: (asm_instruction >> (32 - 11) & 0x1f) as u8, 
        rt: (asm_instruction >> (32 - 16) & 0x1f) as u8,
    }
}

fn parse_DSUBU(asm_instruction: &u32) -> Instruction {
    Instruction::DSUBU {
        rd: (asm_instruction >> (32 - 21) & 0x1f) as u8, 
        rs: (asm_instruction >> (32 - 11) & 0x1f) as u8, 
        rt: (asm_instruction >> (32 - 16) & 0x1f) as u8,
    }
}

fn parse_J(asm_instruction: &u32) -> Instruction {
    Instruction::J {
        off: (asm_instruction & 0x03ffffff) as u32,
    }
}

fn parse_JAL(asm_instruction: &u32) -> Instruction {
    Instruction::JAL {
        off: (asm_instruction & 0x03ffffff) as u32,
    }
}

fn parse_JALR(asm_instruction: &u32) -> Instruction {
    Instruction::JALR {
        rd: (asm_instruction >> (32 - 21) & 0x1f) as u8, 
        rs: (asm_instruction >> (32 - 11) & 0x1f) as u8,
    }
}

fn parse_JR(asm_instruction: &u32) -> Instruction {
    Instruction::JR {
        rs: (asm_instruction >> (32 - 11) & 0x1f) as u8,
    }
}

fn parse_LB(asm_instruction: &u32) -> Instruction {
    Instruction::LB {
        rt: (asm_instruction >> (32 - 16) & 0x1f) as u8,
        off: (asm_instruction & 0xffff) as u16,
        base: (asm_instruction >> (32 - 11) & 0x1f) as u8,
    }
}

fn parse_LBU(asm_instruction: &u32) -> Instruction {
    Instruction::LBU {
        rt: (asm_instruction >> (32 - 16) & 0x1f) as u8,
        off: (asm_instruction & 0xffff) as u16,
        base: (asm_instruction >> (32 - 11) & 0x1f) as u8,
    }
}

fn parse_LD(asm_instruction: &u32) -> Instruction {
    Instruction::LD {
        rt: (asm_instruction >> (32 - 16) & 0x1f) as u8,
        off: (asm_instruction & 0xffff) as u16,
        base: (asm_instruction >> (32 - 11) & 0x1f) as u8,
    }
}

fn parse_LDL(asm_instruction: &u32) -> Instruction {
    Instruction::LDL {
        rt: (asm_instruction >> (32 - 16) & 0x1f) as u8,
        off: (asm_instruction & 0xffff) as u16,
        base: (asm_instruction >> (32 - 11) & 0x1f) as u8,
    }
}

fn parse_LDR(asm_instruction: &u32) -> Instruction {
    Instruction::LDL {
        rt: (asm_instruction >> (32 - 16) & 0x1f) as u8,
        off: (asm_instruction & 0xffff) as u16,
        base: (asm_instruction >> (32 - 11) & 0x1f) as u8,
    }
}

fn parse_LH(asm_instruction: &u32) -> Instruction {
    Instruction::LH {
        rt: (asm_instruction >> (32 - 16) & 0x1f) as u8,
        off: (asm_instruction & 0xffff) as u16,
        base: (asm_instruction >> (32 - 11) & 0x1f) as u8,
    }
}

fn parse_LHU(asm_instruction: &u32) -> Instruction {
    Instruction::LHU {
        rt: (asm_instruction >> (32 - 16) & 0x1f) as u8,
        off: (asm_instruction & 0xffff) as u16,
        base: (asm_instruction >> (32 - 11) & 0x1f) as u8,
    }
}

fn parse_LUI(asm_instruction: &u32) -> Instruction {
    Instruction::LUI {
        rt: (asm_instruction >> (32 - 16) & 0x1f) as u8,
        imm: (asm_instruction & 0xffff) as u16,
    }
}

fn parse_LW(asm_instruction: &u32) -> Instruction {
    Instruction::LW {
        rt: (asm_instruction >> (32 - 16) & 0x1f) as u8,
        off: (asm_instruction & 0xffff) as u16,
        base: (asm_instruction >> (32 - 11) & 0x1f) as u8,
    }
}

fn parse_LWL(asm_instruction: &u32) -> Instruction {
    Instruction::LWL {
        rt: (asm_instruction >> (32 - 16) & 0x1f) as u8,
        off: (asm_instruction & 0xffff) as u16,
        base: (asm_instruction >> (32 - 11) & 0x1f) as u8,
    }
}

fn parse_LWR(asm_instruction: &u32) -> Instruction {
    Instruction::LWR {
        rt: (asm_instruction >> (32 - 16) & 0x1f) as u8,
        off: (asm_instruction & 0xffff) as u16,
        base: (asm_instruction >> (32 - 11) & 0x1f) as u8,
    }
}

fn parse_LWU(asm_instruction: &u32) -> Instruction {
    Instruction::LWU {
        rt: (asm_instruction >> (32 - 16) & 0x1f) as u8,
        off: (asm_instruction & 0xffff) as u16,
        base: (asm_instruction >> (32 - 11) & 0x1f) as u8,
    }
}

fn parse_MFHI(asm_instruction: &u32) -> Instruction {
    Instruction::MFHI {
        rd: (asm_instruction >> (32 - 21) & 0x1f) as u8, 
    }
}

fn parse_MFLO(asm_instruction: &u32) -> Instruction {
    Instruction::MFLO {
        rd: (asm_instruction >> (32 - 21) & 0x1f) as u8, 
    }
}

fn parse_MOVN(asm_instruction: &u32) -> Instruction {
    Instruction::MOVN {
        rd: (asm_instruction >> (32 - 21) & 0x1f) as u8, 
        rs: (asm_instruction >> (32 - 11) & 0x1f) as u8, 
        rt: (asm_instruction >> (32 - 16) & 0x1f) as u8,
    }
}

fn parse_MOVZ(asm_instruction: &u32) -> Instruction {
    Instruction::MOVZ {
        rd: (asm_instruction >> (32 - 21) & 0x1f) as u8, 
        rs: (asm_instruction >> (32 - 11) & 0x1f) as u8, 
        rt: (asm_instruction >> (32 - 16) & 0x1f) as u8,
    }
}

fn parse_MTHI(asm_instruction: &u32) -> Instruction {
    Instruction::MTHI {
        rs: (asm_instruction >> (32 - 11) & 0x1f) as u8, 
    }
}

fn parse_MTLO(asm_instruction: &u32) -> Instruction {
    Instruction::MTLO {
        rs: (asm_instruction >> (32 - 11) & 0x1f) as u8, 
    }
}

fn parse_MULT(asm_instruction: &u32) -> Instruction {
    Instruction::MULT {
        rs: (asm_instruction >> (32 - 11) & 0x1f) as u8, 
        rt: (asm_instruction >> (32 - 16) & 0x1f) as u8,
    }
}

fn parse_MULTU(asm_instruction: &u32) -> Instruction {
    Instruction::MULT {
        rs: (asm_instruction >> (32 - 11) & 0x1f) as u8, 
        rt: (asm_instruction >> (32 - 16) & 0x1f) as u8,
    }
}

fn parse_NOR(asm_instruction: &u32) -> Instruction {
    Instruction::NOR {
        rd: (asm_instruction >> (32 - 21) & 0x1f) as u8, 
        rs: (asm_instruction >> (32 - 11) & 0x1f) as u8, 
        rt: (asm_instruction >> (32 - 16) & 0x1f) as u8,
    }
}

fn parse_OR(asm_instruction: &u32) -> Instruction {
    Instruction::OR {
        rd: (asm_instruction >> (32 - 21) & 0x1f) as u8, 
        rs: (asm_instruction >> (32 - 11) & 0x1f) as u8, 
        rt: (asm_instruction >> (32 - 16) & 0x1f) as u8,
    }
}

fn parse_ORI(asm_instruction: &u32) -> Instruction {
    Instruction::ORI {
        rs: (asm_instruction >> (32 - 11) & 0x1f) as u8, 
        rt: (asm_instruction >> (32 - 16) & 0x1f) as u8,
        imm: (asm_instruction & 0xffff) as u16,
    }
}

fn parse_PREF(asm_instruction: &u32) -> Instruction {
    Instruction::PREF {
        hint: (asm_instruction >> (32 - 16) & 0x1f) as u8,
        off: (asm_instruction & 0xffff) as u16,
        base: (asm_instruction >> (32 - 11) & 0x1f) as u8,
    }
}

fn parse_SB(asm_instruction: &u32) -> Instruction {
    Instruction::SB {
        rt: (asm_instruction >> (32 - 16) & 0x1f) as u8,
        off: (asm_instruction & 0xffff) as u16,
        base: (asm_instruction >> (32 - 11) & 0x1f) as u8,
    }
}

fn parse_SD(asm_instruction: &u32) -> Instruction {
    Instruction::SD {
        rt: (asm_instruction >> (32 - 16) & 0x1f) as u8,
        off: (asm_instruction & 0xffff) as u16,
        base: (asm_instruction >> (32 - 11) & 0x1f) as u8,
    }
}

fn parse_SDL(asm_instruction: &u32) -> Instruction {
    Instruction::SDL {
        rt: (asm_instruction >> (32 - 16) & 0x1f) as u8,
        off: (asm_instruction & 0xffff) as u16,
        base: (asm_instruction >> (32 - 11) & 0x1f) as u8,
    }
}

fn parse_SDR(asm_instruction: &u32) -> Instruction {
    Instruction::SDR {
        rt: (asm_instruction >> (32 - 16) & 0x1f) as u8,
        off: (asm_instruction & 0xffff) as u16,
        base: (asm_instruction >> (32 - 11) & 0x1f) as u8,
    }
}

fn parse_SH(asm_instruction: &u32) -> Instruction {
    Instruction::SH {
        rt: (asm_instruction >> (32 - 16) & 0x1f) as u8,
        off: (asm_instruction & 0xffff) as u16,
        base: (asm_instruction >> (32 - 11) & 0x1f) as u8,
    }
}

fn parse_SLL(asm_instruction: &u32) -> Instruction {
    Instruction::SLL {
        rt: (asm_instruction >> (32 - 16) & 0x1f) as u8,
        rd: (asm_instruction >> (32 - 21) & 0x1f) as u8, 
        sa: (asm_instruction >> (32 - 26) & 0x1f) as u8,
    }
}

fn parse_SLLV(asm_instruction: &u32) -> Instruction {
    Instruction::SLLV {
        rd: (asm_instruction >> (32 - 21) & 0x1f) as u8, 
        rt: (asm_instruction >> (32 - 16) & 0x1f) as u8,
        rs: (asm_instruction >> (32 - 11) & 0x1f) as u8, 
    }
}

fn parse_SLT(asm_instruction: &u32) -> Instruction {
    Instruction::SLLV {
        rd: (asm_instruction >> (32 - 21) & 0x1f) as u8, 
        rt: (asm_instruction >> (32 - 16) & 0x1f) as u8,
        rs: (asm_instruction >> (32 - 11) & 0x1f) as u8, 
    }
}

fn parse_SLTI(asm_instruction: &u32) -> Instruction {
    Instruction::SLTI {
        rs: (asm_instruction >> (32 - 11) & 0x1f) as u8, 
        rt: (asm_instruction >> (32 - 16) & 0x1f) as u8,
        imm: (asm_instruction & 0xffff) as i16,
    }
}

fn parse_SLTIU(asm_instruction: &u32) -> Instruction {
    Instruction::SLTIU {
        rs: (asm_instruction >> (32 - 11) & 0x1f) as u8, 
        rt: (asm_instruction >> (32 - 16) & 0x1f) as u8,
        imm: (asm_instruction & 0xffff) as u16,
    }
}

fn parse_SLTU(asm_instruction: &u32) -> Instruction {
    Instruction::SLTU {
        rd: (asm_instruction >> (32 - 21) & 0x1f) as u8, 
        rt: (asm_instruction >> (32 - 16) & 0x1f) as u8,
        rs: (asm_instruction >> (32 - 11) & 0x1f) as u8, 
    }
}

fn parse_SRA(asm_instruction: &u32) -> Instruction {
    Instruction::SRA {
        rt: (asm_instruction >> (32 - 16) & 0x1f) as u8,
        rd: (asm_instruction >> (32 - 21) & 0x1f) as u8, 
        sa: (asm_instruction >> (32 - 26) & 0x1f) as u8,
    }
}

fn parse_SRAV(asm_instruction: &u32) -> Instruction {
    Instruction::SRAV {
        rd: (asm_instruction >> (32 - 21) & 0x1f) as u8, 
        rt: (asm_instruction >> (32 - 16) & 0x1f) as u8,
        rs: (asm_instruction >> (32 - 11) & 0x1f) as u8, 
    }
}

fn parse_SRL(asm_instruction: &u32) -> Instruction {
    Instruction::SRL {
        rt: (asm_instruction >> (32 - 16) & 0x1f) as u8,
        rd: (asm_instruction >> (32 - 21) & 0x1f) as u8, 
        sa: (asm_instruction >> (32 - 26) & 0x1f) as u8,
    }
}

fn parse_SRLV(asm_instruction: &u32) -> Instruction {
    Instruction::SRLV {
        rd: (asm_instruction >> (32 - 21) & 0x1f) as u8, 
        rt: (asm_instruction >> (32 - 16) & 0x1f) as u8,
        rs: (asm_instruction >> (32 - 11) & 0x1f) as u8, 
    }
}

fn parse_SUB(asm_instruction: &u32) -> Instruction {
    Instruction::SUB {
        rd: (asm_instruction >> (32 - 21) & 0x1f) as u8, 
        rt: (asm_instruction >> (32 - 16) & 0x1f) as u8,
        rs: (asm_instruction >> (32 - 11) & 0x1f) as u8, 
    }
}

fn parse_SUBU(asm_instruction: &u32) -> Instruction {
    Instruction::SUBU {
        rd: (asm_instruction >> (32 - 21) & 0x1f) as u8, 
        rt: (asm_instruction >> (32 - 16) & 0x1f) as u8,
        rs: (asm_instruction >> (32 - 11) & 0x1f) as u8, 
    }
}

fn parse_SW(asm_instruction: &u32) -> Instruction {
    Instruction::SW {
        rt: (asm_instruction >> (32 - 16) & 0x1f) as u8,
        off: (asm_instruction & 0xffff) as u16,
        base: (asm_instruction >> (32 - 11) & 0x1f) as u8,
    }
}

fn parse_SWL(asm_instruction: &u32) -> Instruction {
    Instruction::SWL {
        rt: (asm_instruction >> (32 - 16) & 0x1f) as u8,
        off: (asm_instruction & 0xffff) as u16,
        base: (asm_instruction >> (32 - 11) & 0x1f) as u8,
    }
}

fn parse_SWR(asm_instruction: &u32) -> Instruction {
    Instruction::SWR {
        rt: (asm_instruction >> (32 - 16) & 0x1f) as u8,
        off: (asm_instruction & 0xffff) as u16,
        base: (asm_instruction >> (32 - 11) & 0x1f) as u8,
    }
}

fn parse_SYNC(asm_instruction: &u32) -> Instruction {
    Instruction::SYNC {
        stype: (asm_instruction >> (32 - 26) & 0x1f) as u8,
    }
}

fn parse_SYSCALL(asm_instruction: &u32) -> Instruction {
    Instruction::SYSCALL {
        code: (asm_instruction >> 6 & 0x000fffff) as u32,
    }
}

fn parse_TEQ(asm_instruction: &u32) -> Instruction {
    Instruction::TEQ {
        rs: (asm_instruction >> (32 - 11) & 0x1f) as u8, 
        rt: (asm_instruction >> (32 - 16) & 0x1f) as u8,
        code: (asm_instruction >> 6 & 0x03ff) as u16,
    }
}

fn parse_TEQI(asm_instruction: &u32) -> Instruction {
    Instruction::TEQI {
        rs: (asm_instruction >> (32 - 11) & 0x1f) as u8, 
        imm: (asm_instruction & 0xffff) as i16
    }
}

fn parse_TGE(asm_instruction: &u32) -> Instruction {
    Instruction::TGE {
        rs: (asm_instruction >> (32 - 11) & 0x1f) as u8, 
        rt: (asm_instruction >> (32 - 16) & 0x1f) as u8,
        code: (asm_instruction >> 6 & 0x03ff) as u16,
    }
}

fn parse_TGEI(asm_instruction: &u32) -> Instruction {
    Instruction::TGEI {
        rs: (asm_instruction >> (32 - 11) & 0x1f) as u8, 
        imm: (asm_instruction & 0xffff) as i16
    }
}

fn parse_TGEIU(asm_instruction: &u32) -> Instruction {
    Instruction::TGEIU {
        rs: (asm_instruction >> (32 - 11) & 0x1f) as u8, 
        imm: (asm_instruction & 0xffff) as i16
    }
}

fn parse_TGEU(asm_instruction: &u32) -> Instruction {
    Instruction::TGEU {
        rs: (asm_instruction >> (32 - 11) & 0x1f) as u8, 
        rt: (asm_instruction >> (32 - 16) & 0x1f) as u8,
        code: (asm_instruction >> 6 & 0x03ff) as u16,
    }
}

fn parse_TLT(asm_instruction: &u32) -> Instruction {
    Instruction::TLT {
        rs: (asm_instruction >> (32 - 11) & 0x1f) as u8, 
        rt: (asm_instruction >> (32 - 16) & 0x1f) as u8,
        code: (asm_instruction >> 6 & 0x03ff) as u16,
    }
}

fn parse_TLTI(asm_instruction: &u32) -> Instruction {
    Instruction::TLTI {
        rs: (asm_instruction >> (32 - 11) & 0x1f) as u8, 
        imm: (asm_instruction & 0xffff) as i16
    }
}

fn parse_TLTIU(asm_instruction: &u32) -> Instruction {
    Instruction::TLTIU {
        rs: (asm_instruction >> (32 - 11) & 0x1f) as u8, 
        imm: (asm_instruction & 0xffff) as i16
    }
}

fn parse_TLTU(asm_instruction: &u32) -> Instruction {
    Instruction::TLTU {
        rs: (asm_instruction >> (32 - 11) & 0x1f) as u8, 
        rt: (asm_instruction >> (32 - 16) & 0x1f) as u8,
        code: (asm_instruction >> 6 & 0x03ff) as u16,
    }
}

fn parse_TNE(asm_instruction: &u32) -> Instruction {
    Instruction::TNE {
        rs: (asm_instruction >> (32 - 11) & 0x1f) as u8, 
        rt: (asm_instruction >> (32 - 16) & 0x1f) as u8,
        code: (asm_instruction >> 6 & 0x03ff) as u16,
    }
}

fn parse_TNEI(asm_instruction: &u32) -> Instruction {
    Instruction::TNEI {
        rs: (asm_instruction >> (32 - 11) & 0x1f) as u8, 
        imm: (asm_instruction & 0xffff) as i16
    }
}

fn parse_XOR(asm_instruction: &u32) -> Instruction {
    Instruction::XOR {
        rd: (asm_instruction >> (32 - 21) & 0x1f) as u8, 
        rs: (asm_instruction >> (32 - 11) & 0x1f) as u8, 
        rt: (asm_instruction >> (32 - 16) & 0x1f) as u8,  
    }
}

fn parse_XORI(asm_instruction: &u32) -> Instruction {
    Instruction::XORI {
        rs: (asm_instruction >> (32 - 11) & 0x1f) as u8, 
        rt: (asm_instruction >> (32 - 16) & 0x1f) as u8,
        imm: (asm_instruction & 0xffff) as u16,
    }
}
 
fn get_instruction_type(asm_instruction: &u32) -> InstructionType {
    if is_special(asm_instruction) {
        InstructionType::Special {
            opcode: get_special_opcode(asm_instruction),
        }
    } else if is_regim(asm_instruction) {
        InstructionType::Regimm {
            opcode: get_regimm_opcode(asm_instruction),
        }
    } else {
        InstructionType::Normal {
            opcode: get_normal_opcode(asm_instruction),
        }
    }
}

pub fn parse_instructions(asm: &[u32]) -> Vec<Instruction> {
    let mut out_instructions: Vec<Instruction> = vec![];

    for asm_instruction in asm {
        out_instructions.push(match get_instruction_type(asm_instruction) {
            InstructionType::Special { opcode } => {
                match opcode {
                    0 => parse_SLL(asm_instruction),
                    2 => parse_SRL(asm_instruction),
                    3 => parse_SRA(asm_instruction),
                    4 => parse_SLLV(asm_instruction),
                    6 => parse_SRLV(asm_instruction),
                    7 => parse_SRAV(asm_instruction),
                    8 => parse_JR(asm_instruction),
                    9 => parse_JALR(asm_instruction),
                    10 => parse_MOVZ(asm_instruction),
                    11 => parse_MOVN(asm_instruction),
                    12 => parse_SYSCALL(asm_instruction),
                    15 => parse_SYNC(asm_instruction),
                    16 => parse_MFHI(asm_instruction),
                    17 => parse_MTHI(asm_instruction),
                    18 => parse_MFLO(asm_instruction),
                    19 => parse_MTLO(asm_instruction),
                    20 => parse_DSLLV(asm_instruction),
                    22 => parse_DSRLV(asm_instruction),
                    23 => parse_DSRAV(asm_instruction),
                    24 => parse_MULT(asm_instruction),
                    25 => parse_MULTU(asm_instruction),
                    26 => parse_DIV(asm_instruction),
                    27 => parse_DIVU(asm_instruction),
                    32 => parse_ADD(asm_instruction),
                    33 => parse_ADDU(asm_instruction),
                    34 => parse_SUB(asm_instruction),
                    35 => parse_SUBU(asm_instruction),
                    36 => parse_AND(asm_instruction),
                    37 => parse_OR(asm_instruction),
                    38 => parse_XOR(asm_instruction),
                    39 => parse_NOR(asm_instruction),
                    42 => parse_SLT(asm_instruction),
                    43 => parse_SLTU(asm_instruction),
                    44 => parse_DADD(asm_instruction),
                    45 => parse_DADDU(asm_instruction),
                    46 => parse_DSUB(asm_instruction),
                    47 => parse_DSUBU(asm_instruction),
                    48 => parse_TGE(asm_instruction),
                    49 => parse_TGEU(asm_instruction),
                    50 => parse_TLT(asm_instruction),
                    51 => parse_TLTU(asm_instruction),
                    52 => parse_TEQ(asm_instruction),
                    54 => parse_TNE(asm_instruction),
                    56 => parse_DSLL(asm_instruction),
                    58 => parse_DSRL(asm_instruction),
                    59 => parse_DSRA(asm_instruction),
                    60 => parse_DSLL32(asm_instruction),
                    62 => parse_DSRL32(asm_instruction),
                    63 => parse_DSRA32(asm_instruction),
                    _ => NOP,
                }
            },
            InstructionType::Regimm { opcode } => {
                match opcode {
                    0 => parse_BLTZ(asm_instruction),
                    1 => parse_BGEZ(asm_instruction),
                    2 => parse_BLTZL(asm_instruction),
                    3 => parse_BGEZL(asm_instruction),
                    8 => parse_TGEI(asm_instruction),
                    9 => parse_TGEIU(asm_instruction),
                    10 => parse_TLTI(asm_instruction),
                    11 => parse_TLTIU(asm_instruction),
                    12 => parse_TEQI(asm_instruction),
                    13 => parse_BREAK(asm_instruction),
                    14 => parse_TNEI(asm_instruction),
                    16 => parse_BLTZAL(asm_instruction),
                    17 => parse_BGEZAL(asm_instruction),
                    18 => parse_BLTZALL(asm_instruction),
                    19 => parse_BGEZALL(asm_instruction),
                    _ => NOP
                }
            }
            InstructionType::Normal { opcode } => {
                match opcode {
                    2 => parse_J(asm_instruction),
                    3 => parse_JAL(asm_instruction),
                    4 => parse_BEQ(asm_instruction),
                    5 => parse_BNE(asm_instruction),
                    6 => parse_BLEZ(asm_instruction),
                    7 => parse_BGTZ(asm_instruction),
                    8 => parse_ADDI(asm_instruction),
                    9 => parse_ADDI(asm_instruction),
                    10 => parse_SLTI(asm_instruction),
                    11 => parse_SLTIU(asm_instruction),
                    12 => parse_ANDI(asm_instruction),
                    13 => parse_ORI(asm_instruction),
                    14 => parse_XORI(asm_instruction),
                    15 => parse_LUI(asm_instruction),
                    20 => parse_BEQL(asm_instruction),
                    21 => parse_BNEL(asm_instruction),
                    22 => parse_BLEZL(asm_instruction),
                    23 => parse_BGTZL(asm_instruction),
                    24 => parse_DADDI(asm_instruction),
                    25 => parse_DADDIU(asm_instruction),
                    26 => parse_LDL(asm_instruction),
                    26 => parse_LDR(asm_instruction),
                    32 => parse_LB(asm_instruction),
                    33 => parse_LH(asm_instruction),
                    34 => parse_LWL(asm_instruction),
                    35 => parse_LW(asm_instruction),
                    36 => parse_LBU(asm_instruction),
                    37 => parse_LHU(asm_instruction),
                    38 => parse_LWR(asm_instruction),
                    39 => parse_LWU(asm_instruction),
                    40 => parse_SB(asm_instruction),
                    41 => parse_SH(asm_instruction),
                    42 => parse_SWL(asm_instruction),
                    43 => parse_SW(asm_instruction),
                    44 => parse_SDL(asm_instruction),
                    45 => parse_SDR(asm_instruction),
                    50 => parse_SWR(asm_instruction),
                    51 => parse_PREF(asm_instruction),
                    55 => parse_LD(asm_instruction),
                    63 => parse_SD(asm_instruction),
                    _ => NOP,
                }
            },
        });
    }

    out_instructions
}