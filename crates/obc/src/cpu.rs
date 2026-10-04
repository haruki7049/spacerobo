//! A minimal RV32I CPU core: registers, program counter, and a fetch-decode-execute step.
//!
//! Scoped to the RV32I base integer opcodes `spelling`'s `internal/vm/cpu.go` implements: `LUI`,
//! `AUIPC`, `JAL`, `JALR`, the branches, loads, stores, the `OP-IMM` family and the `OP` family.
//! `ECALL`/`EBREAK`/`FENCE`/`WFI` and any privileged behavior are out of scope. An unsupported
//! opcode is treated as a no-op (`pc` still advances by 4).

use crate::bus::Bus;

/// Sign-extends the low `bits` bits of `value`, treating bit `bits - 1` as the sign bit.
fn sign_extend(value: u32, bits: u32) -> i32 {
    let shift = 32u32.wrapping_sub(bits);
    value.wrapping_shl(shift).cast_signed().wrapping_shr(shift)
}

/// Decodes an I-type immediate (`OP-IMM`, loads, `JALR`): `inst[31:20]`, sign-extended.
fn imm_i(inst: u32) -> i32 {
    sign_extend(inst.wrapping_shr(20), 12)
}

/// Decodes an S-type immediate (stores): `inst[31:25]:inst[11:7]`, sign-extended.
fn imm_s(inst: u32) -> i32 {
    let hi = inst.wrapping_shr(25) & 0x7f;
    let lo = inst.wrapping_shr(7) & 0x1f;
    sign_extend(hi.wrapping_shl(5) | lo, 12)
}

/// Decodes a B-type immediate (branches): `inst[31]:inst[7]:inst[30:25]:inst[11:8]:0`, sign-extended.
fn imm_b(inst: u32) -> i32 {
    let bit12 = inst.wrapping_shr(31) & 0x1;
    let bit11 = inst.wrapping_shr(7) & 0x1;
    let bits10_5 = inst.wrapping_shr(25) & 0x3f;
    let bits4_1 = inst.wrapping_shr(8) & 0xf;
    let value = bit12
        .wrapping_shl(12)
        .wrapping_add(bit11.wrapping_shl(11))
        .wrapping_add(bits10_5.wrapping_shl(5))
        .wrapping_add(bits4_1.wrapping_shl(1));
    sign_extend(value, 13)
}

/// Decodes a U-type immediate (`LUI`, `AUIPC`): `inst[31:12]`, already in its final position.
fn imm_u(inst: u32) -> i32 {
    (inst & 0xffff_f000).cast_signed()
}

/// Decodes a J-type immediate (`JAL`): `inst[31]:inst[19:12]:inst[20]:inst[30:21]:0`, sign-extended.
fn imm_j(inst: u32) -> i32 {
    let bit20 = inst.wrapping_shr(31) & 0x1;
    let bits19_12 = inst.wrapping_shr(12) & 0xff;
    let bit11 = inst.wrapping_shr(20) & 0x1;
    let bits10_1 = inst.wrapping_shr(21) & 0x3ff;
    let value = bit20
        .wrapping_shl(20)
        .wrapping_add(bits19_12.wrapping_shl(12))
        .wrapping_add(bit11.wrapping_shl(11))
        .wrapping_add(bits10_1.wrapping_shl(1));
    sign_extend(value, 21)
}

/// RV32I CPU state: 32 general-purpose registers (`x0` always reads as zero) and the program
/// counter.
#[derive(Debug, Default)]
pub struct Cpu {
    regs: [u32; 32],
    pub pc: u32,
}

impl Cpu {
    /// Creates a CPU with all registers and `pc` at zero.
    pub fn new() -> Self {
        Self::default()
    }

    /// Reads register `index` (0-31). `x0` always reads as zero.
    pub fn reg(&self, index: u32) -> u32 {
        self.regs[index as usize]
    }

