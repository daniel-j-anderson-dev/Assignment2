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
        return write!(f, "{} {}", self.ticket_number, self.name);
    }
}
impl Clone for Person {
    fn clone(&self) -> Self {
        return Person { 
            name: self.name.clone(),
            ticket_number: self.ticket_number.clone()
        };
    }
}
impl Person {
    fn new(name: &str) -> Self {
        let name: String = name.to_owned();
        let ticket_number: usize;
        unsafe { // program is not concurrent or parallel so this is OK
            ticket_number = NEXT_TICKET_NUMBER;
            NEXT_TICKET_NUMBER += 1;
        }
        return Person { name, ticket_number};
    }
}

#[test]
fn test() {
    let p1 = Person::default();
    let p2 = Person::new("Daniel");
    let p3 = Person::new("Ziz");
    assert!(&p1.to_string() == "0 NO NAME");
    assert!(&p2.to_string() == "1 Daniel");
    assert!(&p3.to_string() == "2 Ziz");
}