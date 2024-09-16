use crate::{
    bytecode::{Bytecode, Ip},
    compiler::Compiler,
    lox_value::LoxValue,
    opcodes::Op,
    stack::{Sp, Stack},
};

#[derive(Debug, Clone, Copy)]
pub enum Error {
    Compile,
    Runtime,
}

pub const STACK_SIZE: usize = 10;

#[derive(Debug)]
pub struct VM {
    ip: Option<Ip>,
    sp: Option<Sp<STACK_SIZE>>,
    // FIXME: Make more rusty
    bytecode: Option<Bytecode>,
    stack: Stack<STACK_SIZE>,
}

impl VM {
    pub fn interpret(&mut self, code: &str) -> Result<(), Error> {
        let compiler = Compiler::default();
        let bytecode = compiler.compile(code);

        self.bytecode = Some(bytecode);

        let bc = self.bytecode.as_ref().unwrap();
        self.ip = Some(bc.get_base_ip().unwrap());

        self.sp = Some(self.stack.get_base_sp());

        loop {
            let inst = self.ip.as_mut().unwrap().get_op();
            self.ip.as_mut().unwrap().inc(1);

            if cfg!(feature = "vm-trace-execution") {
                println!("          ");
                for slot in self
                    .stack
                    .get_stack_iterator(self.sp.as_mut().unwrap().clone())
                {
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
        let byte = self.ip.as_ref().unwrap().get_u8();
        self.ip.as_mut().unwrap().inc(1);
        byte
    }

    fn read_constant(&mut self) -> LoxValue {
        let index = self.read_u8() as usize;
        self.bytecode.as_ref().unwrap().get_constant(index).clone()
    }

    fn push(&mut self, value: &LoxValue) {
        self.sp.as_mut().unwrap().write_value(value);
        self.sp.as_mut().unwrap().inc(1);
    }

    fn pop(&mut self) -> LoxValue {
        self.sp.as_mut().unwrap().dec(1);
        let val = self.sp.as_mut().unwrap().get_value();
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

impl Default for VM {
    fn default() -> Self {
        Self {
            ip: None,
            sp: None,
            bytecode: None,
            stack: Stack::new(),
        }
    }
}
