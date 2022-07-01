use super::immediate::Immediate;
use super::{modrm, sib};
use super::register::Register;
use super::memory::Memory;
use super::operand::Operand;

fn validate_operands(operand1: Operand, operand2: Operand) {
    match operand1 {
        Operand::Register(_) => match operand2 {
            Operand::Register(_) => {},
            Operand::Memory(_) => panic!("invalid operands"),
        },
        Operand::Memory(_) => match operand2 {
            Operand::Register(_) => {},
            Operand::Memory(_) => panic!("invalid operands"),
        },
    }
}

fn emit_immediate(immediate: Immediate, buffer: &mut Vec<u8>) {
    match immediate {
        Immediate::U8(imm) => buffer.extend_from_slice(
            &imm.to_le_bytes()
        ),

        Immediate::U16(imm) => buffer.extend_from_slice(
            &imm.to_le_bytes()
        ),

        Immediate::U32(imm) => buffer.extend_from_slice(
            &imm.to_le_bytes()
        ),
    }
}

fn emit_modrm_reg_reg(rm: Register, register: Register, buffer: &mut Vec<u8>) {
    buffer.push(
        modrm::gen_modrm(
            modrm::Mod::Reg,
            modrm::register_to_rm(rm),
            modrm::register_to_reg(register)
        )
    );
}

fn emit_modrm_mem_reg(rm: Memory, register: Register, buffer: &mut Vec<u8>) {
    match rm {
        Memory::Register(reg) => {
            buffer.push(
                modrm::gen_modrm(
                    modrm::Mod::NoDisp,
                    modrm::register_to_rm(reg),
                    modrm::register_to_reg(register)
                )
            );
        },

        Memory::Immediate(imm) => {
            buffer.push(
                modrm::gen_modrm(
                    modrm::Mod::NoDisp,
                    modrm::Rm::SPEC_DISP32,
                    modrm::register_to_reg(register)
                )
            );
            
            emit_immediate(imm, buffer);
        },

        Memory::RegisterDisplacement(reg, disp) => {
            buffer.push(
                modrm::gen_modrm(
                    modrm::Mod::Disp32,
                    modrm::register_to_rm(reg),
                    modrm::register_to_reg(register)
                )
            );

            emit_immediate(disp, buffer);
        },

        Memory::BaseIndex(base, index) => {
            buffer.push(
                modrm::gen_modrm(
                    modrm::Mod::NoDisp,
                    modrm::Rm::SPEC_SIB,
                    modrm::register_to_reg(register)
                )
            );

            buffer.push(
                sib::gen_sib(
                    sib::Scale::NoScale,
                    sib::register_to_index(index),
                    sib::register_to_base(base)
                )
            );
        },

        Memory::IndexScale(index, scale) => {
            buffer.push(
                modrm::gen_modrm(
                    modrm::Mod::NoDisp,
                    modrm::Rm::SPEC_SIB,
                    modrm::register_to_reg(register)
                )
            );

            buffer.push(
                sib::gen_sib(
                    match scale {
                        super::memory::Scale::X2 => sib::Scale::Scale2,
                        super::memory::Scale::X4 => sib::Scale::Scale4,
                        super::memory::Scale::X8 => sib::Scale::Scale8,
                    },
                    sib::register_to_index(index),
                    sib::Base::ModSpecific
                )
            );
        },

        Memory::BaseIndexScale(base, index, scale) => {
            buffer.push(
                modrm::gen_modrm(
                    modrm::Mod::NoDisp,
                    modrm::Rm::SPEC_SIB,
                    modrm::register_to_reg(register)
                )
            );

            buffer.push(
                sib::gen_sib(
                    match scale {
                        super::memory::Scale::X2 => sib::Scale::Scale2,
                        super::memory::Scale::X4 => sib::Scale::Scale4,
                        super::memory::Scale::X8 => sib::Scale::Scale8,
                    },
                    sib::register_to_index(index),
                    sib::register_to_base(base)
                )
            );
        },

        Memory::BaseIndexScaleDisplacement(base, index, scale, disp) => {
            buffer.push(
                modrm::gen_modrm(
                    modrm::Mod::Disp32,
                    modrm::Rm::SPEC_SIB,
                    modrm::register_to_reg(register)
                )
            );

            buffer.push(
                sib::gen_sib(
                    match scale {
                        super::memory::Scale::X2 => sib::Scale::Scale2,
                        super::memory::Scale::X4 => sib::Scale::Scale4,
                        super::memory::Scale::X8 => sib::Scale::Scale8,
                    },
                    sib::register_to_index(index),
                    sib::register_to_base(base)
                )
            );

            emit_immediate(disp, buffer);
        },
    }
}

pub fn emit_modrm_byte(operand1: Operand, operand2: Operand, buffer: &mut Vec<u8>) {
    validate_operands(operand1, operand2);

    match operand1 {
        Operand::Register(register) => emit_modrm_reg_reg(
            register,
            match operand2 {
                Operand::Register(reg) => reg,
                _ => unreachable!()
            },
            buffer
        ),

        Operand::Memory(memory) => emit_modrm_mem_reg(
            memory,
            match operand2 {
                Operand::Register(reg) => reg,
                _ => unreachable!(),
            },
            buffer
        ),
    }
}

