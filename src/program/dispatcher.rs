use crate::program::bus::Bus;

#[derive(Clone)]
pub struct Dispatcher {
    bus_queue: Vec<Bus>
}
impl Dispatcher {
    pub fn new() -> Dispatcher {
        return Dispatcher { bus_queue: Vec::<Bus>::new() }
    }

    pub fn add_bus(&mut self, bus: Bus) -> usize{
        self.bus_queue.push(bus);
        return self.bus_queue.len() - 1;
    }

    // add_bus() "overload"
    pub fn insert_bus(&mut self, bus: Bus, queue_position: usize) -> usize {
        if queue_position > self.bus_queue.len() {
            self.bus_queue.push(bus);
            return self.bus_queue.len();
        } else {            
            self.bus_queue.insert(queue_position, bus);
            return queue_position;
        }
    }

    pub fn find_bus(&self, bus_id: usize) -> Option<&Bus> {
        for (queue_position, bus) in self.bus_queue.iter().enumerate() {
            if bus.get_id() == bus_id {
                return self.bus_queue.get(queue_position);
            }
        }
        return None;
    }

    pub fn find_bus_mut(&mut self, bus_id: usize) -> Option<&mut Bus> {
        for (queue_position, bus) in self.bus_queue.iter().enumerate() {
            if bus.get_id() == bus_id {
                return self.bus_queue.get_mut(queue_position);
            }
        }
        return None;
    }

    pub fn remove_bus(&mut self, bus_id: usize) -> Option<(Bus, usize)> {
        for (queue_position, bus) in self.bus_queue.iter().enumerate() {
            if bus.get_id() == bus_id {
                return Some((self.bus_queue.remove(queue_position), queue_position));
            }
        }
        return None;
    }

    pub fn dispatch_bus(&mut self) -> Option<Bus> {
        if self.bus_queue.is_empty() {
            return None;
        } else {
            return Some(self.bus_queue.remove(0));
        }
    }
}

impl std::fmt::Display for Dispatcher {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut buses: String = String::new();
        for bus in self.bus_queue.iter() {
            buses.push_str(bus.to_string().as_str());
            buses.push('\n')
        }
        return write!(f, "{buses}");
    }
}