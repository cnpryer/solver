use std::collections::HashMap;

use crate::{Float, Id, Number};

pub struct Input {
    stops: Vec<Stop>,
    vehicles: Vec<Vehicle>,
    distance_matrix: Vec<Vec<f64>>,
}

struct Stop {
    id: Id,
    precedes: Vec<Id>,
    quantity: HashMap<String, Number>,
    start_time_windows: [u64; 2],
    location: Location,
}

struct Location {
    lat: Float,
    lon: Float,
}

struct Vehicle {
    capacity: HashMap<String, Number>,
    speed: Float,
    initial_stops: Vec<InitialStop>,
}

struct InitialStop {
    id: Id,
}
