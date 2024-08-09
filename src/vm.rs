use crate::{
    bytecode::{Bytecode, Ip},
    lox_value::LoxValue,
    opcodes::Op,
    stack::{Sp, Stack},
    states::{Initialized, Uninitialized},
};

pub enum Error {
    Compile,
    Runtime,
}

pub const STACK_SIZE: usize = 256;

pub struct VM<S> {
    ip: Ip<S>,
    sp: Sp<S, STACK_SIZE>,
    // FIXME: Make more rusty
    bytecode: *const Bytecode,
    stack: Stack<STACK_SIZE>,
}

impl VM<Initialized> {
    // Change structure later
    fn interpret(&mut self, bytecode: &Bytecode) -> Result<(), Error> {
        self.ip = bytecode.get_base_ip();
        self.bytecode = bytecode as *const Bytecode;

        loop {
            let inst = self.ip.get_op();

            if cfg!(feature = "vm-trace-execution") {
                println!("{inst}");
            }

            match inst {
                Op::Ret => return Ok(()),
                Op::ConstantSmall => self.op_constant_small(),
                _ => panic!("Unexpected inst"),
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
        unsafe { (*self.bytecode).get_constant(index).clone() }
    }

    fn push(&mut self, value: &LoxValue) {
        self.sp.write_value(value);
        self.sp.inc(1);
    }

    fn op_constant_small(&mut self) {
        let constant = self.read_constant();
        println!("{constant}")
    }
}

impl Default for VM<Uninitialized> {
    fn default() -> Self {
        Self {
            ip: Ip::<Uninitialized>::create_uninitialized(),
            sp: Sp::<Uninitialized, STACK_SIZE>::create_uninitialized(),
            bytecode: std::ptr::null(),
            stack: Stack::new(),
        }
    }
}
