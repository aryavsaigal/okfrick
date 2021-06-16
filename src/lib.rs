use std::{env, fs};

pub fn parse_args(mut args: env::Args) -> Result<String, &'static  str> {
    args.next();
    match args.next() {
        Some(arg) => Ok(arg),
        None => Err("No filename provided.")
    }
}

pub fn read_file(filename: String) -> Result<String, Box<dyn std::error::Error>> {
    let content = fs::read_to_string(filename)?;
    Ok(content)
}

pub fn interpret(content: String) {
    let mut cell: u8 = 0;
    let mut index = 0;
    let chars: Vec<char> = content.chars().collect();
    loop {
        if index == content.len() { break };
        let c = chars[index];

        if c == '+' {
            if cell == 255 { 
                cell = 0;
            }
            else {
                cell += 1;
            };
        }
        else if c == '!' {
            print!("{}", String::from_utf8(vec!(cell)).expect("Unknown value."));
        }
        else if c == '?' {
            let mut input = String::new();
            std::io::stdin().read_line(&mut input).expect("Error reading line.");
            cell = input.trim().as_bytes()[0]
        }
        else if c == '(' && cell == 0 {
            let mut loop_count = 1;
            while loop_count > 0 {
                index += 1;
                let current_char = chars[index];
                if current_char == ')' {
                    loop_count -= 1;
                }
                else if current_char == '(' {
                    loop_count += 1;
                }
            }
        }
        else if c == ')' && cell != 0 {
            let mut loop_count = 1;
            while loop_count > 0 {
                index -= 1;
                let current_char = chars[index];
                if current_char == '(' {
                    loop_count -= 1;
                }
                else if current_char == ')' {
                    loop_count += 1;
                }
            }
        } 

        index += 1;
    }
}