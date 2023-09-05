use crate::person::Person;

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