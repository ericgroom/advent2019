#[macro_export]
macro_rules! execute {
    ($computer:expr, input $input:block, output $output:block) => {
        loop {
            let interrupt = $computer.execute();
            match interrupt {
                Interrupt::Input => $input,
                Interrupt::Output => $output,
                Interrupt::Halt => break,
            }
        }
    };
    ($computer:expr, output $output:block, input $input:block) => {
        loop {
            let interrupt = $computer.execute();
            match interrupt {
                Interrupt::Input => $input,
                Interrupt::Output => $output,
                Interrupt::Halt => break,
            }
        }
    };
    ($computer:expr, input $input:block) => {
        loop {
            let interrupt = $computer.execute();
            match interrupt {
                Interrupt::Input => $input,
                Interrupt::Output => panic!("unexpected output interrupt"),
                Interrupt::Halt => break,
            }
        }
    };
    ($computer:expr, output $output:block) => {
        loop {
            let interrupt = $computer.execute();
            match interrupt {
                Interrupt::Input => panic!("unexpected input interrupt"),
                Interrupt::Output => $output,
                Interrupt::Halt => break,
            }
        }
    };
    ($computer:expr) => {
        loop {
            let interrupt = $computer.execute();
            match interrupt {
                Interrupt::Input => panic!("unexpected input interrupt"),
                Interrupt::Output => panic!("unexpected output interrupt"),
                Interrupt::Halt => break,
            }
        }
    };
}
