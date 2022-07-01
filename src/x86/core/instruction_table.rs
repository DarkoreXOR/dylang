
#[allow(unused)]
#[derive(PartialEq, Eq, Clone, Copy)]
pub enum Kind {
    Mov,
}

#[allow(unused)]
#[derive(PartialEq, Eq, Clone, Copy)]
pub enum OpType {
    NoOperand,
    ModRm8,
    ModRm16_32,
    Reg8,
    Reg16_32,
}

#[allow(unused)]
#[derive(PartialEq, Eq, Clone, Copy)]
pub struct InstrInfo {
    pub op: &'static [u8],
    kind: Kind,
    op_type1: OpType,
    op_type2: OpType,
}

impl InstrInfo {
    const fn new(
        op: &'static [u8],
        kind: Kind,
        op_type1: OpType,
        op_type2: OpType,
    ) -> Self {
        Self {
            op,
            kind,
            op_type1,
            op_type2
        }
    }
}

static INSTRUCTION_TABLE: &[InstrInfo] = &[
    InstrInfo::new(&[0x89], Kind::Mov, OpType::ModRm16_32, OpType::Reg16_32),
    InstrInfo::new(&[0x8B], Kind::Mov, OpType::Reg16_32, OpType::ModRm16_32),
];

pub fn query_instruction(
    kind: Kind,
    op_type1: Option<OpType>,
    op_type2: Option<OpType>
) -> Option<InstrInfo> {
    let instructions = INSTRUCTION_TABLE
        .iter()
        .filter(Box::new(move |ii: &&InstrInfo| ii.kind == kind) as Box<dyn Fn(&&InstrInfo) -> bool>);
    
    let instructions = if let Some(op_type) = op_type1 {
        instructions.filter(Box::new(move |ii: &&InstrInfo| ii.op_type1 == op_type) as Box<dyn Fn(&&InstrInfo) -> bool>)
    } else {
        instructions.filter(Box::new(move |_: &&InstrInfo| true) as Box<dyn Fn(&&InstrInfo) -> bool>)
    };

    let mut instructions = if let Some(op_type) = op_type2 {
        instructions.filter(Box::new(move |ii: &&InstrInfo| ii.op_type2 == op_type) as Box<dyn Fn(&&InstrInfo) -> bool>)
    } else {
        instructions.filter(Box::new(move |_: &&InstrInfo| true) as Box<dyn Fn(&&InstrInfo) -> bool>)
    };
    
    if let Some(result) = instructions.next() {
        return Some(result.clone());
    }

    None
}
