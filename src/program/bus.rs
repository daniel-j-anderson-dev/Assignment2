use crate::program::person::Person;

static mut NEXT_ID: usize = 0;
pub struct Bus {
    passengers: Vec<Person>,
    id: usize,
}
impl Default for Bus {
    fn default() -> Self {
        let passengers: Vec<Person> = Vec::new();
        let id: usize;
        unsafe {
            id = NEXT_ID;
            NEXT_ID += 1;
        }
        return Bus { passengers, id };
    }
}
impl std::fmt::Display for Bus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        return write!(f, "{}", self.get_id());
    }
}
impl Bus {
    fn new(passengers: &[Person]) -> Self {
        let passengers: Vec<Person> = passengers.to_vec();
        let id: usize;
        unsafe {
            id = NEXT_ID;
            NEXT_ID += 1;
        }
        return Bus { passengers, id };
    }

    pub fn get_id(&self) -> usize {
        return self.id;
    }

    pub fn add_person(&mut self, person: Person) {
        self.passengers.push(person);
    }

    pub fn remove_person(&mut self, person: &Person) -> Option<Person> {
        match self.passengers.binary_search(&person) {
            Ok(found_index) => return Some(self.passengers.remove(found_index)),
            Err(_possible_index) => return None,
        }
    }

    pub fn find_person(&self, name: &str) -> Option<&Person> {
        for person in self.passengers.iter() {
            if name == person.get_name() {
                return Some(&person);
            }
        }
        return None;
    }

    pub fn transfer_person(source_bus: &mut Bus, destination_bus: &mut Bus, person: &Person) -> bool {
        if let Some(removed_person) = source_bus.remove_person(person) {
            destination_bus.add_person(removed_person);
            return true;
        } else {
            return false;
        }
    }

    pub fn get_passengers(&self) -> String {
        let mut passengers_string: String = String::new();
        for passenger in self.passengers.iter() {
            passengers_string.push_str(passenger.to_string().as_str());
            passengers_string.push('\n');
        }
        return passengers_string;
    }
}

/// Test must not be run concurrently. this will caused undefined behavior
#[cfg(test)]
mod test {
    use crate::program::{bus::Bus, person::Person};
    #[test]
    fn constructors() {
        let passengers_b = [Person::default(), Person::default(), Person::default()];
        let bus_a = Bus::default();
        let bus_b = Bus::new(&passengers_b);
        
        assert!(bus_a.passengers == Vec::new());
        assert!(bus_a.id == 0);
    
        assert!(bus_b.passengers == passengers_b.to_vec());
        assert!(bus_b.id == 1);
    }
    
    #[test]
    fn get_passengers() {
        let bus = Bus::new(&[Person::default(), Person::default(), Person::default()]);
        let mut expected_output = String::new();
        for passenger in bus.passengers.iter() {
            expected_output.push_str(format!("{passenger}\n").as_str());
        }
        assert!(bus.get_passengers() == expected_output);
    }

    #[test]
    fn to_string() { // also checks get_id() implicitly
        let bus = Bus::new(&[Person::default(), Person::default(), Person::default()]);
        let expected_output = bus.get_id().to_string();
        assert!(bus.to_string() == expected_output);
    }

    #[test]
    fn add_person() {
        let mut passengers = vec![Person::new("a"), Person::new("b"), Person::new("c")];
        let mut bus = Bus::new(&passengers);
        let person_to_add = Person::new("d"); 
        bus.add_person(person_to_add.clone()); // if i dont clone then ticket number will increment
        passengers.push(person_to_add.clone());
        assert!(bus.passengers == passengers);
    }
    
    #[test]
    fn remove_person() {
        let person_a = Person::new("a"); 
        let person_b = Person::new("b"); 
        let person_c = Person::new("c"); 
        let mut passengers = vec![person_a.clone(), person_b.clone(), person_c.clone()];
        let mut bus = Bus::new(&passengers);
        bus.remove_person(&person_b);
        passengers.remove(1);
        assert!(bus.passengers == passengers);
    }

    #[test]
    fn find_person() {
        let person_a = Person::new("a"); 
        let person_b = Person::new("b"); 
        let person_c = Person::new("c"); 
        let passengers = vec![person_a.clone(), person_b.clone(), person_c.clone()];
        let bus = Bus::new(&passengers);

        let output_a = bus.find_person("a");
        let output_b = bus.find_person("b");
        let otuput_c = bus.find_person("c");
        let output_d = bus.find_person("d");

        assert!(output_a == Some(&person_a));
        assert!(output_b == Some(&person_b));
        assert!(otuput_c == Some(&person_c));
        assert!(output_d == None);
    }
    
    #[test]
    fn transfer_person() {
        let person_a = Person::new("a"); 
        let person_b = Person::new("b"); 
        let person_c = Person::new("c"); 
        let person_e = Person::new("e"); 
        let person_f = Person::new("f"); 
        let person_g = Person::new("g"); 

        let mut source_bus_passengers = vec![person_a.clone(), person_b.clone(), person_c.clone()];
        let mut destination_bus_passengers = vec![person_e.clone(), person_f.clone(), person_g.clone()];

        let mut source_bus = Bus::new(&source_bus_passengers);
        let mut destination_bus = Bus::new(&destination_bus_passengers);

        Bus::transfer_person(&mut source_bus, &mut destination_bus, &person_b);

        destination_bus_passengers.push(source_bus_passengers.remove(1));

        assert!(source_bus.passengers == source_bus_passengers);
        assert!(destination_bus.passengers == destination_bus_passengers);
    }
    
}
