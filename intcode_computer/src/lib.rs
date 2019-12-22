pub mod instruction;
pub mod operations;
pub mod parameter;
pub mod pipe;

use instruction::*;
use operations::*;
use parameter::*;
use std::cell::{Cell, RefCell};
use std::collections::{HashMap, VecDeque};

pub trait Computer<MemoryType> {
    /// returns false if the program has halted, if the value is true, there may be an interrupt
    fn execute(&self) -> bool;
    fn step(&self) -> bool;
}

pub type IntcodeMemoryCellType = i64;
pub type IntcodeMemoryType = Vec<i64>;
type InternalMemoryType = HashMap<usize, i64>;

pub struct IntCodeComputer<'a> {
    memory: RefCell<InternalMemoryType>,
    instruction_ptr: Cell<usize>,
    input_buffer: RefCell<VecDeque<IntcodeMemoryCellType>>,
    output_callback: &'a dyn Fn(IntcodeMemoryCellType),
    interupted: Cell<bool>,
    relative_base: Cell<IntcodeMemoryCellType>,
}

impl<'a> IntCodeComputer<'a> {
    pub fn new(
        memory: Vec<IntcodeMemoryCellType>,
        output: &'a dyn Fn(IntcodeMemoryCellType),
    ) -> IntCodeComputer<'a> {
        IntCodeComputer {
            memory: RefCell::new(memory.into_iter().enumerate().collect()),
            instruction_ptr: Cell::new(0),
            input_buffer: RefCell::new(VecDeque::new()),
            output_callback: output,
            interupted: Cell::new(false),
            relative_base: Cell::new(0),
        }
    }

    pub fn provide_input(&self, input: IntcodeMemoryCellType) {
        self.input_buffer.borrow_mut().push_back(input);
    }

    pub fn terminate(self) -> IntcodeMemoryType {
        let mut sorted_by_address: Vec<_> = self.memory.into_inner().drain().collect();
        sorted_by_address.sort_unstable_by_key(|(k, _)| *k);
        sorted_by_address.drain(..).map(|(_, v)| v).collect()
    }
}

impl<'a> Computer<IntcodeMemoryCellType> for IntCodeComputer<'a> {
    fn execute(&self) -> bool {
        let memory_len = self.memory.borrow().len();
        while self.instruction_ptr.get() < memory_len {
            self.step();
            if self.interupted.get() {
                return true;
            }
        }
        return false;
    }

    fn step(&self) -> bool {
        let instruction = {
            let memory = self.memory.borrow();
            Instruction::read(&memory, &self.instruction_ptr.get())
        };
        self.execute_instruction(instruction)
    }
}

impl<'a> IntCodeComputer<'a> {
    /// returns true if halted, false otherwise
    fn execute_instruction(&self, instruction: Instruction) -> bool {
        let mut memory = self.memory.borrow_mut();
        let relative_base = self.relative_base.get();
        match instruction.operation {
            Operation::Add => {
                arithmetic_operation(
                    &instruction,
                    &mut memory,
                    &relative_base,
                    Box::new(|x, y| x + y),
                );
            }
            Operation::Multiply => {
                arithmetic_operation(
                    &instruction,
                    &mut memory,
                    &relative_base,
                    Box::new(|x, y| x * y),
                );
            }
            Operation::Input => {
                if self.interupted.get() || !self.input_buffer.borrow().is_empty() {
                    self.interupted.replace(false);
                    let input_result = (self.input_buffer.borrow_mut().pop_front())
                        .expect("input buffer empty after interrupt");
                    let storage_index = resolve_pointer(instruction.parameters[0], &relative_base);
                    memory.insert(storage_index, input_result);
                } else {
                    self.interupted.replace(true);
                    return false;
                }
            }
            Operation::Output => {
                let value =
                    resolve_value_in_memory(instruction.parameters[0], &memory, &relative_base);
                (self.output_callback)(value);
            }
            Operation::JumpIfTrue => {
                if resolve_value_in_memory(instruction.parameters[0], &memory, &relative_base) != 0
                {
                    let jump_address =
                        resolve_value_in_memory(instruction.parameters[1], &memory, &relative_base);
                    self.instruction_ptr.replace(jump_address as usize);
                    return false;
                }
            }
            Operation::JumpIfFalse => {
                if resolve_value_in_memory(instruction.parameters[0], &memory, &relative_base) == 0
                {
                    let jump_address =
                        resolve_value_in_memory(instruction.parameters[1], &memory, &relative_base);
                    self.instruction_ptr.replace(jump_address as usize);
                    return false;
                }
            }
            Operation::LessThan => {
                arithmetic_operation(
                    &instruction,
                    &mut memory,
                    &relative_base,
                    Box::new(|x, y| if x < y { 1 } else { 0 }),
                );
            }
            Operation::Equals => {
                arithmetic_operation(
                    &instruction,
                    &mut memory,
                    &relative_base,
                    Box::new(|x, y| if x == y { 1 } else { 0 }),
                );
            }
            Operation::AdjustRelativeBase => {
                let delta_base =
                    resolve_value_in_memory(instruction.parameters[0], &memory, &relative_base);
                self.relative_base.set(delta_base + relative_base);
            }
            Operation::Halt => {
                //TODO: use set instead of replace everywhere
                self.instruction_ptr.replace(memory.len());
                return true;
            }
        }
        let old_ptr = self.instruction_ptr.get();
        let new_ptr =
            instruction.operation.parameter_count() + 1 + (old_ptr as IntcodeMemoryCellType);
        self.instruction_ptr.replace(new_ptr as usize);
        false
    }
}

fn arithmetic_operation(
    instruction: &Instruction,
    memory: &mut InternalMemoryType,
    relative_base: &IntcodeMemoryCellType,
    transform: Box<
        dyn FnOnce(IntcodeMemoryCellType, IntcodeMemoryCellType) -> IntcodeMemoryCellType,
    >,
) {
    let storage_index = resolve_pointer(instruction.parameters[2], relative_base);
    let operand1 = resolve_value_in_memory(instruction.parameters[0], &memory, relative_base);
    let operand2 = resolve_value_in_memory(instruction.parameters[1], &memory, relative_base);
    memory.insert(storage_index, transform(operand1, operand2));
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
