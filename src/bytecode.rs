use std::{pin::Pin, ptr::NonNull};

use num_enum::TryFromPrimitive;

use crate::{lox_value::LoxValue, opcodes::Op};

#[derive(Debug)]
pub struct Ip {
    ptr: NonNull<u8>,
}

impl Ip {
    pub unsafe fn create(code: Pin<&[u8]>) -> Ip {
        let ptr = code.as_ptr();
        assert!(ptr != std::ptr::null());

        Ip {
            ptr: NonNull::new(ptr as *mut u8).unwrap(),
        }
    }
}

impl Ip {
    #[inline(always)]
    pub fn get_op(&self) -> Op {
        let byte = unsafe { *self.ptr.as_ptr() };
        let op = Op::try_from_primitive(byte).unwrap();
        op
    }

    #[inline(always)]
    pub fn get_u8(&self) -> u8 {
        let byte = unsafe { *self.ptr.as_ptr() };
        byte
    }

    #[inline(always)]
    pub fn inc(&mut self, offset: usize) {
        unsafe { self.ptr = self.ptr.add(offset) };
    }
}

#[derive(Debug, Default)]
pub struct Bytecode {
    code: Vec<u8>,
    constants: Vec<LoxValue>,
    lines: Vec<i32>,
    finished_compilation: bool,
}

impl Bytecode {
    pub fn new() -> Bytecode {
        Bytecode {
            code: vec![],
            constants: vec![],
            lines: vec![],
            finished_compilation: false,
        }
    }

    /*
    When this is called compilation should have finished, therefore code should have stopped growing
    With a secure memory location this pointer is safe, as the interpreter is single-threaded
    Furthermore the vector can be considered immutable, the pointer is required only for pointer arithmetic
    */
    pub fn get_base_ip(&self) -> Option<Ip> {
        if !self.finished_compilation {
            return None;
        } else {
            return unsafe { Some(Ip::create(Pin::new(self.code.as_slice()))) };
        }
    }

    pub fn get_code_len(&self) -> usize {
        return self.code.len();
    }

    pub fn write_u8(&mut self, byte: u8, line: i32) {
        self.code.push(byte);
        self.lines.push(line);
    }

    pub fn add_constant(&mut self, value: LoxValue) -> usize {
        self.constants.push(value);
        self.constants.len() - 1
    }

    pub fn get_constant(&self, index: usize) -> LoxValue {
        self.constants.get(index).unwrap().clone()
    }

    pub fn disassemble(&self, name: &str) -> String {
        let mut disassembly = String::with_capacity(20);

        disassembly.push_str(&format!("== {name} ==\n"));

        let mut op_index = 0;
        while op_index < self.code.len() {
            disassembly.push_str(&format!("{op_index:04} "));

            if op_index > 0 && self.lines[op_index] == self.lines[op_index - 1] {
                disassembly.push_str("   | ");
            } else {
                let line = self.lines[op_index];
                disassembly.push_str(&format!("{line: >4} "));
            }

            let op_text = if let Ok(op) = Op::try_from_primitive(self.code[op_index]) {
                let op_string = match op {
                    Op::ConstantSmall => {
                        let constant = self.code[op_index + 1];
                        let value = &self.constants[constant as usize];
                        format!("{op: <16} {constant:04} {value}")
                    }
                    Op::Ret | Op::Negate | Op::Add | Op::Subtract | Op::Multiply | Op::Divide => {
                        format!("{op}")
                    }
                };
                op_index += 1 + op.operand_count();
                op_string
            } else {
                op_index = self.code.len();
                "Illegal Instruction".to_owned()
            };
            disassembly.push_str(&op_text);
            disassembly.push('\n');
        }

        disassembly
    }
}
