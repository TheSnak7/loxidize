use loxidize::bytecode::Bytecode;
use loxidize::opcodes::Op;
use loxidize::vm::VM;

fn main() {
    let mut bc = Bytecode::new();
    let constant = bc.add_constant(1.2.into());
    assert!(constant < u8::MAX as usize);
    bc.write_u8(Op::ConstantSmall.into(), 123);
    bc.write_u8(constant as u8, 123);
    bc.write_u8(Op::Negate.into(), 123);
    bc.write_u8(Op::Ret.into(), 123);
    println!("{}", &bc.disassemble("test chunk"));
    let vm = VM::default();
    let mut vm = vm.init(bc);
    vm.interpret().expect("Did not expect to error");
}
