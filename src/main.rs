use std::{env, process};

fn main() {
    let filename = match okfrick::parse_args(env::args()) {
        Ok(arg) => arg,
        Err(err) => {
            eprintln!("{}", err);
            process::exit(1);
        }
    };

    let content = okfrick::read_file(filename).unwrap_or_else(|err| {
        eprintln!("{}", err);
        process::exit(1);
    });
    okfrick::interpret(content);
}
