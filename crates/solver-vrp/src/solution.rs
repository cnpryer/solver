use std::collections::HashMap;

use crate::{
    Index,
    model::{self, Model},
};
use model::{PlanUnit as ModelPlanUnit, Stop as ModelStop};

pub struct Solution {
    value: f64,
    vehicles: Vehicles,
    planned_plan_units: PlanUnitsCollection,
    unplanned_plan_units: PlanUnitsCollection,
}

impl Solution {
    pub fn new(model: &Model) -> Self {
        let mut planned_plan_units = PlanUnitsCollection::new();
        let mut unplanned_plan_units = PlanUnitsCollection::new();

        let mut solution = Self {
            value: 0.,
            vehicles: Vehicles(
                model
                    .vehicles()
                    .iter()
                    .filter(|vehicle| !vehicle.is_empty())
                    .map(|vehicle| {
                        for stop in vehicle.stops() {
                            // TODO: Naive solution for now.
                            todo!("pull planned and unplanned plan units from model")
                        }

                        Vehicle {
                            index: vehicle.index(),
                        }
                    })
                    .collect(),
            ),
            planned_plan_units,
            unplanned_plan_units,
        };

        solution
    }

    pub fn value(&self) -> f64 {
        self.value
    }

    fn vehicles(&self) -> &Vehicles {
        &self.vehicles
    }

    fn stop(&self, model_plan_unit: &ModelPlanUnit, model_stop: &ModelStop) -> Option<&Stop> {
        self.planned_stop(model_plan_unit, model_stop)
            .or(self.unplanned_stop(model_plan_unit, model_stop))
    }

    fn planned_stop(
        &self,
        model_plan_unit: &ModelPlanUnit,
        model_stop: &ModelStop,
    ) -> Option<&Stop> {
        self.planned_plan_units
            .plan_unit_index(model_plan_unit)
            .and_then(|index| self.planned_plan_units.plan_unit_stop(index, model_stop))
    }

    fn unplanned_stop(
        &self,
        model_plan_unit: &ModelPlanUnit,
        model_stop: &ModelStop,
    ) -> Option<&Stop> {
        self.unplanned_plan_units
            .plan_unit_index(model_plan_unit)
            .and_then(|index| self.unplanned_plan_units.plan_unit_stop(index, model_stop))
    }
}

struct Vehicles(Vec<Vehicle>);

impl Vehicles {
    // TODO
    pub fn iter(&self) -> impl Iterator<Item = &Vehicle> {
        self.0.iter()
    }
}

struct Vehicle {
    index: Index,
}

struct PlanUnitsCollection {
    indices: HashMap<ModelIndex, Index>,
    plan_units: PlanUnits,
}

type ModelIndex = Index;

impl PlanUnitsCollection {
    fn new() -> Self {
        Self {
            indices: HashMap::new(),
            plan_units: PlanUnits::new(),
        }
    }

    fn plan_unit_index(&self, model_plan_unit: &ModelPlanUnit) -> Option<&Index> {
        self.indices.get(model_plan_unit.index())
    }

    fn plan_unit_stop(&self, plan_unit_index: &Index, model_stop: &ModelStop) -> Option<&Stop> {
        self.plan_units
            .get(*plan_unit_index)
            .and_then(|unit| unit.stop(model_stop))
    }
}

struct PlanUnits(Vec<PlanUnit>);

impl PlanUnits {
    fn new() -> Self {
        Self(Vec::new())
    }

    fn get(&self, index: Index) -> Option<&PlanUnit> {
        self.0.get(index)
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

    fn stop(&self, model_stop: &ModelStop) -> Option<&Stop> {
        self.stops()
            .iter()
            .find(|it| it.index == model_stop.index())
    }
}

struct Stops(Vec<Stop>);

struct Stop {
    index: Index,
}
