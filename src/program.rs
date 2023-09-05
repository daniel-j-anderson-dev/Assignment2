pub struct Driver {}
impl Driver {
    pub fn main() {
        println!("Bus Dispatching System\n");
        let menu: &str ="1. Add bus\n2. Add Person to bus\n3. Remove bus\n4. Remove person\n5. List passengers\n6. List busses\n7. Requeue bus\n8. Transfer person\n9. Dispatch bus\n0. Exit\n";
        loop {
            match Driver::get_input(menu) {
                1 => {}
                2 => {}
                3 => {}
                4 => {}
                5 => {}
                6 => {}
                7 => {}
                8 => {}
                9 => {}
                0 => {}
                _ => eprintln!("Invalid input: input must be between 0-9 inclusive")
            }
        }
    }

    fn get_input<T>(prompt: &str) -> T 
    where
        T: std::str::FromStr,
        <T as std::str::FromStr>::Err: std::error::Error,
    {
        loop {
            match Driver::read_line(prompt).parse() {
                Ok(input) => return input,
                Err(parse_error) => {
                    eprintln!("\nInvalid input: {parse_error}\n");
                    continue;
                }
            }
        }
    }

    fn read_line(prompt: &str) -> String {
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