use clap::{App, Arg, SubCommand};
use intcode_assembler::assemble;
use intcode_computer::{Computer, IntCodeComputer};
use std::fs::{read_to_string, write};
use std::io;

fn main() {
    let build_command = SubCommand::with_name("build")
        .about("builds assembly into intcode program")
        .arg(
            Arg::with_name("OUTPUT")
                .short("o")
                .long("output file")
                .value_name("FILE")
                .help("Sets a custom output file")
                .default_value("a.int")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("INPUT")
                .help("Sets the input file to use")
                .required(true)
                .index(1),
        );
    let run_command = SubCommand::with_name("run")
        .about("runs an intcode program")
        .arg(
            Arg::with_name("INPUT")
                .help("Sets the input file to use")
                .required(true)
                .index(1),
        );
    let matches = App::new("Assembler for IntCode")
        .version(env!("CARGO_PKG_VERSION"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .about(env!("CARGO_PKG_DESCRIPTION"))
        .subcommand(build_command)
        .subcommand(run_command)
        .get_matches();
    if let Some(subcommand) = matches.subcommand_name() {
        match subcommand {
            "build" => {
                let matches = matches.subcommand_matches("build").unwrap();
                let input_file = matches.value_of("INPUT").unwrap();
                let output_file = matches.value_of("OUTPUT").unwrap();
                build(input_file, output_file);
            }
            "run" => {
                let matches = matches.subcommand_matches("run").unwrap();
                let input_file = matches.value_of("INPUT").unwrap();
                run(input_file);
            }
            _ => {}
        }
    }
}

fn build(input_file: &str, output_file: &str) {
    let assembly = read_to_string(input_file).expect("Invalid input file");
    let intcode = assemble(&assembly);
    let intcode_strs: Vec<_> = intcode.into_iter().map(|i| i.to_string()).collect();
    let intcode_str = intcode_strs.join(",");

    match write(output_file, intcode_str) {
        Ok(()) => {}
        Err(error) => eprintln!("{}", error),
    }
}

fn run(input_file: &str) {
    let intcode_str = read_to_string(input_file).expect("Invalid input file");
    let intcode: Vec<i32> = intcode_str.split(',').map(|s| s.parse().unwrap()).collect();
    let output_handler = |output| println!("{}", output);
    let computer = IntCodeComputer::new(intcode, &output_handler);
    let mut str_buffer = String::new();
    let mut int_buffer: Vec<i32> = Vec::new();
    while computer.execute() {
        while int_buffer.is_empty() {
            match io::stdin().read_line(&mut str_buffer) {
                Ok(_) => {
                    for maybe_int in str_buffer.lines() {
                        if let Some(input) = maybe_int.parse::<i32>().ok() {
                            int_buffer.push(input);
                        }
                    }
                }
                Err(error) => eprint!("{}", error),
            }
        }
        for input in int_buffer.drain(..) {
            computer.provide_input(input);
        }
    }
}
