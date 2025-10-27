#[derive(Clone, Debug)]
pub struct Solution {
    vehicles: SolutionVehicles,
    unplanned: SolutionStops,
    value: f64,
    statistics: Option<SolutionStatistics>,
}

impl Default for Solution {
    fn default() -> Self {
        Self::new()
    }
}

impl Solution {
    #[must_use]
    pub fn new() -> Self {
        Solution {
            vehicles: SolutionVehicles::new(),
            unplanned: SolutionStops::new(),
            value: 0.0,
            statistics: None,
        }
    }

    #[must_use]
    pub fn value(&self) -> f64 {
        self.value
    }

    #[must_use]
    pub fn plan(&self, _plan: &Plan) -> Solution {
        todo!()
    }

    #[must_use]
    pub fn best(self, other: Solution) -> Solution {
        if self.value < other.value {
            self
        } else {
            other
        }
    }
}

#[derive(Clone, Debug)]
struct SolutionVehicles(Vec<SolutionVehicle>);

impl SolutionVehicles {
    pub fn new() -> Self {
        SolutionVehicles(Vec::new())
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn get(&self, index: usize) -> Option<&SolutionVehicle> {
        self.0.get(index)
    }

    pub fn push(&mut self, vehicle: SolutionVehicle) {
        self.0.push(vehicle);
    }
}

#[derive(Clone, Debug)]
struct SolutionVehicle {
    pub id: String,
    pub route: Vec<String>,
    pub cost: f64,
}

impl SolutionVehicle {
    pub fn new(id: String) -> Self {
        SolutionVehicle {
            id,
            route: Vec::new(),
            cost: 0.0,
        }
    }
}

#[derive(Clone, Debug)]
struct SolutionStops(Vec<SolutionStop>);

impl SolutionStops {
    pub fn new() -> Self {
        SolutionStops(Vec::new())
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn get(&self, index: usize) -> Option<&SolutionStop> {
        self.0.get(index)
    }

    pub fn push(&mut self, stop: SolutionStop) {
        self.0.push(stop);
    }
}

#[derive(Clone, Debug)]
struct SolutionStop {
    pub id: String,
    pub reason: String,
}

impl SolutionStop {
    pub fn new(id: String, reason: String) -> Self {
        SolutionStop { id, reason }
    }
}

#[derive(Clone, Debug)]
struct SolutionStatistics {
    iterations: usize,
    duration: f64,
}

pub struct Plan {}
