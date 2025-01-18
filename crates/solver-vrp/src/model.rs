use crate::{Id, Index};

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
}

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
    vehicle_type: VehicleType,
    stops: Vec<Stop>,
}

impl Vehicle {
    fn id(&self) -> &Id {
        &self.id
    }

    fn index(&self) -> Index {
        self.index
    }

    fn vehicle_type(&self) -> &VehicleType {
        &self.vehicle_type
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
