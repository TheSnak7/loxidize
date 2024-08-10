use std::io::{self, Write};

use crate::vm::VM;

pub fn repl() {
    let mut vm = VM::default();
    loop {
        print!("> ");
        io::stdout().flush().expect("Failed to flush io");

        let mut line = String::new();

        io::stdin()
            .read_line(&mut line)
            .expect("Failed to read line");

        vm.interpret(&line);
    }
}
