use std::collections::HashMap;

use types::{Float, Id, Number};

pub struct Input {
    pub stops: Vec<Stop>,
    pub vehicles: Vec<Vehicle>,
    pub distance_matrix: Vec<Vec<Number>>,
    pub options: Option<()>,
}

pub struct Stop {
    pub id: Id,
    pub precedes: Vec<Id>,
    pub quantity: HashMap<String, Number>,
    pub start_time_windows: [u64; 2],
    pub location: Location,
}

pub struct Location {
    pub lat: Float,
    pub lon: Float,
}

pub struct Vehicle {
    pub id: Id,
    pub capacity: HashMap<String, Number>,
    pub speed: Float,
    pub initial_stops: Vec<InitialStop>,
}

pub struct InitialStop {
    pub id: Id,
}
