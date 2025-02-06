use std::{collections::{hash_map::Iter, HashMap}, hash::Hash, ops::Mul};
use nalgebra::{RealField, Rotation3, SimdRealField, SimdValue, Vector3};
use num_traits::{Float, FromPrimitive};
use crate::{constants::f64::CONVERT_DEG_TO_RAD, Body, OrbitalElements};

#[cfg(feature="bevy")]
use bevy::prelude::*;

/// Holds the data for all the bodies being simulated
/// 
/// This is the main source of information for game engine implementations. The game engine should
/// feed its celestial body information into this database, and then query it to get the results of
/// calculations back.
/// 
/// `T` is the type used for the floating point data stored inside the database, and `H` is the
/// hashable type used for handles to celestial bodies which are used to retrieve a specific body
/// from the database and also define parent/child relationships
#[cfg_attr(feature="bevy", derive(Resource))]
pub struct Database<H, T> {
	bodies: HashMap<H, DatabaseEntry<H, T>>,
}
impl<H, T> Database<H, T> where H: Clone + Eq + Hash + FromPrimitive, T: Clone + Float + FromPrimitive {
	/// Creates a new database pre-populated with celestial bodies from our solar system
	pub fn solar_system() -> Self {
		let mut db = Self::default();
		// sol/sun
		let sun_handle = H::from_u16(0).unwrap();
		let sun_info: Body<T> = Body::new_sol();
		let sun_entry = DatabaseEntry::new(sun_info).with_scale(T::from_f64(1.0 / 5_000_000_000.0).unwrap());
		db.add_entry(sun_handle.clone(), sun_entry);
		// mercury
		let mercury_handle = H::from_u16(1).unwrap();
		let mercury_info: Body<T> = Body::default()
			.with_mass_kg(T::from_f64(3.3011e23).unwrap())
			.with_radius_km(T::from_f64(2439.7).unwrap());
		let mercury_orbit: OrbitalElements<T> = OrbitalElements::default()
			.with_semimajor_axis_km(T::from_f64(5.791e7).unwrap())
			.with_eccentricity(T::from_f64(0.205630).unwrap())
			.with_inclination_deg(T::from_f64(7.005).unwrap())
			.with_arg_of_periapsis_deg(T::from_f64(29.124).unwrap())
			.with_long_of_ascending_node_deg(T::from_f64(48.331).unwrap());
		let mercury_entry = DatabaseEntry::new(mercury_info)
			.with_parent(sun_handle.clone(), mercury_orbit)
			.with_mean_anomaly_deg(T::from_f64(174.796).unwrap());
		db.add_entry(mercury_handle, mercury_entry);
		// venus
		let venus_handle = H::from_u16(2).unwrap();
		let venus_info: Body<T> = Body::default()
			.with_mass_kg(T::from_f64(4.8675e24).unwrap())
			.with_radius_km(T::from_f64(6051.8).unwrap());
		let venus_orbit: OrbitalElements<T> = OrbitalElements::default()
			.with_semimajor_axis_km(T::from_f64(1.0821e8).unwrap())
			.with_eccentricity(T::from_f64(0.006772).unwrap())
			.with_inclination_deg(T::from_f64(3.39458).unwrap())
			.with_arg_of_periapsis_deg(T::from_f64(54.884).unwrap())
			.with_long_of_ascending_node_deg(T::from_f64(76.680).unwrap());
		let venus_entry = DatabaseEntry::new(venus_info)
			.with_parent(sun_handle.clone(), venus_orbit)
			.with_mean_anomaly_deg(T::from_f64(	50.115).unwrap());
		db.add_entry(venus_handle, venus_entry);
		// earth
		let earth_handle = H::from_u16(3).unwrap();
		let earth_info: Body<T> = Body::new_earth();
		let earth_orbit: OrbitalElements<T> = OrbitalElements::default()
			.with_semimajor_axis_km(T::from_f64(149598023.0).unwrap())
			.with_eccentricity(T::from_f64(0.0167086).unwrap())
			.with_inclination_deg(T::from_f64(0.00005).unwrap())
			.with_arg_of_periapsis_deg(T::from_f64(114.20783).unwrap())
			.with_long_of_ascending_node_deg(T::from_f64(-11.26064).unwrap());
		let earth_entry = DatabaseEntry::new(earth_info)
			.with_parent(sun_handle.clone(), earth_orbit)
			.with_mean_anomaly_deg(T::from_f64(358.617).unwrap());
		db.add_entry(earth_handle, earth_entry);
		// return database
		db
	}
	/// Adds a new entry to the database
	pub fn add_entry(&mut self, handle: H, entry: DatabaseEntry<H, T>) {
		self.bodies.insert(handle, entry);
	}
	/// Gets the entry from the database with the given handle
	pub fn get_entry(&self, handle: H) -> &DatabaseEntry<H, T> {
		self.bodies.get(&handle).unwrap()
	}
	/// Gets the position of the given body at the given mean anomaly value
	pub fn position_at_mean_anomaly(&self, handle: H, mean_anomaly: T) -> Vector3<T> where T: RealField + SimdValue + SimdRealField {
		let zero = T::from_f64(0.0).unwrap();
		let one = T::from_f64(1.0).unwrap();
		let two = T::from_f64(2.0).unwrap();
		let orbiting_body = self.bodies.get(&handle).unwrap();
		if let Some(orbit) = &orbiting_body.orbit {
			let parent_body = self.bodies.get(&orbiting_body.parent.clone().unwrap()).unwrap();
			let true_anomaly = mean_anomaly + two * orbit.eccentricity * Float::sin(mean_anomaly) + T::from_f64(1.25).unwrap() * Float::powi(orbit.eccentricity, 2) * Float::sin(two * mean_anomaly);
			let radius = orbit.semimajor_axis * (one - Float::powi(orbit.eccentricity, 2)) / (one + orbit.eccentricity * Float::cos(true_anomaly));
			let scale = parent_body.scale;
			let game_radius = radius * scale;
			let rot_true_anomaly = Rotation3::new(Vector3::new(zero, one, zero) * true_anomaly);
			let rot_long_of_ascending_node = Rotation3::new(Vector3::new(zero, one, zero) * orbit.long_of_ascending_node);
			let dir_ascending_node = rot_long_of_ascending_node * Vector3::new(one, zero, zero);
			let dir_normal = Vector3::new(one, zero, zero).cross(&dir_ascending_node);
			let rot_inclination = Rotation3::new(dir_ascending_node * orbit.inclination);
			let rot_arg_of_periapsis = Rotation3::new(dir_normal * orbit.arg_of_periapsis);
			let direction = rot_inclination * rot_arg_of_periapsis * rot_true_anomaly * Vector3::new(one, zero, zero);
			return direction * game_radius;
		} else {
			return Vector3::new(zero, zero, zero);
		}
	}
	/// Calculate the radius of the sphere of influence of the body with the given handle
	pub fn radius_soi(&self, handle: H) -> T {
		let orbiting_body = self.bodies.get(&handle).unwrap();
		let orbiting_body_info = orbiting_body.info.clone();
		if let Some(orbit) = &orbiting_body.orbit {
			let parent_body = self.bodies.get(&orbiting_body.parent.clone().unwrap()).unwrap();
			let parent_body_info = parent_body.info.clone();
			let exponent = T::from_f64(2.0 / 5.0).unwrap();
			return orbit.semimajor_axis * (orbiting_body_info.mass_kg() / parent_body_info.mass_kg()).powf(exponent);
		} else {
			let minimum_gravity = T::from_f64(0.0000005).unwrap();
			return orbiting_body_info.distance_of_gravity(minimum_gravity);
		}
	}
	pub fn iter(&self) -> Iter<'_, H, DatabaseEntry<H, T>> {
		self.bodies.iter()
	}
}
impl<H, T> Default for Database<H, T> {
	fn default() -> Self {
		Self{ bodies: HashMap::new() }
	}
}


pub struct DatabaseEntry<H, T> {
	pub parent: Option<H>,
	pub info: Body<T>,
	pub orbit: Option<OrbitalElements<T>>,
	pub mean_anomaly_at_epoch: T,
	pub scale: T,
}
impl<H, T> DatabaseEntry<H, T> where T: FromPrimitive + Mul<T, Output=T> {
	pub fn new(info: Body<T>) -> Self {
		Self{
			info,
			parent: None, orbit: None, mean_anomaly_at_epoch: T::from_f64(0.0).unwrap(),
			scale: T::from_f64(1.0 / 3_000_000.0).unwrap(),
		}
	}
	pub fn with_parent(mut self, parent_handle: H, orbital_elements: OrbitalElements<T>) -> Self {
		self.parent = Some(parent_handle);
		self.orbit = Some(orbital_elements);
		self
	}
	pub fn with_scale(mut self, scale: T) -> Self {
		self.scale = scale;
		self
	}
	pub fn with_mean_anomaly_deg(mut self, mean_anomaly: T) -> Self {
		self.mean_anomaly_at_epoch = mean_anomaly * T::from_f64(CONVERT_DEG_TO_RAD).unwrap();
		self
	}
}