    /// Writes register `index` (0-31). A write to `x0` is discarded.
    fn set_reg(&mut self, index: u32, value: u32) {
        if index != 0 {
            self.regs[index as usize] = value;
        }
    }

    /// Fetches the instruction at `pc` from `bus`, decodes and executes it, then advances `pc`
    /// (sequentially, or to a jump/branch target).
    pub fn step<B: Bus + ?Sized>(&mut self, bus: &mut B) {
        let inst = bus.read(self.pc, 4);
        let opcode = inst & 0x7f;
        let rd = inst.wrapping_shr(7) & 0x1f;
        let funct3 = inst.wrapping_shr(12) & 0x7;
        let rs1 = inst.wrapping_shr(15) & 0x1f;
        let rs2 = inst.wrapping_shr(20) & 0x1f;
        let funct7 = inst.wrapping_shr(25) & 0x7f;
        let shamt = rs2 & 0x1f;

        let mut next_pc = self.pc.wrapping_add(4);

        match opcode {
            // LUI
            0x37 => self.set_reg(rd, imm_u(inst).cast_unsigned()),
            // AUIPC
            0x17 => self.set_reg(rd, self.pc.wrapping_add(imm_u(inst).cast_unsigned())),
            // JAL
            0x6f => {
                self.set_reg(rd, next_pc);
                next_pc = self.pc.wrapping_add(imm_j(inst).cast_unsigned());
            }
            // JALR
            0x67 => {
                let base = self.reg(rs1).cast_signed();
                let target = base.wrapping_add(imm_i(inst)).cast_unsigned();
                self.set_reg(rd, next_pc);
                next_pc = target & !1;
            }
            // Branches: BEQ, BNE, BLT, BGE, BLTU, BGEU
            0x63 => {
                let a = self.reg(rs1);
                let b = self.reg(rs2);
                let taken = match funct3 {
                    0x0 => a == b,
                    0x1 => a != b,
                    0x4 => a.cast_signed() < b.cast_signed(),
                    0x5 => a.cast_signed() >= b.cast_signed(),
                    0x6 => a < b,
                    0x7 => a >= b,
                    _ => false,
                };
                if taken {
                    next_pc = self.pc.wrapping_add(imm_b(inst).cast_unsigned());
                }
            }
            // Loads: LB, LH, LW, LBU, LHU
            0x03 => {
                let addr = self
                    .reg(rs1)
                    .cast_signed()
                    .wrapping_add(imm_i(inst))
                    .cast_unsigned();
                let value = match funct3 {
                    0x0 => sign_extend(bus.read(addr, 1), 8).cast_unsigned(),
                    0x1 => sign_extend(bus.read(addr, 2), 16).cast_unsigned(),
                    0x2 => bus.read(addr, 4),
                    0x4 => bus.read(addr, 1),
                    0x5 => bus.read(addr, 2),
                    _ => 0,
                };
                self.set_reg(rd, value);
            }
            // Stores: SB, SH, SW
            0x23 => {
                let addr = self
                    .reg(rs1)
                    .cast_signed()
                    .wrapping_add(imm_s(inst))
                    .cast_unsigned();
                let value = self.reg(rs2);
                match funct3 {
                    0x0 => bus.write(addr, 1, value),
                    0x1 => bus.write(addr, 2, value),
                    0x2 => bus.write(addr, 4, value),
                    _ => {}
                }
            }
            // OP-IMM: ADDI, SLTI, SLTIU, XORI, ORI, ANDI, SLLI, SRLI, SRAI
            0x13 => {
                let a = self.reg(rs1);
                let imm = imm_i(inst);
                let value = match funct3 {
                    0x0 => a.cast_signed().wrapping_add(imm).cast_unsigned(),
                    0x2 => u32::from(a.cast_signed() < imm),
                    0x3 => u32::from(a < imm.cast_unsigned()),
                    0x4 => a ^ imm.cast_unsigned(),
                    0x6 => a | imm.cast_unsigned(),
                    0x7 => a & imm.cast_unsigned(),
                    0x1 => a.wrapping_shl(shamt),
                    0x5 if funct7 & 0x20 == 0 => a.wrapping_shr(shamt),
                    0x5 => a.cast_signed().wrapping_shr(shamt).cast_unsigned(),
                    _ => 0,
                };
                self.set_reg(rd, value);
            }
            // OP: ADD, SUB, SLL, SLT, SLTU, XOR, SRL, SRA, OR, AND
            0x33 => {
                let a = self.reg(rs1);
                let b = self.reg(rs2);
                let bshamt = b & 0x1f;
                let value = match (funct3, funct7) {
                    (0x0, 0x00) => a.wrapping_add(b),
                    (0x0, 0x20) => a.wrapping_sub(b),
                    (0x1, _) => a.wrapping_shl(bshamt),
                    (0x2, _) => u32::from(a.cast_signed() < b.cast_signed()),
                    (0x3, _) => u32::from(a < b),
                    (0x4, _) => a ^ b,
                    (0x5, 0x00) => a.wrapping_shr(bshamt),
                    (0x5, 0x20) => a.cast_signed().wrapping_shr(bshamt).cast_unsigned(),
                    (0x6, _) => a | b,
                    (0x7, _) => a & b,
                    _ => 0,
                };
                self.set_reg(rd, value);
            }
            // Unsupported opcode: no-op, pc still advances sequentially.
            _ => {}
        }

        self.pc = next_pc;
    }
}

