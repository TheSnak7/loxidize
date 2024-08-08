use num_enum::TryFromPrimitive;

use crate::opcodes::Op;

#[derive(Default)]
pub struct Bytecode {
    bytes: Vec<u8>,
}

impl Bytecode {
    pub fn new() -> Bytecode {
        Bytecode { bytes: vec![] }
    }

    pub fn disassemble(&self, name: &str) -> String {
        let mut disassembly = String::with_capacity(20);

        disassembly.push_str(&format!("== {} ==\n", name));

        let mut op_index = 0;
        while op_index < self.bytes.len() {
            let opcode = match Op::try_from_primitive(self.bytes[0]) {
                Ok(op) => {
                    let op_string = format!("{:04} {}", op_index, op);
                    op_index += 1;
                    op_string
                }
                Err(_) => {
                    op_index = self.bytes.len();
                    format!("{:04} {} ", op_index, "Illegal Instruction")
                }
            };
            disassembly.push_str(&opcode);
        }

        disassembly
    }

    pub fn write_op(&mut self, op: Op) {
        self.bytes.push(op as u8)
    }
}
