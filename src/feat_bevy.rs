use std::{collections::hash_map::Iter, hash::Hash};
use bevy::prelude::*;
use nalgebra::Vector3;
use num_traits::FromPrimitive;
use crate::{Database, DatabaseEntry};


#[derive(Default, Resource)]
pub struct BevyPlanetDatabase<H> {
    database: Database<H, f32>
}
impl<H> BevyPlanetDatabase<H> where H: Clone + Eq + Hash + FromPrimitive {
    pub fn get_entry(&self, handle: &H) -> &DatabaseEntry<H, f32> {
        self.database.get_entry(handle)
    }
    pub fn position_at_mean_anomaly(&self, handle: &H, mean_anomaly: f32) -> Vec3 {
        vec_nalgebra_to_bevy(self.database.position_at_mean_anomaly(handle, mean_anomaly))
    }
    pub fn absolute_position_at_time(&self, handle: &H, time: f32) -> Vec3 {
        vec_nalgebra_to_bevy(self.database.absolute_position_at_time(handle, time))
    }
    pub fn radius_soi(&self, handle: &H) -> f32 {
        self.database.radius_soi(handle)
    }
    pub fn with_solar_system(mut self) -> Self {
        self.database = self.database.with_solar_system();
        self
    }
    pub fn iter(&self) -> Iter<'_, H, DatabaseEntry<H, f32>> {
        self.database.iter()
    }
}

pub fn vec_nalgebra_to_bevy(input: Vector3<f32>) -> Vec3 {
    Vec3::new(input.x, input.y, input.z)
}
