use crate::program::bus::Bus;

pub struct Dispatcher {
    bus_queue: Vec<Bus>
}
impl Dispatcher {
    pub fn add_bus(&mut self, bus: Bus) {
        self.bus_queue.push(bus);
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
}