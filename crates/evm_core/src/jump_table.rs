use crate::{Evm, opcodes::Opcode, operations::arithmetics::*};

pub type OpcodeFN = fn(&mut Evm);

fn noop(_evm: &mut Evm) {}

pub fn build_jump_table() -> [OpcodeFN; 256] {
    let mut jump_table = [noop as OpcodeFN; 256];

    jump_table[Opcode::STOP as usize] = stop;
    jump_table[Opcode::ADD as usize] = add;
    jump_table[Opcode::MUL as usize] = mul;
    jump_table[Opcode::SUB as usize] = sub;
    jump_table[Opcode::DIV as usize] = div;
    jump_table[Opcode::SDIV as usize] = sdiv;
    jump_table[Opcode::MOD as usize] = modulo;
    jump_table[Opcode::MULMOD as usize] = mulmod;
    jump_table[Opcode::ADDMOD as usize] = addmod;
    jump_table[Opcode::EXP as usize] = exp;
    jump_table[Opcode::SIGNEXTEND as usize] = signextend;

}
