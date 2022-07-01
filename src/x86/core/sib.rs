use super::register::{Register, GPReg32};


//
// SIB byte:
//
// 7                           0
// +---+---+---+---+---+---+---+---+
// | scale |   index   |    base   |
// +---+---+---+---+---+---+---+---+
//

pub enum Scale {
    NoScale = 0b00,
    Scale2 = 0b01,
    Scale4 = 0b10,
    Scale8 = 0b11,
}

pub enum Index {
    EAX = 0b000, // eax
    ECX = 0b001, // ecx
    EDX = 0b010, // edx
    EBX = 0b011, // ebx
    None = 0b100, // none
    EBP = 0b101, // ebp
    ESI = 0b110, // esi
    EDI = 0b111, // edi
}

pub enum Base {
    EAX = 0b000,
    ECX = 0b001,
    EDX = 0b010,
    EBX = 0b011,
    ESP = 0b100,
    ModSpecific = 0b101, // modrm specific!
    ESI = 0b110,
    EDI = 0b111,
}

pub fn gen_sib(scale: Scale, index: Index, base: Base) -> u8 {
    let scale_value = (scale as u8) << 6;
    let index_value = (index as u8) << 3;
    let base_value = (base as u8) << 0;
    scale_value | index_value | base_value
}

pub fn register_to_index(register: Register) -> Index {
    match register {
        Register::GPR32(reg) => match reg {
            GPReg32::EAX => Index::EAX,
            GPReg32::ECX => Index::ECX,
            GPReg32::EDX => Index::EDX,
            GPReg32::EBX => Index::EBX,
            GPReg32::ESP => Index::None,
            GPReg32::EBP => Index::EBP,
            GPReg32::ESI => Index::ESI,
            GPReg32::EDI => Index::EDI,
        },
        _ => unimplemented!(),
    }
}

pub fn register_to_base(register: Register) -> Base {
    match register {
        Register::GPR32(reg) => match reg {
            GPReg32::EAX => Base::EAX,
            GPReg32::ECX => Base::ECX,
            GPReg32::EDX => Base::EDX,
            GPReg32::EBX => Base::EBX,
            GPReg32::ESP => Base::ESP,
            GPReg32::EBP => Base::ModSpecific,
            GPReg32::ESI => Base::ESI,
            GPReg32::EDI => Base::EDI,
        },
        _ => unimplemented!(),
    }
}

#[cfg(test)]
mod tests {
    use super::{gen_sib, Scale, Index, Base};

    #[test]
    fn test_sib() {
        let result = gen_sib(
            Scale::Scale4,
            Index::None,
            Base::ECX,
        );

        assert_eq!(
            0xA1,
            result
        );
    }
}

