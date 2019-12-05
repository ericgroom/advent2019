use std::cell::RefCell;

trait Computer<MemoryType> {
    fn execute(&self) -> Vec<MemoryType>;
}

pub struct IntCodeComputer<'a> {
    memory: RefCell<Vec<i32>>,
    instruction_ptr: usize,
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
            instruction_ptr: 0,
            input_callback: input,
            output_callback: output,
        }
    }
}

impl<'a> Computer<i32> for IntCodeComputer<'a> {
    fn execute(&self) -> Vec<i32> {
        let memory = self.memory.borrow();
        while self.instruction_ptr < memory.len() {
            let instruction = Instruction::read(&memory);
            match instruction.operation {
                Operation::Add => {
                    if let Pointer(storage_index) = instruction.parameters[0] {
                        let operand1 = resolve_value_in_memory(instruction.parameters[1], &memory);
                        let operand2 = resolve_value_in_memory(instruction.parameters[2], &memory);
                        memory[storage_index] = operand1 + operand2
                    } else {
                        panic!("attempting to store arithmetic operation result to value");
                    }
                }
            }
        }
        vec![]
    }
}

fn resolve_value_in_memory(parameter: Parameter, memory: &Vec<i32>) -> i32 {
    match parameter {
        Parameter::Value(value) => value,
        Parameter::Pointer(index) => memory[index],
    }
}

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

struct Instruction {
    operation: Operation,
    parameters: Vec<Parameter>,
}

fn process_opcode(opcode: &i32) -> (Operation, Vec<ParameterMode>) {
    let operation_int = opcode % 100;
    let mode1_int = (opcode / 100) % 10;
    let mode2_int = (opcode / 1000) % 10;
    let mode3_int = (opcode / 10000) % 10;
    (
        Operation::from(operation_int),
        vec![
            ParameterMode::from(mode1_int),
            ParameterMode::from(mode2_int),
            ParameterMode::from(mode3_int),
        ],
    )
}

impl Instruction {
    fn read(memory: &Vec<i32>) -> Instruction {
        let mut iter = memory.iter();
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
    }
}