#[cfg(test)]
mod tests {
    use super::Cpu;
    use crate::bus::{Bus, Ram};

    fn encode_r(opcode: u32, rd: u32, funct3: u32, rs1: u32, rs2: u32, funct7: u32) -> u32 {
        opcode
            .wrapping_add(rd.wrapping_shl(7))
            .wrapping_add(funct3.wrapping_shl(12))
            .wrapping_add(rs1.wrapping_shl(15))
            .wrapping_add(rs2.wrapping_shl(20))
            .wrapping_add(funct7.wrapping_shl(25))
    }

    fn encode_i(opcode: u32, rd: u32, funct3: u32, rs1: u32, imm: i32) -> u32 {
        opcode
            .wrapping_add(rd.wrapping_shl(7))
            .wrapping_add(funct3.wrapping_shl(12))
            .wrapping_add(rs1.wrapping_shl(15))
            .wrapping_add(imm.cast_unsigned().wrapping_shl(20))
    }

    fn encode_s(opcode: u32, funct3: u32, rs1: u32, rs2: u32, imm: i32) -> u32 {
        let imm = imm.cast_unsigned();
        let lo = imm & 0x1f;
        let hi = imm.wrapping_shr(5) & 0x7f;
        opcode
            .wrapping_add(lo.wrapping_shl(7))
            .wrapping_add(funct3.wrapping_shl(12))
            .wrapping_add(rs1.wrapping_shl(15))
            .wrapping_add(rs2.wrapping_shl(20))
            .wrapping_add(hi.wrapping_shl(25))
    }

    fn encode_b(opcode: u32, funct3: u32, rs1: u32, rs2: u32, imm: i32) -> u32 {
        let imm = imm.cast_unsigned();
        let bit11 = imm.wrapping_shr(11) & 0x1;
        let bits4_1 = imm.wrapping_shr(1) & 0xf;
        let bits10_5 = imm.wrapping_shr(5) & 0x3f;
        let bit12 = imm.wrapping_shr(12) & 0x1;
        opcode
            .wrapping_add(bit11.wrapping_shl(7))
            .wrapping_add(bits4_1.wrapping_shl(8))
            .wrapping_add(funct3.wrapping_shl(12))
            .wrapping_add(rs1.wrapping_shl(15))
            .wrapping_add(rs2.wrapping_shl(20))
            .wrapping_add(bits10_5.wrapping_shl(25))
            .wrapping_add(bit12.wrapping_shl(31))
    }

