pub struct TrafficLight {
    pub is_green: bool,       
    pub last_change: u32,     
}

impl TrafficLight {
    pub fn new() -> Self {
        TrafficLight {
            is_green: false,  
            last_change: 0,
        }
    }

    pub fn toggle(&mut self) {
        self.is_green = !self.is_green;
    }
}
