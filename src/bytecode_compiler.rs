use crate::{
    ast::{Ast, BinOpKind, ExprKind},
    bytecode::Bytecode,
    opcodes::Op,
};

pub struct BytecodeCompiler<'ast> {
    ast: &'ast Ast,
    bytecode_block: Bytecode,
}

impl<'ast> BytecodeCompiler<'ast> {
    pub fn new(ast: &'ast Ast) -> Self {
        Self {
            ast: ast,
            bytecode_block: Bytecode::default(),
        }
    }

    pub fn compile(mut self) -> Bytecode {
        self.visit_expr(&self.ast.root);
        // While compiling arithmetic expressions, a return must be inserted manually
        self.bytecode_block.write_u8(Op::Ret.into(), 111);
        self.bytecode_block.finished_compilation = true;
        return self.bytecode_block;
    }

    fn visit_expr(&mut self, expr: &'ast crate::ast::Expr) -> () {
        match &expr.kind {
            ExprKind::Binary(op, lhs, rhs) => {
                self.visit_expr(lhs);
                self.visit_expr(rhs);

                match op {
                    BinOpKind::Add => {
                        self.bytecode_block.write_u8(Op::Add.into(), 111);
                    }
                    BinOpKind::Sub => {
                        self.bytecode_block.write_u8(Op::Subtract.into(), 111);
                    }
                }
            }
            ExprKind::Lit(lit) => {
                let constant = self.bytecode_block.add_constant(lit.symbol.into());
                assert!(constant < u8::MAX as usize);
                // FIXME: Line stubbed for now
                self.bytecode_block.write_u8(Op::ConstantSmall.into(), 111);
                self.bytecode_block.write_u8(constant as u8, 111);
            }
        }
    }
}
