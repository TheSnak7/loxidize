use bytecode::Bytecode;
use opcodes::Op;

mod bytecode;
mod lox_value;
mod opcodes;

fn main() {
    let mut bc = Bytecode::new();
    let constant = bc.add_constant(1.2.into());
    assert!(constant < u8::MAX as usize);
    bc.write_u8(Op::ConstantSmall.into(), 123);
    bc.write_u8(constant as u8, 123);
    bc.write_u8(Op::Ret.into(), 123);
    println!("{}", &bc.disassemble("test chunk"));
}
