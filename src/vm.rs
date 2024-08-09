use crate::{
    bytecode::{Bytecode, Ip},
    lox_value::LoxValue,
    opcodes::Op,
};

pub enum Error {
    Compile,
    Runtime,
}

pub struct VM {
    ip: Ip,
    // FIXME: Make more rusty
    bytecode: *const Bytecode,
}

impl VM {
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

    fn read_u8(&self) -> u8 {
        self.ip.get_u8()
    }

    fn read_constant(&self) -> LoxValue {
        let index = self.read_u8() as usize;
        unsafe { (*self.bytecode).get_constant(index).clone() }
    }

    fn op_constant_small(&mut self) {
        let constant = self.read_constant();
        println!("{constant}")
    }
}

impl Default for VM {
    fn default() -> Self {
        Self {
            ip: Ip::default(),
            bytecode: std::ptr::null(),
        }
    }
}
