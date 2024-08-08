use bytecode::Bytecode;
use opcodes::Op;

mod bytecode;
mod opcodes;

fn main() {
    let mut bc = Bytecode::new();
    bc.write_op(Op::Ret);
    println!("{}", &bc.disassemble("test chunk"))
}
