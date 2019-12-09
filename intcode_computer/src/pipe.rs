use super::IntcodeMemoryCellType;
use std::cell::RefCell;
use std::collections::VecDeque;

#[derive(Debug)]
pub struct Pipe {
    buffer: RefCell<VecDeque<IntcodeMemoryCellType>>,
}

impl Pipe {
    pub fn new() -> Pipe {
        Pipe {
            buffer: RefCell::new(VecDeque::new()),
        }
    }

    pub fn receive(&self) -> IntcodeMemoryCellType {
        return self
            .buffer
            .borrow_mut()
            .pop_front()
            .expect("pipe should have input");
    }

    pub fn send(&self, output: IntcodeMemoryCellType) {
        self.buffer.borrow_mut().push_back(output);
    }

    pub fn is_empty(&self) -> bool {
        self.buffer.borrow().is_empty()
    }
}
