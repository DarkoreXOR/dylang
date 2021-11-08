use crate::x86::{instruction_table, operand::Operand, utils};

pub struct Codegen {
    buffer: Vec<u8>
}

impl Codegen {
    pub fn new() -> Self {
        Self {
            buffer: Vec::new(),
        }
    }

    pub fn get_bytes(&self) -> &[u8] {
        &self.buffer
    }

/*     pub fn emit_byte(&mut self, byte: u8) {
        self.buffer.push(byte);
    } */

    pub fn emit_bytes(&mut self, bytes: &[u8]) {
        self.buffer.extend_from_slice(bytes);
    }

    pub fn mov(&mut self, operand1: Operand, operand2: Operand) {
        let candidate = instruction_table::query_instruction(
            instruction_table::Kind::Mov,
            Some(instruction_table::OpType::ModRm16_32),
            Some(instruction_table::OpType::Reg16_32)
        );

        if let Some(instr_info) = candidate {
            self.emit_bytes(instr_info.op);
            utils::emit_modrm_byte(operand1, operand2, &mut self.buffer);
        } else {
            panic!("invalid instruction format")
        }
    }
}