    fn encode_u(opcode: u32, rd: u32, imm: i32) -> u32 {
        opcode
            .wrapping_add(rd.wrapping_shl(7))
            .wrapping_add(imm.cast_unsigned() & 0xffff_f000)
    }

    fn encode_j(opcode: u32, rd: u32, imm: i32) -> u32 {
        let imm = imm.cast_unsigned();
        let bit20 = imm.wrapping_shr(20) & 0x1;
        let bits10_1 = imm.wrapping_shr(1) & 0x3ff;
        let bit11 = imm.wrapping_shr(11) & 0x1;
        let bits19_12 = imm.wrapping_shr(12) & 0xff;
        opcode
            .wrapping_add(rd.wrapping_shl(7))
            .wrapping_add(bits19_12.wrapping_shl(12))
            .wrapping_add(bit11.wrapping_shl(20))
            .wrapping_add(bits10_1.wrapping_shl(21))
            .wrapping_add(bit20.wrapping_shl(31))
    }

    /// x0's unit tests
    mod x0 {
        use super::{Bus, Cpu, Ram, encode_i};

        /// A write to x0 (via ADDI x0, x1, 1) is discarded; x0 still reads as zero
        #[test]
        fn write_is_discarded() {
            let mut cpu = Cpu::new();
            let mut bus = Ram::new(4);
            bus.write(0, 4, encode_i(0x13, 0, 0x0, 1, 1));
            cpu.step(&mut bus);
            assert_eq!(cpu.reg(0), 0);
        }
    }

    /// LUI and AUIPC's unit tests
    mod upper_immediate {
        use super::{Bus, Cpu, Ram, encode_u};

        /// LUI loads the upper 20 bits into rd and pc advances by 4
        #[test]
        fn lui_loads_the_upper_immediate() {
            let mut cpu = Cpu::new();
            let mut bus = Ram::new(4);
            bus.write(0, 4, encode_u(0x37, 1, 0x1234_5000));
            cpu.step(&mut bus);
            assert_eq!(cpu.reg(1), 0x1234_5000);
            assert_eq!(cpu.pc, 4);
        }

        /// AUIPC adds the upper immediate to the current pc
        #[test]
        fn auipc_adds_to_pc() {
            let mut cpu = Cpu::new();
            cpu.pc = 0x100;
            let mut bus = Ram::new(0x104);
            bus.write(0x100, 4, encode_u(0x17, 1, 0x1000));
            cpu.step(&mut bus);
            assert_eq!(cpu.reg(1), 0x1100);
            assert_eq!(cpu.pc, 0x104);
        }
    }

    /// JAL and JALR's unit tests
    mod jumps {
        use super::{Bus, Cpu, Ram, encode_i, encode_j};

        /// JAL stores the return address in rd and jumps to pc + imm
        #[test]
        fn jal_jumps_and_links() {
            let mut cpu = Cpu::new();
            let mut bus = Ram::new(16);
            bus.write(0, 4, encode_j(0x6f, 1, 12));
            cpu.step(&mut bus);
            assert_eq!(cpu.reg(1), 4);
            assert_eq!(cpu.pc, 12);
        }

        /// JALR jumps to rs1 + imm with bit 0 cleared, and links rd
        #[test]
        fn jalr_jumps_to_register_plus_offset() {
            let mut cpu = Cpu::new();
            let mut bus = Ram::new(16);
            bus.write(0, 4, encode_i(0x13, 2, 0x0, 0, 0x41)); // ADDI x2, x0, 0x41
            bus.write(4, 4, encode_i(0x67, 1, 0x0, 2, 4)); // JALR x1, 4(x2)
            cpu.step(&mut bus);
            cpu.step(&mut bus);
            assert_eq!(cpu.reg(1), 8);
            assert_eq!(cpu.pc, 0x44);
        }
    }

    /// Branch instructions' unit tests
    mod branches {
        use super::{Bus, Cpu, Ram, encode_b, encode_i};

