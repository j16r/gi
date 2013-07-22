pub struct Instruction {
  opcode : OpCode
}

pub enum OpCode {
  NoOp,
  Def,
  Return,
  Call,
  Require
}

pub fn perform_no_op() {
}

pub fn perform_def() {
}

pub fn perform_return() {
}

pub fn perform_call() {
}
