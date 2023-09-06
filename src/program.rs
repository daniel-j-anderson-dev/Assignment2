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
                            println!("{name} has been added to bus {id}")
                        }
                        None => println!("No bus with id {id}"),
                    }
                }
                3 => {
                    let id: usize = Driver::get_parsed_input("Enter bus id: ");
                    match dispatcher.remove_bus(id) {
                        Some(removed_bus) => println!("bus {removed_bus} removed"),
                        None => println!("No bus with id {id}"),
                    }
                }
                4 => {
                    let id: usize = Driver::get_parsed_input("Enter bus id: ");
                    match dispatcher.find_bus_mut(id) {
                        Some(found_bus) => {
                            let name: String = Driver::get_raw_input("Enter person's name: ");
                            let person: Person = match found_bus.clone().find_person(&name) {
                                Some(found_person) => found_person.clone(),
                                None => {
                                    println!("No such person found in bus {id}");
                                    continue;
                                }
                            };
                            match found_bus.remove_person(&person) {
                                Some(removed_person) => println!("{} has been removed from bus {id}", removed_person.get_name()),
                                None => {
                                    println!("No such person found in bus {id}");
                                    continue;
                                }
                            }
                        },
                        None => {
                            println!("No bus with id {id}");
                            continue;
                        }
                    };
                }
                5 => {
                    let id: usize = Driver::get_parsed_input("Enter bus id: ");
                    match dispatcher.find_bus(id) {
                        Some(found_bus) => {
                            print!("Bus id {}\n{}", found_bus.get_id(), found_bus.get_passengers());
                        }
                        None => {
                            println!("No bus with id {id}");
                            continue;
                        }
                    }
                }
                6 => {
                    print!("BUS QUEUE\n{dispatcher}");
                }
                7 => {
                    let id: usize = Driver::get_parsed_input("Enter bus id: ");
                    match dispatcher.remove_bus(id) {
                        Some(found_bus) => {
                            let new_queue_position: usize = Driver::get_parsed_input("Enter new bus position: ");
                            dispatcher.insert_bus(found_bus, new_queue_position);
                        }
                        None => {
                            println!("No bus with id {id}");
                            continue;
                        }
                    }
                }
                8 => {
                    // didnt use transfer_person but it still works because remove person returns an Option<person> instead of bool
                    let name: String = Driver::get_raw_input("Enter person's name: ");
                    let source_bus_id: usize = Driver::get_parsed_input("Enter id of bus 1: ");
                    let destination_bus_id: usize = Driver::get_parsed_input("Enter id of bus 2: ");
                    let person: Person = match dispatcher.find_bus_mut(source_bus_id) {
                        Some(source_bus) => {
                            match source_bus.clone().find_person(&name) {
                                Some(person) => source_bus.remove_person(person).unwrap(),
                                None => {
                                    println!("No person named {name} in bus {source_bus_id}");
                                    continue;
                                }
                            }
                        }
                        None => {
                            println!("No bus with id {source_bus_id}");
                            continue;
                        }
                    };
                    match dispatcher.find_bus_mut(destination_bus_id) {
                        Some(destination_bus) => destination_bus.add_person(person),
                        None => {
                            println!("No bus with id {destination_bus_id}");
                            continue;
                        }
                    }
                }
                9 => {
                    match dispatcher.dispatch_bus() {
                        Some(bus) => println!("Bus {} has been dispatched", bus.get_id()),
                        None => println!("Bus queue is empty"),
                    }
                }
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