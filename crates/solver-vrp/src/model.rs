use std::collections::HashMap;

use crate::{schema::Input, Id, Index};

pub struct Model {
    stops: Stops,
    plan_units: PlanUnits,
    vehicles: Vehicles,
}

impl Model {
    pub fn new() -> Self {
        Self {
            stops: Stops::new(),
            vehicles: Vehicles::new(),
            plan_units: PlanUnits::new(),
        }
    }

    fn stops(&self) -> &Stops {
        &self.stops
    }

    fn vehicles(&self) -> &Vehicles {
        &self.vehicles
    }
}

impl From<Input> for Model {
    fn from(input: Input) -> Self {
        let mut stops = Stops(Vec::with_capacity(input.stops.len()));
        let mut stop_map = HashMap::with_capacity(input.stops.len());

        for (index, stop) in input.stops.iter().enumerate() {
            stops.push(Stop {
                id: stop.id.clone(),
                index,
                location: Location {
                    latitude: stop.location.lat.into(),
                    longitude: stop.location.lon.into(),
                },
            });
            stop_map.insert(&stop.id, index);
        }

        let vehicles = Vehicles(
            input
                .vehicles
                .iter()
                .enumerate()
                .map(|(index, vehicle)| Vehicle {
                    id: vehicle.id.clone(),
                    index,
                    stops: vehicle
                        .initial_stops
                        .iter()
                        .map(|stop| {
                            stops
                                .get(
                                    *stop_map
                                        .get(&stop.id)
                                        .expect("initial stop not found in stops"),
                                )
                                .unwrap()
                                .clone()
                        })
                        .collect(),
                })
                .collect(),
        );

        Self {
            stops,
            plan_units: todo!(),
            vehicles,
        }
    }
}

struct PlanUnits(Vec<PlanUnit>);

impl PlanUnits {
    fn new() -> Self {
        Self(Vec::new())
    }
}

pub struct PlanUnit {
    index: Index,
    stops: Vec<Stop>,
}

impl PlanUnit {
    pub fn index(&self) -> &Index {
        &self.index
    }

    fn stops(&self) -> &Vec<Stop> {
        &self.stops
    }
}

struct Stops(Vec<Stop>);

impl Stops {
    fn new() -> Self {
        Self(Vec::new())
    }

    fn get(&self, index: Index) -> Option<&Stop> {
        self.0.get(index)
    }

    fn push(&mut self, stop: Stop) {
        self.0.push(stop);
    }
}

#[derive(Clone)]
pub struct Stop {
    id: Id,
    index: Index,
    location: Location,
}

impl Stop {
    fn id(&self) -> &Id {
        &self.id
    }

    pub fn index(&self) -> Index {
        self.index
    }

    fn location(&self) -> Location {
        self.location
    }
}

#[derive(Clone, Copy)]
struct Location {
    latitude: f64,
    longitude: f64,
}

pub struct Vehicles(Vec<Vehicle>);

impl Vehicles {
    fn new() -> Self {
        Self(Vec::new())
    }
}

struct Vehicle {
    id: Id,
    index: Index,
    stops: Vec<Stop>,
}

impl Vehicle {
    fn id(&self) -> &Id {
        &self.id
    }

    fn index(&self) -> Index {
        self.index
    }

    fn stops(&self) -> &Vec<Stop> {
        &self.stops
    }

    fn push_stop(&mut self, stop: Stop) {
        self.stops.push(stop);
    }
}

struct VehicleType {
    id: Id,
    index: Index,
}

impl VehicleType {
    fn id(&self) -> &Id {
        &self.id
    }

    fn index(&self) -> Index {
        self.index
    }
}
