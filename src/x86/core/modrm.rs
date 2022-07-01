use super::register::{Register, GPReg8, GPReg16, GPReg32};

//
// ModRM byte:
//
//  7                           0
// +---+---+---+---+---+---+---+---+
// |  mod  |    reg    |     rm    |
// +---+---+---+---+---+---+---+---+
//

#[allow(unused)]
#[repr(u8)]
pub enum Mod {
    NoDisp = 0b00,
    Disp8 = 0b01,
    Disp32 = 0b10,
    Reg = 0b11,
}

#[allow(unused)]
#[repr(u8)]
pub enum Rm {
    R0 = 0b000, // ax
    R1 = 0b001, // cx
    R2 = 0b010, // dx
    R3 = 0b011, // bx

    // Mod specific:
    // 0bXX - SIB
    // 0b11 - ah
    R4 = 0b100,

    // Mod specific
    // 0b00 - disp32
    // 0bXX - *bp
    R5 = 0b101,

    R6 = 0b110, // si
    R7 = 0b111, // di
}

#[allow(unused)]
impl Rm {
    pub const SPEC_SIB: Self = Self::R4;
    pub const SPEC_DISP32: Self = Self::R5;
    pub const SPEC_EBP: Self = Self::R5;
}

// 8-bit registers
#[allow(unused)]
impl Rm {
    pub const AL: Self = Self::R0;
    pub const CL: Self = Self::R1;
    pub const DL: Self = Self::R2;
    pub const BL: Self = Self::R3;
    pub const AH: Self = Self::R4;
    pub const CH: Self = Self::R5;
    pub const DH: Self = Self::R6;
    pub const BH: Self = Self::R7;
}

// 16-bit registers
#[allow(unused)]
impl Rm {
    pub const AX: Self = Self::R0;
    pub const CX: Self = Self::R1;
    pub const DX: Self = Self::R2;
    pub const BX: Self = Self::R3;
    pub const SP: Self = Self::R4;
    pub const BP: Self = Self::R5;
    pub const SI: Self = Self::R6;
    pub const DI: Self = Self::R7;
}

// 32-bit registers
#[allow(unused)]
impl Rm {
    pub const EAX: Self = Self::R0;
    pub const ECX: Self = Self::R1;
    pub const EDX: Self = Self::R2;
    pub const EBX: Self = Self::R3;
    pub const ESP: Self = Self::R4;
    pub const EBP: Self = Self::R5;
    pub const ESI: Self = Self::R6;
    pub const EDI: Self = Self::R7;
}

#[allow(unused)]
pub enum Reg {
    R0 = 0b000, // ax
    R1 = 0b001, // cx
    R2 = 0b010, // dx
    R3 = 0b011, // bx
    R4 = 0b100, // sp
    R5 = 0b101, // bp
    R6 = 0b110, // si
    R7 = 0b111, // di
}

// 8-bit registers
#[allow(unused)]
impl Reg {
    pub const AL: Self = Self::R0;
    pub const CL: Self = Self::R1;
    pub const DL: Self = Self::R2;
    pub const BL: Self = Self::R3;
    pub const AH: Self = Self::R4;
    pub const CH: Self = Self::R5;
    pub const DH: Self = Self::R6;
    pub const BH: Self = Self::R7;
}

// 16-bit registers
#[allow(unused)]
impl Reg {
    pub const AX: Self = Self::R0;
    pub const CX: Self = Self::R1;
    pub const DX: Self = Self::R2;
    pub const BX: Self = Self::R3;
    pub const SP: Self = Self::R4;
    pub const BP: Self = Self::R5;
    pub const SI: Self = Self::R6;
    pub const DI: Self = Self::R7;
}

// 32-bit registers
#[allow(unused)]
impl Reg {
    pub const EAX: Self = Self::R0;
    pub const ECX: Self = Self::R1;
    pub const EDX: Self = Self::R2;
    pub const EBX: Self = Self::R3;
    pub const ESP: Self = Self::R4;
    pub const EBP: Self = Self::R5;
    pub const ESI: Self = Self::R6;
    pub const EDI: Self = Self::R7;
}

#[allow(unused)]
pub fn gen_modrm(mod_: Mod, rm: Rm, reg: Reg) -> u8 {
    let mod_value = (mod_ as u8) << 6;
    let src_value = (reg as u8) << 3;
    let dst_value = (rm as u8) << 0;
    mod_value | src_value | dst_value
}

#[allow(unused)]
pub fn register_to_rm(register: Register) -> Rm {
    match register {
        Register::GPR8(reg) => match reg {
            GPReg8::AL => Rm::AL,
            GPReg8::CL => Rm::CL,
            GPReg8::DL => Rm::DL,
            GPReg8::BL => Rm::BL,
            GPReg8::AH => Rm::AH,
            GPReg8::CH => Rm::CH,
            GPReg8::DH => Rm::DH,
            GPReg8::BH => Rm::BH,
        },
        Register::GPR16(reg) => match reg {
            GPReg16::AX => Rm::AX,
            GPReg16::CX => Rm::CX,
            GPReg16::DX => Rm::DX,
            GPReg16::BX => Rm::BX,
            GPReg16::SP => Rm::SP,
            GPReg16::BP => Rm::BP,
            GPReg16::SI => Rm::SI,
            GPReg16::DI => Rm::DI,
        },
        Register::GPR32(reg) => match reg {
            GPReg32::EAX => Rm::EAX,
            GPReg32::ECX => Rm::ECX,
            GPReg32::EDX => Rm::EDX,
            GPReg32::EBX => Rm::EBX,
            GPReg32::ESP => Rm::ESP,
            GPReg32::EBP => Rm::EBP,
            GPReg32::ESI => Rm::ESI,
            GPReg32::EDI => Rm::EDI,
        },
        Register::SegR(reg) => unimplemented!(),
    }
}

#[allow(unused)]
pub fn register_to_reg(register: Register) -> Reg {
    match register {
        Register::GPR8(reg) => match reg {
            GPReg8::AL => Reg::AL,
            GPReg8::CL => Reg::CL,
            GPReg8::DL => Reg::DL,
            GPReg8::BL => Reg::BL,
            GPReg8::AH => Reg::AH,
            GPReg8::CH => Reg::CH,
            GPReg8::DH => Reg::DH,
            GPReg8::BH => Reg::BH,
        }

        Register::GPR16(reg) => match reg {
            GPReg16::AX => Reg::AX,
            GPReg16::CX => Reg::CX,
            GPReg16::DX => Reg::DX,
            GPReg16::BX => Reg::BX,
            GPReg16::SP => Reg::SP,
            GPReg16::BP => Reg::BP,
            GPReg16::SI => Reg::SI,
            GPReg16::DI => Reg::DI,
        }

        Register::GPR32(reg) => match reg {
            GPReg32::EAX => Reg::EAX,
            GPReg32::ECX => Reg::ECX,
            GPReg32::EDX => Reg::EDX,
            GPReg32::EBX => Reg::EBX,
            GPReg32::ESP => Reg::ESP,
            GPReg32::EBP => Reg::EBP,
            GPReg32::ESI => Reg::ESI,
            GPReg32::EDI => Reg::EDI,
        }

        Register::SegR(reg) => unimplemented!(),
    }
}

#[cfg(test)]
mod tests {
    use super::{gen_modrm, Mod, Rm, Reg};

    #[test]
    fn test_modrm() {
        let result = gen_modrm(
            Mod::NoDisp,
            Rm::EDI,
            Reg::AL
        );

        assert_eq!(
            0x07,
            result
        );
    }
}
