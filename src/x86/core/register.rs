
#[allow(unused)]
#[derive(PartialEq, Eq, Clone, Copy)]
pub enum Register {
    GPR8(GPReg8),
    GPR16(GPReg16),
    GPR32(GPReg32),
    SegR(Segment),
}

#[allow(unused)]
#[repr(u8)]
#[derive(PartialEq, Eq, Clone, Copy)]
pub enum GPReg8 {
    AL = 0,
    CL,
    DL,
    BL,
    AH,
    CH,
    DH,
    BH,
}

#[allow(unused)]
#[repr(u8)]
#[derive(PartialEq, Eq, Clone, Copy)]
pub enum GPReg16 {
    AX = 0,
    CX,
    DX,
    BX,
    SP,
    BP,
    SI,
    DI,
}

#[allow(unused)]
#[repr(u8)]
#[derive(PartialEq, Eq, Clone, Copy)]
pub enum GPReg32 {
    EAX = 0,
    ECX,
    EDX,
    EBX,
    ESP,
    EBP,
    ESI,
    EDI,
}

#[allow(unused)]
#[repr(u8)]
#[derive(PartialEq, Eq, Clone, Copy)]
pub enum Segment {
    ES = 0,
    CS,
    SS,
    DS,
    FS,
    GS,
}

/*
pub struct Register {
    value: u8,
}

impl Register {
    #[inline]
    const fn new(value: u8) -> Self {
        Self { value }
    }
}

// 8-bit registers
#[allow(unused)]
impl Register {
    const AL: Self = Self::new(0b000);
    const CL: Self = Self::new(0b001);
    const DL: Self = Self::new(0b010);
    const BL: Self = Self::new(0b011);
    const AH: Self = Self::new(0b100);
    const CH: Self = Self::new(0b101);
    const DH: Self = Self::new(0b110);
    const BH: Self = Self::new(0b111);
}

// 16-bit registers
#[allow(unused)]
impl Register {
    const AX: Self = Self::new(0b000);
    const CX: Self = Self::new(0b001);
    const DX: Self = Self::new(0b010);
    const BX: Self = Self::new(0b011);
    const SP: Self = Self::new(0b100);
    const BP: Self = Self::new(0b101);
    const SI: Self = Self::new(0b110);
    const DI: Self = Self::new(0b111);
}

// 32-bit registers
#[allow(unused)]
impl Register {
    const EAX: Self = Self::new(0b000);
    const ECX: Self = Self::new(0b001);
    const EDX: Self = Self::new(0b010);
    const EBX: Self = Self::new(0b011);
    const ESP: Self = Self::new(0b100);
    const EBP: Self = Self::new(0b101);
    const ESI: Self = Self::new(0b110);
    const EDI: Self = Self::new(0b111);
}
*/