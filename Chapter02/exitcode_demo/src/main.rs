extern crate exitcode;
use std::process;

pub fn parse_int_or_return_error_exitcode(s: String) -> Result<i32, exitcode::ExitCode> {
    match s.parse::<i32>() {
        Ok(i) => Ok(i),
        Err(_) => Err(exitcode::USAGE)
    }
}


fn main() {
    let arg1 = std::env::args().nth(1).expect("no arg1 given");
    let result = parse_int_or_return_error_exitcode(arg1);
    match result {
        Ok(i) => println!("Parsed: {}", i),
        Err(code) => {
            println!("Parse error. Exiting with code: {}", code);
            process::exit(code);
        }
    }

    println!("Exiting with code: {}", exitcode::OK);
    process::exit(exitcode::OK);
}
