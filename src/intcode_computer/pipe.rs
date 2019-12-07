use std::cell::RefCell;
use std::collections::VecDeque;

pub struct Pipe {
    buffer: RefCell<VecDeque<i32>>,
}

impl Pipe {
    pub fn new() -> Pipe {
        Pipe {
            buffer: RefCell::new(VecDeque::new()),
        }
    }

    pub fn input_handle(&self) -> i32 {
        return self.buffer.borrow_mut().pop_front().expect("has inputs");
    }

    pub fn output_handle(&self, output: i32) {
        self.buffer.borrow_mut().push_back(output);
    }
}
