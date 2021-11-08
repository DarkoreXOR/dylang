use super::{immediate::Immediate, register::Register};

#[allow(unused)]
#[repr(u8)]
#[derive(PartialEq, Eq, Clone, Copy)]
pub enum Scale {
    X2,
    X4,
    X8,
}

#[allow(unused)]
#[derive(PartialEq, Eq, Clone, Copy)]
pub enum Memory {
    Register(Register), // [eax]
    Immediate(Immediate), // [0x00000000]
    RegisterDisplacement(Register, Immediate), // [eax + 0x00000000]
    BaseIndex(Register, Register), // [eax + ebx]
    IndexScale(Register, Scale), // [eax * 2]
    BaseIndexScale(Register, Register, Scale), // [eax + ebx * 4]
    BaseIndexScaleDisplacement(Register, Register, Scale, Immediate) // [eax + ebx * 8 + 0x12345678]
}
