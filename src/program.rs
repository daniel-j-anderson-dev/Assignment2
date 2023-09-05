mod bus;
mod person;
mod dispatcher;

use crate::program::{person::Person, bus::Bus, dispatcher::Dispatcher};

pub struct Driver {}
impl Driver {
    pub fn main() {
        let mut dispatcher: Dispatcher = Dispatcher::new();
        println!("Bus Dispatching System");
        loop {
            match Driver::get_menu_input() {
                1 => {
                    let bus: Bus = Bus::default();
                    let id: usize = bus.get_id();
                    let queue_position: usize = dispatcher.add_bus(bus);
                    println!("Bus {id} added to position {queue_position}");
                }
                2 => {
                    let id: usize = Driver::get_parsed_input("Enter bus id: ");
                    match dispatcher.find_bus_mut(id) {
                        Some(bus) => {
                            let name: String = Driver::get_raw_input("Enter person's name: ");
                            let person: Person = Person::new(name.as_str());
                            bus.add_person(person);
                            println!("\n{name} has been added to bus {id}")
                        }
                        None => println!("No bus with id {id}"),
                    }
                }
                3 => {
                    let id: usize = Driver::get_parsed_input("Enter bus id: ");
                    match dispatcher.remove_bus(id) {
                        Some((removed_bus, _queue_position)) => println!("bus {removed_bus} removed"),
                        None => println!("No bus with id {id}"),
                    }
                }
                4 => {
                    let id: usize = Driver::get_parsed_input("Enter bus id: ");
                    let name: String;
                    let bus: &mut Bus;
                    let person: &Person;

                    match dispatcher.find_bus_mut(id) {
                        Some(found_bus) => bus = found_bus,
                        None => {
                            println!("No bus with id {id}");
                            continue;
                        }
                    };

                    name = Driver::get_raw_input("Enter person's name: ");

                    let bus_clone = bus.clone();
                    match bus_clone.find_person(name.as_str()) {
                        Some(found_person) => person = found_person,
                        None => {
                            println!("No such person found in bus {id}");
                            continue;
                        }
                    }

                    bus.remove_person(person);
                }
                5 => {}
                6 => {}
                7 => {}
                8 => {}
                9 => {}
                0 => {
                    println!("Shutting down");
                    break;
                },
                _ => eprintln!("\nInvalid input: input must be between 0-9 inclusive")
            }
        }
    }

    fn get_menu_input() -> usize {
        let input: usize = Driver::get_parsed_input("\n1. Add bus\n2. Add Person to bus\n3. Remove bus\n4. Remove person\n5. List passengers\n6. List busses\n7. Requeue bus\n8. Transfer person\n9. Dispatch bus\n0. Exit\n");
        println!();
        return input;
    }

    fn get_parsed_input<T>(prompt: &str) -> T 
    where
        T: std::str::FromStr,
        <T as std::str::FromStr>::Err: std::error::Error,
    {
        loop {
            match Driver::get_raw_input(prompt).parse() {
                Ok(input) => {
                    return input
                },
                Err(parse_error) => {
                    eprintln!("\nInvalid input: {parse_error}\n");
                    continue;
                }
            }
        }
    }

    fn get_raw_input(prompt: &str) -> String {
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