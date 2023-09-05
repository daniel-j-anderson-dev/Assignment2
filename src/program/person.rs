static mut NEXT_TICKET_NUMBER: usize = 0;
pub struct Person {
    name: String,
    ticket_number: usize,
}
impl Default for Person {
    fn default() -> Self {
        let name: String = "NO NAME".to_owned();
        let ticket_number: usize;
        unsafe { // program is not concurrent or parallel so this is OK
            ticket_number = NEXT_TICKET_NUMBER;
            NEXT_TICKET_NUMBER += 1;
        }
        return Person { name, ticket_number};
    }
}

impl std::fmt::Display for Person{ // this trait gives .to_string()
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        return write!(f, "{} {}", self.get_ticket_number(), self.get_name());
    }
}

impl Person {
    pub fn new(name: &str) -> Self {
        let name: String = name.to_owned();
        let ticket_number: usize;
        unsafe { // program is not concurrent or parallel so this is OK
            ticket_number = NEXT_TICKET_NUMBER;
            NEXT_TICKET_NUMBER += 1;
        }
        return Person { name, ticket_number};
    }
    
    pub fn get_name(&self) -> &str {
        return self.name.as_str();
    }

    pub fn get_ticket_number(&self) -> &usize {
        return &self.ticket_number;
    }
}

// allows for &[Person].to_vec()
impl Clone for Person {
    fn clone(&self) -> Self {
        return Person { 
            name: self.name.clone(),
            ticket_number: self.ticket_number.clone()
        };
    }
}

// allows for sorting
impl PartialEq for Person {
    fn eq(&self, other: &Self) -> bool {
        return self.ticket_number == other.ticket_number;
    }
}
impl Eq for Person {}

impl Ord for Person {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        return self.ticket_number.cmp(&other.ticket_number);
    }
}
impl PartialOrd for Person {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        return Some(self.cmp(other));
    }
}
#[cfg(test)]
mod test {
    use crate::program::person::Person;
    #[test]
    fn test() {
        let p1 = Person::default();
        let p2 = Person::new("Daniel");
        let p3 = Person::new("Ziz");
        assert!(&p1.to_string() == "0 NO NAME");
        assert!(&p2.to_string() == "1 Daniel");
        assert!(&p3.to_string() == "2 Ziz");
    }
}