        /// BEQ takes the branch when the operands are equal
        #[test]
        fn beq_taken() {
            let mut cpu = Cpu::new();
            let mut bus = Ram::new(16);
            // x1 = x2 = 0, so BEQ x1, x2, 8 is taken.
            bus.write(0, 4, encode_b(0x63, 0x0, 1, 2, 8));
            cpu.step(&mut bus);
            assert_eq!(cpu.pc, 8);
        }

        /// BEQ falls through to the next instruction when the operands differ
        #[test]
        fn beq_not_taken() {
            let mut cpu = Cpu::new();
            let mut bus = Ram::new(16);
            // ADDI x2, x0, 1 so x1 (0) != x2 (1).
            bus.write(0, 4, encode_i(0x13, 2, 0x0, 0, 1));
            bus.write(4, 4, encode_b(0x63, 0x0, 1, 2, 8));
            cpu.step(&mut bus); // ADDI
            cpu.step(&mut bus); // BEQ, not taken
            assert_eq!(cpu.pc, 8);
        }

        /// BLT takes the branch for a signed less-than comparison, even across the sign boundary
        #[test]
        fn blt_signed_comparison() {
            let mut cpu = Cpu::new();
            let mut bus = Ram::new(16);
            // x1 = -1 (ADDI x1, x0, -1), x2 = 0 (x0). BLT x1, x2, 8 is taken: -1 < 0.
            bus.write(0, 4, super::encode_i(0x13, 1, 0x0, 0, -1));
            bus.write(4, 4, encode_b(0x63, 0x4, 1, 2, 8));
            cpu.step(&mut bus);
            cpu.step(&mut bus);
            assert_eq!(cpu.pc, 12);
        }

        /// BLTU treats the same bit pattern as unsigned, so -1 (u32::MAX) is not less than 0
        #[test]
        fn bltu_unsigned_comparison() {
            let mut cpu = Cpu::new();
            let mut bus = Ram::new(16);
            bus.write(0, 4, super::encode_i(0x13, 1, 0x0, 0, -1));
            bus.write(4, 4, encode_b(0x63, 0x6, 1, 2, 8));
            cpu.step(&mut bus);
            cpu.step(&mut bus);
            assert_eq!(cpu.pc, 8);
        }
    }

    /// Load instructions' unit tests
    mod loads {
        use super::{Bus, Cpu, Ram, encode_i};

        /// LB sign-extends a negative byte; LBU zero-extends the same byte
        #[test]
        fn lb_sign_extends_and_lbu_zero_extends() {
            let mut cpu = Cpu::new();
            let mut bus = Ram::new(16);
            bus.write(8, 1, 0xFF);
            bus.write(0, 4, encode_i(0x03, 1, 0x0, 0, 8)); // LB x1, 8(x0)
            bus.write(4, 4, encode_i(0x03, 2, 0x4, 0, 8)); // LBU x2, 8(x0)
            cpu.step(&mut bus);
            cpu.step(&mut bus);
            assert_eq!(cpu.reg(1), 0xFFFF_FFFF);
            assert_eq!(cpu.reg(2), 0xFF);
        }

        /// LW loads a full word written through the bus
        #[test]
        fn lw_loads_a_word() {
            let mut cpu = Cpu::new();
            let mut bus = Ram::new(16);
            bus.write(8, 4, 0xDEAD_BEEF);
            bus.write(0, 4, encode_i(0x03, 1, 0x2, 0, 8));
            cpu.step(&mut bus);
            assert_eq!(cpu.reg(1), 0xDEAD_BEEF);
        }
    }

    /// Store instructions' unit tests
    mod stores {
        use super::{Bus, Cpu, Ram, encode_i, encode_s};

