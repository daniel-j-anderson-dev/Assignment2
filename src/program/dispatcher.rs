use crate::program::bus::Bus;

pub struct Dispatcher {
    bus_queue: Vec<Bus>
}
impl Dispatcher {
    pub fn add_bus(&mut self, bus: Bus) {
        self.bus_queue.push(bus);
    }
}