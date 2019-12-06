use std::cell::{Cell, RefCell};

trait Computer<MemoryType> {
    fn execute(&self) -> Vec<MemoryType>;
}

pub struct IntCodeComputer<'a> {
    memory: RefCell<Vec<i32>>,
    instruction_ptr: Cell<usize>,
    input_callback: &'a dyn Fn() -> i32,
    output_callback: &'a dyn Fn(i32),
}

impl<'a> IntCodeComputer<'a> {
    fn new(
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
            let instruction = {
                let memory = self.memory.borrow();
                Instruction::read(&memory, &self.instruction_ptr.get())
            };
            println!(
                "Processing instruction: {:?} at {}",
                instruction,
                self.instruction_ptr.get()
            );
            let should_continue = self.execute_instruction(instruction);
            if !should_continue {
                break;
            }
            println!("Advancing to {}", self.instruction_ptr.get());
        }
        let mut result = Vec::<i32>::new();
        std::mem::swap(&mut result, &mut self.memory.borrow_mut());
        result
    }
}

impl<'a> IntCodeComputer<'a> {
    fn execute_instruction(&self, instruction: Instruction) -> bool {
        let mut memory = self.memory.borrow_mut();
        match instruction.operation {
            Operation::Add => {
                if let Parameter::Pointer(storage_index) = instruction.parameters[2] {
                    let operand1 = resolve_value_in_memory(instruction.parameters[0], &memory);
                    let operand2 = resolve_value_in_memory(instruction.parameters[1], &memory);
                    println!("Storing {} + {} in {}", operand1, operand2, storage_index);
                    memory[storage_index] = operand1 + operand2
                } else {
                    panic!("attempting to store arithmetic operation result to value");
                }
            }
            Operation::Multiply => {
                if let Parameter::Pointer(storage_index) = instruction.parameters[2] {
                    let operand1 = resolve_value_in_memory(instruction.parameters[0], &memory);
                    let operand2 = resolve_value_in_memory(instruction.parameters[1], &memory);
                    memory[storage_index] = operand1 * operand2
                } else {
                    panic!("attempting to store arithmetic operation result to value");
                }
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
            Operation::Halt => return false,
        }
        let old_ptr = self.instruction_ptr.get();
        let new_ptr = instruction.operation.parameter_count() + 1 + (old_ptr as i32);
        self.instruction_ptr.replace(new_ptr as usize);
        return true;
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
    Halt,
}

impl From<i32> for Operation {
    fn from(code: i32) -> Self {
        match code {
            1 => Self::Add,
            2 => Self::Multiply,
            3 => Self::Input,
            4 => Self::Output,
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
    println!("processing opcode: {}", *opcode);
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
    fn test_execute_empty() {
        let computer = IntCodeComputer::new(vec![], &|| 3, &|_| {});
        let output = computer.execute();
        assert_eq!(output, vec![]);
    }

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

    #[test]
    fn test_produces_same_output_as_old_computer() {
        let output = |_| {};
        let input: Box<dyn Fn() -> i32> = Box::new(|| panic!("input should never be called!"));

        assert_eq!(
            IntCodeComputer::new(vec![1, 0, 0, 0, 99], &input, &output).execute(),
            vec![2, 0, 0, 0, 99]
        );
        assert_eq!(
            IntCodeComputer::new(vec![2, 3, 0, 3, 99], &input, &output).execute(),
            vec![2, 3, 0, 6, 99]
        );
        assert_eq!(
            IntCodeComputer::new(vec![2, 4, 4, 5, 99, 0], &input, &output).execute(),
            vec![2, 4, 4, 5, 99, 9801]
        );

        assert_eq!(
            IntCodeComputer::new(vec![1, 1, 1, 4, 99, 5, 6, 0, 99], &input, &output).execute(),
            vec![30, 1, 1, 4, 2, 5, 6, 0, 99]
        );
        assert_eq!(
            IntCodeComputer::new(
                vec![1, 9, 10, 3, 2, 3, 11, 0, 99, 30, 40, 50],
                &input,
                &output
            )
            .execute(),
            vec![3500, 9, 10, 70, 2, 3, 11, 0, 99, 30, 40, 50]
        );
    }
}
