use super::register::Register;
use super::memory::Memory;


#[allow(unused)]
#[derive(PartialEq, Eq, Clone, Copy)]
pub enum Operand {
    Register(Register),
    Memory(Memory),
}
