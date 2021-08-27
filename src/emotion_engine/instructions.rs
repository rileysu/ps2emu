pub const NOP: Instruction = Instruction::SLL {
    rt: 0,
    rd: 0,
    sa: 0,
};
pub enum Instruction {
    ADD {
        rd: u8,
        rs: u8,
        rt: u8,
    },
    ADDI {
        rt: u8,
        rs: u8,
        imm: i16,
    },
    ADDUI {
        rt: u8,
        rs: u8,
        imm: i16,
    },
    ADDU {
        rd: u8,
        rs: u8,
        rt: u8,
    },
    AND {
        rd: u8,
        rs: u8,
        rt: u8,    
    },
    ANDI {
        rt: u8,
        rs: u8,
        imm: u16, 
    },
    BEQ {
        rs: u8,
        rt: u8,
        off: u16,
    },
    BEQL {
        rs: u8,
        rt: u8,
        off: u16,  
    },
    BGEZ {
        rs: u8,
        off: u16,
    },
    BGEZAL {
        rs: u8,
        off: u16,
    },
    BGEZALL {
        rs: u8,
        off: u16,
    },
    BGEZL {
        rs: u8,
        off: u16,
    },
    BGTZ {
        rs: u8,
        off: u16,
    },
    BGTZL {
        rs: u8,
        off: u16,
    },
    BLEZ {
        rs: u8,
        off: u16,
    },
    BLEZL {
        rs: u8,
        off: u16,
    },
    BLTZ {
        rs: u8,
        off: u16,
    },
    BLTZAL {
        rs: u8,
        off: u16,
    },
    BLTZALL {
        rs: u8,
        off: u16,
    },
    BLTZL {
        rs: u8,
        off: u16,
    },
    BNE {
        rs: u8,
        rt: u8,
        off: u16,
    },
    BNEL {
        rs: u8,
        rt: u8,
        off: u16,
    },
    BREAK {
        code: u32
    },
    DADD {
        rd: u8,
        rs: u8,
        rt: u8,
    },
    DADDI {
        rt: u8,
        rs: u8,
        imm: i16,
    },
    DADDIU {
        rt: u8,
        rs: u8,
        imm: i16,
    },
    DADDU {
        rd: u8,
        rs: u8,
        rt: u8,
    },
    DIV {
        rs: u8,
        rt: u8,
    },
    DIVU {
        rs: u8,
        rt: u8,
    },
    DSLL {
        rt: u8,
        rd: u8,
        sa: u8,
    },
    DSLL32 {
        rt: u8,
        rd: u8,
        sa: u8,
    },
    DSLLV {
        rd: u8,
        rt: u8,
        rs: u8,
    },
    DSRA {
        rt: u8,
        rd: u8,
        sa: u8,
    },
    DSRA32 {
        rt: u8,
        rd: u8,
        sa: u8,
    },
    DSRAV {
        rd: u8,
        rt: u8,
        rs: u8,
    },
    DSRL {
        rt: u8,
        rd: u8,
        sa: u8,
    },
    DSRL32 {
        rt: u8,
        rd: u8,
        sa: u8,
    },
    DSRLV {
        rd: u8,
        rt: u8,
        rs: u8,
    },
    DSUB {
        rd: u8,
        rs: u8,
        rt: u8,
    },
    DSUBU {
        rd: u8,
        rs: u8,
        rt: u8,
    },
    J {
        off: u32,
    },
    JAL {
        off: u32,
    },
    JALR {
        rd: u8,
        rs: u8,
    },
    JR {
        rs: u8,
    },
    LB {
        rt: u8,
        off: u16,
        base: u8,
    },
    LBU {
        rt: u8,
        off: u16,
        base: u8,
    },
    LD {
        rt: u8,
        off: u16,
        base: u8,
    },
    LDL {
        rt: u8,
        off: u16,
        base: u8,
    },
    LDR {
        rt: u8,
        off: u16,
        base: u8,
    },
    LH {
        rt: u8,
        off: u16,
        base: u8,
    },
    LHU {
        rt: u8,
        off: u16,
        base: u8,
    },
    LUI {
        rt: u8,
        imm: u16,
    },
    LW {
        rt: u8,
        off: u16,
        base: u8,
    },
    LWL {
        rt: u8,
        off: u16,
        base: u8,
    },
    LWR {
        rt: u8,
        off: u16,
        base: u8,
    },
    LWU {
        rt: u8,
        off: u16,
        base: u8,
    },
    MFHI {
        rd: u8,
    },
    MFLO {
        rd: u8,
    },
    MOVN {
        rd: u8,
        rs: u8,
        rt: u8,
    },
    MOVZ {
        rd: u8,
        rs: u8,
        rt: u8,
    },
    MTHI {
        rs: u8,
    },
    MTLO {
        rs: u8,
    },
    MULT {
        rs: u8,
        rt: u8,
    },
    MULTU {
        rs: u8,
        rt: u8,
    },
    NOR {
        rd: u8,
        rs: u8,
        rt: u8,
    },
    OR {
        rd: u8,
        rs: u8,
        rt: u8,
    },
    ORI {
        rt: u8,
        rs: u8,
        imm: u16,
    },
    PREF {
        hint: u8,
        off: u16,
        base: u8,
    },
    SB {
        rt: u8,
        off: u16,
        base: u8,
    },
    SD {
        rt: u8,
        off: u16,
        base: u8,
    },
    SDL {
        rt: u8,
        off: u16,
        base: u8,
    },
    SDR {
        rt: u8,
        off: u16,
        base: u8,
    },
    SH {
        rt: u8,
        off: u16,
        base: u8,
    },
    SLL {
        rt: u8,
        rd: u8,
        sa: u8,
    },
    SLLV {
        rd: u8,
        rt: u8,
        rs: u8,
    },
    SLT {
        rd: u8,
        rt: u8,
        rs: u8,
    },
    SLTI {
        rt: u8,
        rs: u8,
        imm: i16, 
    },
    SLTIU {
        rt: u8,
        rs: u8,
        imm: u16, 
    },
    SLTU {
        rd: u8,
        rt: u8,
        rs: u8,
    },
    SRA {
        rt: u8,
        rd: u8,
        sa: u8,
    },
    SRAV {
        rd: u8,
        rt: u8,
        rs: u8,
    },
    SRL {
        rt: u8,
        rd: u8,
        sa: u8,
    },
    SRLV {
        rd: u8,
        rt: u8,
        rs: u8,
    },
    SUB {
        rd: u8,
        rt: u8,
        rs: u8,
    },
    SUBU {
        rd: u8,
        rt: u8,
        rs: u8,
    },
    SW {
        rt: u8,
        off: u16,
        base: u8,
    },
    SWL {
        rt: u8,
        off: u16,
        base: u8,
    },
    SWR {
        rt: u8,
        off: u16,
        base: u8,
    },
    SYNC {
        stype: u8,
    },
    SYSCALL {
        code: u32,
    },
    TEQ {
        rs: u8,
        rt: u8,
        code: u16,
    },
    TEQI {
        rs: u8,
        imm: i16,
    },
    TGE {
        rs: u8,
        rt: u8,
        code: u16,
    },
    TGEI {
        rs: u8,
        imm: i16,
    },
    TGEIU {
        rs: u8,
        imm: i16,
    },
    TGEU {
        rs: u8,
        rt: u8,
        code: u16,
    },
    TLT {
        rs: u8,
        rt: u8,
        code: u16,
    },
    TLTI {
        rs: u8,
        imm: i16,
    },
    TLTIU {
        rs: u8,
        imm: i16,
    },
    TLTU {
        rs: u8,
        rt: u8,
        code: u16,
    },
    TNE {
        rs: u8,
        rt: u8,
        code: u16,
    },
    TNEI {
        rs: u8,
        imm: i16,
    },
    XOR {
        rd: u8,
        rs: u8,
        rt: u8,
    },
    XORI {
        rt: u8,
        rs: u8,
        imm: u16,
    },
}