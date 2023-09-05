pub struct Driver {}
impl Driver {
    pub fn main() {
        // let dispatcher = Dispatcher::new();
        println!("Bus Dispatching System\n");
        loop {
            let menu_input: usize = Driver::get_menu_input();
            println!("\n{menu_input}\n");
        }
    }

    fn get_menu_input() -> usize {
        loop {
            let input: usize = Driver::get_input_usize("1. Add bus\n2. Add Person to bus\n3. Remove bus\n4. Remove person\n5. List passengers\n6. List busses\n7. Requeue bus\n8. Transfer person\n9. Dispatch bus\n0. Exit\n");
            match input {
                1 => return input,
                2 => return input,
                3 => return input,
                4 => return input,    
                5 => return input,
                6 => return input,
                7 => return input,
                8 => return input,
                9 => return input,
                0 => return input,
                _ => eprintln!("\nInvalid input: input must be between 0-9 inclusive\n"),
            }
        }
    }

    fn get_input_usize(prompt: &str) -> usize {
        loop {
            match Driver::get_input_string(prompt).parse() {
                Ok(input) => return input,
                Err(parse_int_error) => {
                    eprintln!("\nInvalid input: {parse_int_error}\n");
                    continue;
                }
            }
        }
    }

    fn get_input_string(prompt: &str) -> String {
        use std::io::{stdin, stdout, Write};
        
        loop {
            match stdout().write(prompt.as_bytes()) {
                Ok(bytes_written) =>  {
                    if bytes_written == 0 {
                        eprintln!("\nError: Nothing written to stdout\n");
                        continue;
                    }

                    match stdout().flush() { // retrun flush error
                        Err(io_error) => {
                            eprintln!("\nError. Could not flush stdout: {io_error}\n");
                            continue;
                        }
                        _ => {}
                    }

                    let mut input: String = String::new();
                    
                    match stdin().read_line(&mut input) {
                        Ok(bytes_read) => {
                            if bytes_read == 0 { 
                                eprintln!("\nError: Nothing read from stdin\n");
                                continue;
                            }

                            return input.trim().to_owned();
                        }

                        Err(io_error) => {
                            eprintln!("\nInvalid input: {io_error}\n");
                            continue;
                        }
                    };
                }

                Err(io_error) => {
                    eprintln!("\nInvalid input: {io_error}\n");
                    continue;
                },
            };
        }
    }
}