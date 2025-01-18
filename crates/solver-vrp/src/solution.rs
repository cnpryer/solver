use std::collections::HashMap;

use crate::Index;
use model::{PlanUnit as ModelPlanUnit, Stop as ModelStop};

pub struct Solution {
    vehicles: Vehicles,
    planned_plan_units: PlanUnitsCollection,
    unplanned_plan_units: PlanUnitsCollection,
}

impl Solution {
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

struct Vehicle {
    index: Index,
}

struct PlanUnitsCollection {
    // A map of `model::PlanUnit`'s `Index` to `PlanUnit`'s `Index` in each of the `PlanUnits` they are contained in.
    indices: HashMap<Index, Index>,
    plan_units: PlanUnits,
}

impl PlanUnitsCollection {
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
