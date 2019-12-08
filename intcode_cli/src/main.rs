use clap::{App, Arg};
use intcode_assembler::assemble;
use std::fs::{read_to_string, write};

fn main() {
    let matches = App::new("Assembler for IntCode")
        .version(env!("CARGO_PKG_VERSION"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .about(env!("CARGO_PKG_DESCRIPTION"))
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
        )
        .get_matches();
    let input_file = matches.value_of("INPUT").unwrap();
    let output_file = matches.value_of("OUTPUT").unwrap();
    let assembly = read_to_string(input_file).expect("Invalid input file");
    let intcode = assemble(&assembly);
    let intcode_strs: Vec<_> = intcode.into_iter().map(|i| i.to_string()).collect();
    let intcode_str = intcode_strs.join(",");

    match write(output_file, intcode_str) {
        Ok(()) => {}
        Err(error) => eprintln!("{}", error),
    }
}
