use std::collections::HashMap;

use crate::Id;

pub struct Input {
    pub stops: Vec<Stop>,
    pub vehicles: Vec<Vehicle>,
    pub distance_matrix: Vec<Vec<f64>>,
    pub options: Option<()>,
}

pub struct Stop {
    pub id: Id,
    pub precedes: Vec<Id>,
    pub quantity: HashMap<String, f64>,
    pub start_time_windows: [u64; 2],
    pub location: Location,
}

pub struct Location {
    pub lat: f64,
    pub lon: f64,
}

pub struct Vehicle {
    pub id: Id,
    pub capacity: HashMap<String, f64>,
    pub speed: f64,
    pub initial_stops: Vec<InitialStop>,
}

pub struct InitialStop {
    pub id: Id,
}
