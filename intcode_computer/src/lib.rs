pub mod instruction;
pub mod operations;
pub mod parameter;
pub mod pipe;
pub mod prelude;
mod sugar;

use instruction::*;
use operations::*;
use parameter::*;
use std::collections::{HashMap, VecDeque};

pub trait Computer<MemoryType> {
    fn execute(&mut self) -> Interrupt;
    fn step(&mut self);
}

pub type IntcodeMemoryCellType = i64;
pub type IntcodeMemoryType = Vec<i64>;
type InternalMemoryType = HashMap<usize, i64>;

pub struct IntCodeComputer {
    memory: InternalMemoryType,
    instruction_ptr: usize,
    input_buffer: VecDeque<IntcodeMemoryCellType>,
    output_buffer: VecDeque<IntcodeMemoryCellType>,
    interrupted: Option<Interrupt>,
    relative_base: IntcodeMemoryCellType,
}

impl IntCodeComputer {
    pub fn new(memory: Vec<IntcodeMemoryCellType>) -> IntCodeComputer {
        IntCodeComputer {
            memory: memory.into_iter().enumerate().collect(),
            instruction_ptr: 0,
            input_buffer: VecDeque::new(),
            output_buffer: VecDeque::new(),
            interrupted: None,
            relative_base: 0,
        }
    }

    pub fn provide_input(&mut self, input: IntcodeMemoryCellType) {
        self.input_buffer.push_back(input);
    }

    pub fn take_output(&mut self) -> IntcodeMemoryCellType {
        self.output_buffer.pop_front().unwrap()
    }

    pub fn terminate(mut self) -> IntcodeMemoryType {
        // TODO: insert blanks
        let mut sorted_by_address: Vec<_> = self.memory.drain().collect();
        sorted_by_address.sort_unstable_by_key(|(k, _)| *k);
        sorted_by_address.drain(..).map(|(_, v)| v).collect()
    }
}

impl<'a> Computer<IntcodeMemoryCellType> for IntCodeComputer {
    fn execute(&mut self) -> Interrupt {
        let memory_len = self.memory.len();
        while self.instruction_ptr < memory_len {
            self.step();
            if let Some(interrupt) = self.interrupted {
                return interrupt;
            }
        }
        return Interrupt::Halt;
    }

    fn step(&mut self) {
        let instruction = Instruction::read(&self.memory, &self.instruction_ptr);
        self.execute_instruction(instruction);
    }
}

macro_rules! arith {
    ($transform:expr, $instruction:expr, $computer:ident) => {
        let storage_index = resolve_pointer($instruction.parameters[2], &$computer.relative_base);
        let operand1 = resolve_value_in_memory(
            $instruction.parameters[0],
            &$computer.memory,
            &$computer.relative_base,
        );
        let operand2 = resolve_value_in_memory(
            $instruction.parameters[1],
            &$computer.memory,
            &$computer.relative_base,
        );
        $computer
            .memory
            .insert(storage_index, $transform(operand1, operand2));
    };
}

impl IntCodeComputer {
    fn execute_instruction(&mut self, instruction: Instruction) {
        match instruction.operation {
            Operation::Add => {
                arith!(|x, y| x + y, instruction, self);
            }
            Operation::Multiply => {
                arith!(|x, y| x * y, instruction, self);
            }
            Operation::Input => {
                if self.interrupted == Some(Interrupt::Input) || !self.input_buffer.is_empty() {
                    self.interrupted = None; // bug?
                    let input_result = self
                        .input_buffer
                        .pop_front()
                        .expect("input buffer empty after interrupt");
                    let storage_index =
                        resolve_pointer(instruction.parameters[0], &self.relative_base);
                    self.memory.insert(storage_index, input_result);
                } else {
                    self.interrupted = Some(Interrupt::Input);
                    return;
                }
            }
            Operation::Output => {
                if Some(Interrupt::Output) == self.interrupted {
                    self.interrupted = None;
                } else {
                    let value = resolve_value_in_memory(
                        instruction.parameters[0],
                        &self.memory,
                        &self.relative_base,
                    );
                    self.output_buffer.push_back(value);
                    self.interrupted = Some(Interrupt::Output);
                    return;
                }
            }
            Operation::JumpIfTrue => {
                if resolve_value_in_memory(
                    instruction.parameters[0],
                    &self.memory,
                    &self.relative_base,
                ) != 0
                {
                    let jump_address = resolve_value_in_memory(
                        instruction.parameters[1],
                        &self.memory,
                        &self.relative_base,
                    );
                    self.instruction_ptr = jump_address as usize;
                    return;
                }
            }
            Operation::JumpIfFalse => {
                if resolve_value_in_memory(
                    instruction.parameters[0],
                    &self.memory,
                    &self.relative_base,
                ) == 0
                {
                    let jump_address = resolve_value_in_memory(
                        instruction.parameters[1],
                        &self.memory,
                        &self.relative_base,
                    );
                    self.instruction_ptr = jump_address as usize;
                    return;
                }
            }
            Operation::LessThan => {
                arith!(|x, y| if x < y { 1 } else { 0 }, instruction, self);
            }
            Operation::Equals => {
                arith!(|x, y| if x == y { 1 } else { 0 }, instruction, self);
            }
            Operation::AdjustRelativeBase => {
                let delta_base = resolve_value_in_memory(
                    instruction.parameters[0],
                    &self.memory,
                    &self.relative_base,
                );
                self.relative_base += delta_base;
            }
            Operation::Halt => {
                self.interrupted = Some(Interrupt::Halt);
                return;
            }
        }
        self.advance_instruction_pointer(&instruction);
    }

    fn advance_instruction_pointer(&mut self, instruction: &Instruction) {
        self.instruction_ptr += 1 + instruction.operation.parameter_count();
    }
}

fn resolve_value_in_memory(
    parameter: Parameter,
    memory: &InternalMemoryType,
    relative_base: &IntcodeMemoryCellType,
) -> IntcodeMemoryCellType {
    match parameter {
        Parameter::Value(value) => value,
        pointer => memory
            .get(&resolve_pointer(pointer, relative_base))
            .copied()
            .unwrap_or_default(),
    }
}

fn resolve_pointer(parameter: Parameter, relative_base: &IntcodeMemoryCellType) -> usize {
    match parameter {
        Parameter::Value(_) => panic!("attempting to access value as pointer"),
        Parameter::Pointer(index) => index,
        Parameter::Relative(offset) => (offset + relative_base) as usize,
    }
}

impl Instruction {
    fn read(memory: &InternalMemoryType, instruction_ptr: &usize) -> Instruction {
        let opcode = memory[instruction_ptr];
        let OpCode {
            operation,
            parameter_modes,
        } = OpCode::from(opcode);
        let mut parameters: Vec<Parameter> = Vec::new();
        for i in 0..operation.parameter_count() {
            let mode = parameter_modes[i as usize];
            let address = *instruction_ptr + i as usize + 1;
            let value = memory[&address];
            let parameter = match mode {
                ParameterMode::Pointer => Parameter::Pointer(value as usize),
                ParameterMode::Value => Parameter::Value(value),
                ParameterMode::Relative => Parameter::Relative(value),
            };
            parameters.push(parameter);
        }
        Instruction {
            operation: operation,
            parameters: parameters,
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Interrupt {
    Input,
    Output,
    Halt,
}

impl Interrupt {
    pub fn is_halted(&self) -> bool {
        match self {
            Self::Halt => true,
            _ => false,
        }
    }
}
