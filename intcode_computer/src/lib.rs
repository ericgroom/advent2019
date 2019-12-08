mod instruction;
mod operations;
pub mod pipe;

use instruction::*;
use operations::*;
use std::cell::{Cell, RefCell};
use std::collections::VecDeque;

pub trait Computer<MemoryType> {
    /// returns false if the program has halted, if the value is true, there may be an interrupt
    fn execute(&self) -> bool;
}

pub struct IntCodeComputer<'a> {
    pub memory: RefCell<Vec<i32>>,
    instruction_ptr: Cell<usize>,
    input_buffer: RefCell<VecDeque<i32>>,
    output_callback: &'a dyn Fn(i32),
    interupted: Cell<bool>,
}

impl<'a> IntCodeComputer<'a> {
    pub fn new(memory: Vec<i32>, output: &'a dyn Fn(i32)) -> IntCodeComputer<'a> {
        IntCodeComputer {
            memory: RefCell::new(memory),
            instruction_ptr: Cell::new(0),
            input_buffer: RefCell::new(VecDeque::new()),
            output_callback: output,
            interupted: Cell::new(false),
        }
    }

    pub fn provide_input(&self, input: i32) {
        self.input_buffer.borrow_mut().push_back(input);
    }

    pub fn terminate(self) -> Vec<i32> {
        self.memory.into_inner()
    }
}

impl<'a> Computer<i32> for IntCodeComputer<'a> {
    fn execute(&self) -> bool {
        let memory_len = self.memory.borrow().len();
        while self.instruction_ptr.get() < memory_len {
            let instruction = {
                let memory = self.memory.borrow();
                Instruction::read(&memory, &self.instruction_ptr.get())
            };
            let interrupt = self.execute_instruction(instruction);
            if interrupt {
                self.interupted.replace(true);
                return true;
            }
        }
        return false;
    }
}

impl<'a> IntCodeComputer<'a> {
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
                    return true;
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
                return false;
            }
        }
        let old_ptr = self.instruction_ptr.get();
        let new_ptr = instruction.operation.parameter_count() + 1 + (old_ptr as i32);
        self.instruction_ptr.replace(new_ptr as usize);
        false
    }
}

fn arithmetic_operation(
    instruction: &Instruction,
    memory: &mut Vec<i32>,
    transform: Box<dyn FnOnce(i32, i32) -> i32>,
) {
    if let Parameter::Pointer(storage_index) = instruction.parameters[2] {
        let operand1 = resolve_value_in_memory(instruction.parameters[0], &memory);
        let operand2 = resolve_value_in_memory(instruction.parameters[1], &memory);
        memory[storage_index] = transform(operand1, operand2);
    } else {
        panic!("attempting to store arithmetic operation result to value");
    }
}

fn resolve_value_in_memory(parameter: Parameter, memory: &Vec<i32>) -> i32 {
    match parameter {
        Parameter::Value(value) => value,
        Parameter::Pointer(index) => memory[index],
    }
}

fn process_opcode(opcode: &i32) -> (Operation, Vec<ParameterMode>) {
    let operation_int = opcode % 100;
    let operation = Operation::from(operation_int);
    let mut parameter_modes = Vec::new();
    let mut parameter_section = opcode / 100;
    for _ in 0..operation.parameter_count() {
        let mode = parameter_section % 10;
        parameter_modes.push(ParameterMode::from(mode));
        parameter_section /= 10;
    }
    (operation, parameter_modes)
}

impl Instruction {
    fn read(memory: &Vec<i32>, instruction_ptr: &usize) -> Instruction {
        let mut iter = memory[*instruction_ptr..].iter();
        let opcode = iter.next().expect("Cannot read from empty memory");
        let (operation, parameter_modes) = process_opcode(&opcode);
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process_opcode() {
        assert_eq!(
            process_opcode(&1002),
            (
                Operation::Multiply,
                vec![
                    ParameterMode::Pointer,
                    ParameterMode::Value,
                    ParameterMode::Pointer
                ]
            )
        );
        assert_eq!(process_opcode(&99), (Operation::Halt, vec![]));
    }
}
