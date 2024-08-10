use std::pin::Pin;

use crate::{
    bytecode::{Bytecode, Ip},
    lox_value::LoxValue,
    opcodes::Op,
    stack::{Sp, Stack},
    states::{Initialized, State, Uninitialized},
};

#[derive(Debug)]
pub enum Error {
    Compile,
    Runtime,
}

pub const STACK_SIZE: usize = 10;

#[derive(Debug)]
pub struct VM<S: State> {
    ip: Ip<S>,
    sp: Sp<S, STACK_SIZE>,
    // FIXME: Make more rusty
    bytecode: Option<Bytecode>,
    stack: Stack<STACK_SIZE>,
}

impl VM<Initialized> {
    // Change structure later
    // Signature hack for now
    // Correct:     pub fn interpret(&mut self, bytecode: Bytecode) -> Result<(), Error> {
    pub fn interpret(&mut self) -> Result<(), Error> {
        self.ip = self.bytecode.as_ref().unwrap().get_base_ip();
        //self.bytecode = Some(bytecode);

        loop {
            let inst = self.ip.get_op();
            self.ip.inc(1);

            if cfg!(feature = "vm-trace-execution") {
                println!("          ");
                for slot in self.stack.get_stack_iterator(self.sp.clone()) {
                    print!("[ {slot} ]");
                }
                println!();

                println!("{inst}");
            }
            match inst {
                Op::ConstantSmall => self.op_constant_small(),
                Op::Add => self.op_add(),
                Op::Subtract => self.op_subtract(),
                Op::Multiply => self.op_multiply(),
                Op::Divide => self.op_divide(),
                Op::Negate => self.op_negate(),
                Op::Ret => {
                    let val = self.pop();
                    println!("{val}");
                    return Ok(());
                }
            }
        }
    }

    fn read_u8(&mut self) -> u8 {
        let byte = self.ip.get_u8();
        self.ip.inc(1);
        byte
    }

    fn read_constant(&mut self) -> LoxValue {
        let index = self.read_u8() as usize;
        self.bytecode.as_ref().unwrap().get_constant(index).clone()
    }

    fn push(&mut self, value: &LoxValue) {
        self.sp.write_value(value);
        self.sp.inc(1);
    }

    fn pop(&mut self) -> LoxValue {
        self.sp.dec(1);
        let val = self.sp.get_value();
        val
    }

    fn op_constant_small(&mut self) {
        let constant = self.read_constant();
        self.push(&constant);
    }

    fn op_add(&mut self) {
        let b = self.pop();
        let a = self.pop();
        match (a, b) {
            (LoxValue::Number(a), LoxValue::Number(b)) => self.push(&LoxValue::Number(a + b)),
            _ => panic!("Operands must be numbers"),
        }
    }

    fn op_subtract(&mut self) {
        let b = self.pop();
        let a = self.pop();
        match (a, b) {
            (LoxValue::Number(a), LoxValue::Number(b)) => self.push(&LoxValue::Number(a - b)),
            _ => panic!("Operands must be numbers"),
        }
    }

    fn op_multiply(&mut self) {
        let b = self.pop();
        let a = self.pop();
        match (a, b) {
            (LoxValue::Number(a), LoxValue::Number(b)) => self.push(&LoxValue::Number(a * b)),
            _ => panic!("Operands must be numbers"),
        }
    }

    fn op_divide(&mut self) {
        let b = self.pop();
        let a = self.pop();
        match (a, b) {
            (LoxValue::Number(a), LoxValue::Number(b)) => self.push(&LoxValue::Number(a / b)),
            _ => panic!("Operands must be numbers"),
        }
    }

    fn op_negate(&mut self) {
        let val = self.pop();
        match val {
            LoxValue::Number(num) => self.push(&LoxValue::Number(-num)),
        }
    }
}

impl VM<Uninitialized> {
    // Hack for now
    pub fn init(mut self, bytecode: Bytecode) -> VM<Initialized> {
        VM {
            ip: bytecode.get_base_ip(),
            sp: self.stack.get_base_sp(),
            bytecode: Some(bytecode),
            stack: self.stack,
        }
    }
}

impl Default for VM<Uninitialized> {
    fn default() -> Self {
        Self {
            ip: Ip::<Uninitialized>::create_uninitialized(),
            sp: Sp::<Uninitialized, STACK_SIZE>::create_uninitialized(),
            bytecode: None,
            stack: Stack::new(),
        }
    }
}
