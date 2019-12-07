use std::cell::RefCell;
use std::collections::VecDeque;

#[derive(Debug)]
pub struct Pipe {
    buffer: RefCell<VecDeque<i32>>,
}

impl Pipe {
    pub fn new() -> Pipe {
        Pipe {
            buffer: RefCell::new(VecDeque::new()),
        }
    }

    pub fn receive(&self) -> i32 {
        return self
            .buffer
            .borrow_mut()
            .pop_front()
            .expect("pipe should have input");
    }

    pub fn send(&self, output: i32) {
        self.buffer.borrow_mut().push_back(output);
    }

    pub fn is_empty(&self) -> bool {
        self.buffer.borrow().is_empty()
    }
}