        /// SW stores a register's full word at rs1 + imm
        #[test]
        fn sw_stores_a_word() {
            let mut cpu = Cpu::new();
            let mut bus = Ram::new(16);
            bus.write(0, 4, encode_i(0x13, 1, 0x0, 0, -1)); // x1 = -1
            bus.write(4, 4, encode_s(0x23, 0x2, 0, 1, 8)); // SW x1, 8(x0)
            cpu.step(&mut bus);
            cpu.step(&mut bus);
            assert_eq!(bus.read(8, 4), 0xFFFF_FFFF);
        }
    }

    /// OP-IMM family's unit tests
    mod op_imm {
        use super::{Bus, Cpu, Ram, encode_i};

        /// ADDI adds a sign-extended immediate to rs1
        #[test]
        fn addi() {
            let mut cpu = Cpu::new();
            let mut bus = Ram::new(4);
            bus.write(0, 4, encode_i(0x13, 1, 0x0, 0, -1));
            cpu.step(&mut bus);
            assert_eq!(cpu.reg(1), 0xFFFF_FFFF);
        }

        /// SLTI sets rd to 1 when rs1 is signed-less-than the immediate
        #[test]
        fn slti() {
            let mut cpu = Cpu::new();
            let mut bus = Ram::new(8);
            bus.write(0, 4, encode_i(0x13, 1, 0x0, 0, -1)); // x1 = -1
            bus.write(4, 4, encode_i(0x13, 2, 0x2, 1, 0)); // SLTI x2, x1, 0
            cpu.step(&mut bus);
            cpu.step(&mut bus);
            assert_eq!(cpu.reg(2), 1);
        }

        /// SLTIU compares the same bit pattern as unsigned, so -1 is not less than 1
        #[test]
        fn sltiu() {
            let mut cpu = Cpu::new();
            let mut bus = Ram::new(8);
            bus.write(0, 4, encode_i(0x13, 1, 0x0, 0, -1)); // x1 = u32::MAX
            bus.write(4, 4, encode_i(0x13, 2, 0x3, 1, 1)); // SLTIU x2, x1, 1
            cpu.step(&mut bus);
            cpu.step(&mut bus);
            assert_eq!(cpu.reg(2), 0);
        }

        /// XORI, ORI and ANDI apply their bitwise operation with the sign-extended immediate
        #[test]
        fn xori_ori_andi() {
            let mut cpu = Cpu::new();
            let mut bus = Ram::new(16);
            bus.write(0, 4, encode_i(0x13, 1, 0x0, 0, 0b1100)); // x1 = 0b1100
            bus.write(4, 4, encode_i(0x13, 2, 0x4, 1, 0b1010)); // XORI
            bus.write(8, 4, encode_i(0x13, 3, 0x6, 1, 0b1010)); // ORI
            bus.write(12, 4, encode_i(0x13, 4, 0x7, 1, 0b1010)); // ANDI
            for _ in 0..4 {
                cpu.step(&mut bus);
            }
            assert_eq!(cpu.reg(2), 0b0110);
            assert_eq!(cpu.reg(3), 0b1110);
            assert_eq!(cpu.reg(4), 0b1000);
        }

        /// SLLI, SRLI and SRAI shift rs1 by the immediate's low 5 bits
        #[test]
        fn slli_srli_srai() {
            let mut cpu = Cpu::new();
            let mut bus = Ram::new(16);
            bus.write(0, 4, encode_i(0x13, 1, 0x0, 0, -1)); // x1 = u32::MAX
            bus.write(4, 4, encode_i(0x13, 2, 0x1, 1, 4)); // SLLI x2, x1, 4
            bus.write(8, 4, encode_i(0x13, 3, 0x5, 1, 4)); // SRLI x3, x1, 4
            bus.write(12, 4, super::encode_r(0x13, 4, 0x5, 1, 4, 0x20)); // SRAI x4, x1, 4
            for _ in 0..4 {
                cpu.step(&mut bus);
            }
            assert_eq!(cpu.reg(2), 0xFFFF_FFF0);
            assert_eq!(cpu.reg(3), 0x0FFF_FFFF);
            assert_eq!(cpu.reg(4), 0xFFFF_FFFF);
        }
    }

