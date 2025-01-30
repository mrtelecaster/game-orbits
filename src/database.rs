use std::{collections::HashMap, hash::Hash};
use num_traits::{Float, FromPrimitive};
use crate::{Body, OrbitalElements};

/// Holds the data for all the bodies being simulated
/// 
/// This is the main source of information for game engine implementations. The game engine should
/// feed its celestial body information into this database, and then query it to get the results of
/// calculations back.
/// 
/// `T` is the type used for the floating point data stored inside the database, and `H` is the
/// hashable type used for handles to celestial bodies which are used to retrieve a specific body
/// from the database and also define parent/child relationships
pub struct Database<H, T> {
	bodies: HashMap<H, DatabaseEntry<H, T>>,
}
impl<H, T> Database<H, T> where H: Clone + Eq + Hash + FromPrimitive, T: Float + FromPrimitive {
	/// Creates a new database pre-populated with celestial bodies from our solar system
	pub fn solar_system() -> Self {
		let mut db = Self::default();
		// sol/sun
		let sun_handle = H::from_u16(0).unwrap();
		let sun_info: Body<T> = Body::new_sol();
		let sun_entry = DatabaseEntry::new(sun_info);
		db.add_entry(sun_handle.clone(), sun_entry);
		// earth
		let earth_handle = H::from_u16(3).unwrap();
		let earth_info: Body<T> = Body::new_earth();
		let earth_orbit: OrbitalElements<T> = OrbitalElements::default()
			.with_semimajor_axis_km(T::from_f64(149598023.0).unwrap())
			.with_eccentricity(T::from_f64(0.0167086).unwrap())
			.with_inclination_deg(T::from_f64(0.00005).unwrap())
			.with_arg_of_periapsis_deg(T::from_f64(114.20783).unwrap())
			.with_long_of_ascending_node_deg(T::from_f64(-11.26064).unwrap());
		let earth_entry = DatabaseEntry::new(earth_info).with_parent(sun_handle.clone(), earth_orbit);
		db.add_entry(earth_handle, earth_entry);
		// return database
		db
	}
	/// Adds a new entry to the database
	pub fn add_entry(&mut self, handle: H, entry: DatabaseEntry<H, T>) {
		self.bodies.insert(handle, entry);
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
}
impl<H, T> DatabaseEntry<H, T> {
	pub fn new(info: Body<T>) -> Self {
		Self{ parent: None, info, orbit: None }
	}
	pub fn with_parent(mut self, parent_handle: H, orbital_elements: OrbitalElements<T>) -> Self {
		self.parent = Some(parent_handle);
		self.orbit = Some(orbital_elements);
		self
	}
}