use std::hashmap;
pub use vm::instruction::*;

pub struct Engine {
  instructions: ~[Instruction],
  environment: ~Environment
}

struct Environment {
  frames: ~[Frame]
}

impl Environment {
  pub fn new() -> ~Environment {
    ~Environment {
      frames: ~[]}
  }

}

struct Frame {
  symbols: hashmap::HashMap<~str, Value>
}

pub struct Value {
  val: bool
}

impl Engine {
  pub fn new() -> ~Engine {
    ~Engine {
      instructions: ~[],
      environment: Environment::new()}
  }

  fn step(&self, environment: &Environment, instruction: &Instruction) {
    match instruction {
      &Instruction {opcode: NoOp}   => perform_no_op(),
      &Instruction {opcode: Def}    => perform_def(),
      &Instruction {opcode: Return} => perform_return(),
      &Instruction {opcode: Call}   => perform_call(),
      &Instruction {opcode: _}      => ()
    }
  }

  pub fn run(&mut self) {
    for instruction in self.instructions.iter() {
      self.step(self.environment, instruction);
    }
  }
}
