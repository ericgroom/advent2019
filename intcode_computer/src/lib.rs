pub mod instruction;
pub mod operations;
pub mod parameter;
pub mod pipe;

use instruction::*;
use operations::*;
use parameter::*;
use std::cell::{Cell, RefCell};
use std::collections::VecDeque;

pub trait Computer<MemoryType> {
    /// returns false if the program has halted, if the value is true, there may be an interrupt
    fn execute(&self) -> bool;
    fn step(&self) -> bool;
}

pub type IntcodeMemoryCellType = i64;
pub type IntcodeMemoryType = Vec<i64>;

pub struct IntCodeComputer<'a> {
    pub memory: RefCell<IntcodeMemoryType>,
    instruction_ptr: Cell<usize>,
    input_buffer: RefCell<VecDeque<IntcodeMemoryCellType>>,
    output_callback: &'a dyn Fn(IntcodeMemoryCellType),
    interupted: Cell<bool>,
}

impl<'a> IntCodeComputer<'a> {
    pub fn new(
        memory: IntcodeMemoryType,
        output: &'a dyn Fn(IntcodeMemoryCellType),
    ) -> IntCodeComputer<'a> {
        IntCodeComputer {
            memory: RefCell::new(memory),
            instruction_ptr: Cell::new(0),
            input_buffer: RefCell::new(VecDeque::new()),
            output_callback: output,
            interupted: Cell::new(false),
        }
    }

    pub fn provide_input(&self, input: IntcodeMemoryCellType) {
        self.input_buffer.borrow_mut().push_back(input);
    }

    pub fn terminate(self) -> IntcodeMemoryType {
        self.memory.into_inner()
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
        match instruction.operation {
            Operation::Add => {
                arithmetic_operation(&instruction, &mut memory, Box::new(|x, y| x + y));
            }
            Operation::Multiply => {
                arithmetic_operation(&instruction, &mut memory, Box::new(|x, y| x * y));
            }
            Operation::Input => {
                if self.interupted.get() || !self.input_buffer.borrow().is_empty() {
                    self.interupted.replace(false);
                    let input_result = (self.input_buffer.borrow_mut().pop_front())
                        .expect("input buffer empty after interrupt");
                    if let Parameter::Pointer(storage_index) = instruction.parameters[0] {
                        memory[storage_index] = input_result;
                    } else {
                        panic!("attempting to store input result to value")
                    }
                } else {
                    self.interupted.replace(true);
                    return false;
                }
            }
            Operation::Output => {
                let value = resolve_value_in_memory(instruction.parameters[0], &memory);
                (self.output_callback)(value);
            }
            Operation::JumpIfTrue => {
                if resolve_value_in_memory(instruction.parameters[0], &memory) != 0 {
                    let jump_address = resolve_value_in_memory(instruction.parameters[1], &memory);
                    self.instruction_ptr.replace(jump_address as usize);
                    return false;
                }
            }
            Operation::JumpIfFalse => {
                if resolve_value_in_memory(instruction.parameters[0], &memory) == 0 {
                    let jump_address = resolve_value_in_memory(instruction.parameters[1], &memory);
                    self.instruction_ptr.replace(jump_address as usize);
                    return false;
                }
            }
            Operation::LessThan => {
                arithmetic_operation(
                    &instruction,
                    &mut memory,
                    Box::new(|x, y| if x < y { 1 } else { 0 }),
                );
            }
            Operation::Equals => {
                arithmetic_operation(
                    &instruction,
                    &mut memory,
                    Box::new(|x, y| if x == y { 1 } else { 0 }),
                );
            }
            Operation::Halt => {
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
    memory: &mut IntcodeMemoryType,
    transform: Box<
        dyn FnOnce(IntcodeMemoryCellType, IntcodeMemoryCellType) -> IntcodeMemoryCellType,
    >,
) {
    if let Parameter::Pointer(storage_index) = instruction.parameters[2] {
        let operand1 = resolve_value_in_memory(instruction.parameters[0], &memory);
        let operand2 = resolve_value_in_memory(instruction.parameters[1], &memory);
        memory[storage_index] = transform(operand1, operand2);
    } else {
        panic!("attempting to store arithmetic operation result to value");
    }
}

fn resolve_value_in_memory(
    parameter: Parameter,
    memory: &IntcodeMemoryType,
) -> IntcodeMemoryCellType {
    match parameter {
        Parameter::Value(value) => value,
        Parameter::Pointer(index) => memory[index],
    }
}

impl Instruction {
    fn read(memory: &IntcodeMemoryType, instruction_ptr: &usize) -> Instruction {
        let mut iter = memory[*instruction_ptr..].iter();
        let opcode = iter.next().expect("Cannot read from empty memory");
        let OpCode {
            operation,
            parameter_modes,
        } = OpCode::from(*opcode);
        let mut parameters: Vec<Parameter> = Vec::new();
        for i in 0..operation.parameter_count() {
            let mode = parameter_modes[i as usize];
            let value = iter
                .next()
                .expect("Not enough values in memory to process instruction");
            let parameter = match mode {
                ParameterMode::Pointer => Parameter::Pointer(*value as usize),
                ParameterMode::Value => Parameter::Value(*value),
            };
            parameters.push(parameter);
        }
        Instruction {
            operation: operation,
            parameters: parameters,
        }
    }
}
