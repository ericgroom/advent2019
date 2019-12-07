use std::cell::{Cell, RefCell};

pub trait Computer<MemoryType> {
    fn execute(&self) -> Vec<MemoryType>;
}

pub struct IntCodeComputer<'a> {
    memory: RefCell<Vec<i32>>,
    instruction_ptr: Cell<usize>,
    input_callback: &'a dyn Fn() -> i32,
    output_callback: &'a dyn Fn(i32),
}

impl<'a> IntCodeComputer<'a> {
    pub fn new(
        memory: Vec<i32>,
        input: &'a dyn Fn() -> i32,
        output: &'a dyn Fn(i32),
    ) -> IntCodeComputer<'a> {
        IntCodeComputer {
            memory: RefCell::new(memory),
            instruction_ptr: Cell::new(0),
            input_callback: input,
            output_callback: output,
        }
    }
}

impl<'a> Computer<i32> for IntCodeComputer<'a> {
    fn execute(&self) -> Vec<i32> {
        let memory_len = self.memory.borrow().len();
        while self.instruction_ptr.get() < memory_len {
            // println!(
            //     "ptr: {}, memory: {:?}",
            //     self.instruction_ptr.get(),
            //     self.memory.borrow(),
            // );
            let instruction = {
                let memory = self.memory.borrow();
                Instruction::read(&memory, &self.instruction_ptr.get())
            };
            self.execute_instruction(instruction);
        }
        let mut result = Vec::<i32>::new();
        std::mem::swap(&mut result, &mut self.memory.borrow_mut());
        result
    }
}

impl<'a> IntCodeComputer<'a> {
    fn execute_instruction(&self, instruction: Instruction) {
        let mut memory = self.memory.borrow_mut();
        match instruction.operation {
            Operation::Add => {
                arithmetic_operation(&instruction, &mut memory, Box::new(|x, y| x + y));
            }
            Operation::Multiply => {
                arithmetic_operation(&instruction, &mut memory, Box::new(|x, y| x * y));
            }
            Operation::Input => {
                let input_result = (self.input_callback)();
                if let Parameter::Pointer(storage_index) = instruction.parameters[0] {
                    memory[storage_index] = input_result;
                } else {
                    panic!("attempting to store input result to value")
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
                    return;
                }
            }
            Operation::JumpIfFalse => {
                if resolve_value_in_memory(instruction.parameters[0], &memory) == 0 {
                    let jump_address = resolve_value_in_memory(instruction.parameters[1], &memory);
                    self.instruction_ptr.replace(jump_address as usize);
                    return;
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
                return;
            }
        }
        let old_ptr = self.instruction_ptr.get();
        let new_ptr = instruction.operation.parameter_count() + 1 + (old_ptr as i32);
        self.instruction_ptr.replace(new_ptr as usize);
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

#[derive(Clone, Copy, Debug)]
enum Parameter {
    Value(i32),
    Pointer(usize),
}

#[derive(Clone, Copy, PartialEq, Debug)]
enum ParameterMode {
    Value,
    Pointer,
}

impl From<i32> for ParameterMode {
    fn from(code: i32) -> Self {
        match code {
            0 => Self::Pointer,
            1 => Self::Value,
            x => panic!("Unknown parameter mode: {}", x),
        }
    }
}

#[derive(PartialEq, Debug)]
enum Operation {
    Add,
    Multiply,
    Input,
    Output,
    JumpIfTrue,
    JumpIfFalse,
    LessThan,
    Equals,
    Halt,
}

impl From<i32> for Operation {
    fn from(code: i32) -> Self {
        match code {
            1 => Self::Add,
            2 => Self::Multiply,
            3 => Self::Input,
            4 => Self::Output,
            5 => Self::JumpIfTrue,
            6 => Self::JumpIfFalse,
            7 => Self::LessThan,
            8 => Self::Equals,
            99 => Self::Halt,
            x => panic!("Unknown opcode: {}", x),
        }
    }
}

impl Operation {
    fn parameter_count(&self) -> i32 {
        match *self {
            Self::Add => 3,
            Self::Multiply => 3,
            Self::Input => 1,
            Self::Output => 1,
            Self::JumpIfTrue => 2,
            Self::JumpIfFalse => 2,
            Self::LessThan => 3,
            Self::Equals => 3,
            Self::Halt => 0,
        }
    }
}

#[derive(Debug)]
struct Instruction {
    operation: Operation,
    parameters: Vec<Parameter>,
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
