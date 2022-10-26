#[derive(Clone)]
pub struct Connection {
    pub weight: f64,
    pub delta_weight: f64,
}

impl Connection {
    pub fn new() -> Connection {
        Connection {
            weight: 0.0,
            delta_weight: 0.0,
        }
    }
}