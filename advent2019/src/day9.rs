use intcode_computer::{Computer, IntCodeComputer};

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::{Cell, RefCell};

    #[test]
    fn test_relative_mode() {
        let program = vec![
            109, 1, 204, -1, 1001, 100, 1, 100, 1008, 100, 16, 101, 1006, 101, 0, 99,
        ];
        let program_copy = program.clone();
        let output_container = RefCell::new(Vec::new());
        let output_handle = |i| output_container.borrow_mut().push(i);
        let computer = IntCodeComputer::new(program, &output_handle);
        while computer.execute() {}
        assert_eq!(program_copy, output_container.into_inner());
    }

    #[test]
    fn test_large_output() {
        let program = vec![1102, 34915192, 34915192, 7, 4, 7, 99, 0];
        let output_container = Cell::new(0);
        let output_handle = |i| {
            output_container.replace(i);
        };
        let computer = IntCodeComputer::new(program, &output_handle);
        while computer.execute() {}
        let mut output_digit = output_container.into_inner();
        let mut digit_count = 0;
        while output_digit != 0 {
            digit_count += 1;
            output_digit /= 10;
        }
        assert_eq!(digit_count, 16);
    }

    #[test]
    fn test_large_output_in_middle() {
        let program = vec![104, 1125899906842624, 99];
        let output_container = Cell::new(0);
        let output_handle = |i| {
            output_container.replace(i);
        };
        let computer = IntCodeComputer::new(program, &output_handle);
        while computer.execute() {}
        assert_eq!(output_container.into_inner(), 1125899906842624);
    }
}