    /// OP family's unit tests
    mod op {
        use super::{Bus, Cpu, Ram, encode_i, encode_r};

        /// ADD and SUB on two registers
        #[test]
        fn add_and_sub() {
            let mut cpu = Cpu::new();
            let mut bus = Ram::new(16);
            bus.write(0, 4, encode_i(0x13, 1, 0x0, 0, 5)); // x1 = 5
            bus.write(4, 4, encode_i(0x13, 2, 0x0, 0, 3)); // x2 = 3
            bus.write(8, 4, encode_r(0x33, 3, 0x0, 1, 2, 0x00)); // ADD x3, x1, x2
            bus.write(12, 4, encode_r(0x33, 4, 0x0, 1, 2, 0x20)); // SUB x4, x1, x2
            for _ in 0..4 {
                cpu.step(&mut bus);
            }
            assert_eq!(cpu.reg(3), 8);
            assert_eq!(cpu.reg(4), 2);
        }

        /// SLT and SLTU on the same bit pattern give different results for a negative register
        #[test]
        fn slt_and_sltu() {
            let mut cpu = Cpu::new();
            let mut bus = Ram::new(16);
            bus.write(0, 4, encode_i(0x13, 1, 0x0, 0, -1)); // x1 = u32::MAX
            bus.write(4, 4, encode_i(0x13, 2, 0x0, 0, 0)); // x2 = 0
            bus.write(8, 4, encode_r(0x33, 3, 0x2, 1, 2, 0x00)); // SLT x3, x1, x2
            bus.write(12, 4, encode_r(0x33, 4, 0x3, 1, 2, 0x00)); // SLTU x4, x1, x2
            for _ in 0..4 {
                cpu.step(&mut bus);
            }
            assert_eq!(cpu.reg(3), 1);
            assert_eq!(cpu.reg(4), 0);
        }

        /// XOR, OR and AND on two registers
        #[test]
        fn xor_or_and() {
            let mut cpu = Cpu::new();
            let mut bus = Ram::new(20);
            bus.write(0, 4, encode_i(0x13, 1, 0x0, 0, 0b1100));
            bus.write(4, 4, encode_i(0x13, 2, 0x0, 0, 0b1010));
            bus.write(8, 4, encode_r(0x33, 3, 0x4, 1, 2, 0x00)); // XOR
            bus.write(12, 4, encode_r(0x33, 4, 0x6, 1, 2, 0x00)); // OR
            bus.write(16, 4, encode_r(0x33, 5, 0x7, 1, 2, 0x00)); // AND
            for _ in 0..5 {
                cpu.step(&mut bus);
            }
            assert_eq!(cpu.reg(3), 0b0110);
            assert_eq!(cpu.reg(4), 0b1110);
            assert_eq!(cpu.reg(5), 0b1000);
        }

        /// SLL, SRL and SRA on two registers, shift amount taken from the low 5 bits of rs2
        #[test]
        fn sll_srl_sra() {
            let mut cpu = Cpu::new();
            let mut bus = Ram::new(20);
            bus.write(0, 4, encode_i(0x13, 1, 0x0, 0, -1)); // x1 = u32::MAX
            bus.write(4, 4, encode_i(0x13, 2, 0x0, 0, 4)); // x2 = 4
            bus.write(8, 4, encode_r(0x33, 3, 0x1, 1, 2, 0x00)); // SLL
            bus.write(12, 4, encode_r(0x33, 4, 0x5, 1, 2, 0x00)); // SRL
            bus.write(16, 4, encode_r(0x33, 5, 0x5, 1, 2, 0x20)); // SRA
            for _ in 0..5 {
                cpu.step(&mut bus);
            }
            assert_eq!(cpu.reg(3), 0xFFFF_FFF0);
            assert_eq!(cpu.reg(4), 0x0FFF_FFFF);
            assert_eq!(cpu.reg(5), 0xFFFF_FFFF);
        }
    }
}
