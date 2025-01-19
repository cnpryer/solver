use std::{
    collections::{HashMap, HashSet},
    vec::IntoIter,
};

use crate::{Id, Index, schema::Input};

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

    pub fn vehicles(&self) -> &Vehicles {
        &self.vehicles
    }
}

impl From<Input> for Model {
    fn from(input: Input) -> Self {
        let stop_count = input.stops.len();
        let mut stops_vec = Vec::with_capacity(stop_count);
        let mut stop_map = HashMap::with_capacity(stop_count);

        for (index, stop) in input.stops.iter().enumerate() {
            let model_stop = Stop {
                id: stop.id.clone(),
                index,
                location: Location {
                    latitude: stop.location.lat,
                    longitude: stop.location.lon,
                },
            };

            stop_map.insert(stop.id.as_str(), index);
            stops_vec.push(model_stop);
        }

        let mut plan_units = PlanUnits::new();
        let mut plan_unit_set = HashSet::with_capacity(stop_count);

        for (index, stop) in input.stops.iter().enumerate() {
            if plan_unit_set.contains(&index) {
                continue;
            }

            let mut plan_unit = PlanUnit {
                index: plan_units.len(),
                stops: vec![index],
            };

            // Currently only one pickup and one delivery stop is supported.
            if stop
                .precedes
                .as_ref()
                .is_some_and(|precedes| precedes.len() > 1)
            {
                panic!("stop {} has too many precedes", stop.id);
            }

            if let Some(id) = stop.precedes.as_ref().and_then(|precedes| precedes.first()) {
                let next_index = *stop_map.get(id.as_str()).expect("stop precedes not found");
                if plan_unit_set.contains(&next_index) {
                    panic!("stop {} is already part of a plan unit", id);
                }
                plan_unit.stops.push(next_index);
                plan_unit_set.insert(next_index);
            }

            plan_unit_set.insert(index);
            plan_units.push(plan_unit);
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
                        .as_ref()
                        .map(|it| {
                            it.iter()
                                .map(|stop| {
                                    let stop_index = *stop_map
                                        .get(stop.id.as_str())
                                        .expect("initial stop not found in stops");
                                    stops_vec[stop_index].clone()
                                })
                                .collect()
                        })
                        .unwrap_or(Vec::new()),
                })
                .collect(),
        );

        Self {
            stops: Stops(stops_vec),
            plan_units,
            vehicles,
        }
    }
}

struct PlanUnits(Vec<PlanUnit>);

impl PlanUnits {
    fn new() -> Self {
        Self(Vec::new())
    }

    fn len(&self) -> usize {
        self.0.len()
    }

    fn push(&mut self, plan_unit: PlanUnit) {
        self.0.push(plan_unit);
    }
}

pub struct PlanUnit {
    index: Index,
    stops: Vec<Index>,
}

impl PlanUnit {
    pub fn index(&self) -> &Index {
        &self.index
    }

    fn stops(&self) -> &Vec<Index> {
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

    // TODO
    pub fn iter(&self) -> impl Iterator<Item = &Vehicle> {
        self.0.iter()
    }
}

pub struct Vehicle {
    id: Id,
    index: Index,
    stops: Vec<Stop>,
}

impl Vehicle {
    fn id(&self) -> &Id {
        &self.id
    }

    pub fn index(&self) -> Index {
        self.index
    }

    pub fn stops(&self) -> &Vec<Stop> {
        &self.stops
    }

    fn push_stop(&mut self, stop: Stop) {
        self.stops.push(stop);
    }

    pub fn is_empty(&self) -> bool {
        self.stops.is_empty()
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
