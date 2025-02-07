use std::{collections::{hash_map::Iter, HashMap}, hash::Hash, ops::Mul};
use nalgebra::{RealField, Rotation3, SimdRealField, SimdValue, Vector3};
use num_traits::{Float, FromPrimitive};
use crate::{constants::f64::CONVERT_DEG_TO_RAD, Body, OrbitalElements};

#[cfg(feature="bevy")]
use bevy::prelude::*;

pub mod handles
{
	pub const HANDLE_SOL: u16 = 0;
	pub const HANDLE_MERCURY: u16 = 1;
	pub const HANDLE_VENUS: u16 = 2;
	pub const HANDLE_EARTH: u16 = 3;
	pub const HANDLE_LUNA: u16 = 4;
	pub const HANDLE_MARS: u16 = 5;
	pub const HANDLE_PHOBOS: u16 = 6;
	pub const HANDLE_DEIMOS: u16 = 7;
	pub const HANDLE_JUPITER: u16 = 8;
	pub const HANDLE_IO: u16 = HANDLE_JUPITER + 1;
	pub const HANDLE_EUROPA: u16 = HANDLE_JUPITER + 2;
	pub const HANDLE_GANYMEDE: u16 = HANDLE_JUPITER + 3;
	pub const HANDLE_CALLISTO: u16 = HANDLE_JUPITER + 4;
	pub const HANDLE_AMALTHEA: u16 = HANDLE_JUPITER + 5;
	pub const HANDLE_HIMALIA: u16 = HANDLE_JUPITER + 6;
	pub const HANDLE_ELARA: u16 = HANDLE_JUPITER + 7;
	pub const HANDLE_PASIPHAE: u16 = HANDLE_JUPITER + 8;
	pub const HANDLE_SINOPE: u16 = HANDLE_JUPITER + 9;
	pub const HANDLE_LYSITHAE: u16 = HANDLE_JUPITER + 10;
	pub const HANDLE_CARME: u16 = HANDLE_JUPITER + 11;
	pub const HANDLE_ANANKE: u16 = HANDLE_JUPITER + 12;
	pub const HANDLE_LEDA: u16 = HANDLE_JUPITER + 13;
	pub const HANDLE_THEBE: u16 = HANDLE_JUPITER + 14;
	pub const HANDLE_ADRASTEA: u16 = HANDLE_JUPITER + 15;
	pub const HANDLE_METIS: u16 = HANDLE_JUPITER + 16;
	pub const HANDLE_CALLIRHOE: u16 = HANDLE_JUPITER + 17;
	pub const HANDLE_THEMISTO: u16 = HANDLE_JUPITER + 18;
	pub const HANDLE_CARPO: u16 = HANDLE_JUPITER + 46;
	pub const HANDLE_EIRENE: u16 = HANDLE_JUPITER + 57;
	pub const HANDLE_PHILOPHROSYNE: u16 = HANDLE_JUPITER + 59;
	pub const HANDLE_EUPHEME: u16 = HANDLE_JUPITER + 60;
	pub const HANDLE_VALETUDO: u16 = HANDLE_JUPITER + 62;
	pub const HANDLE_PANDIA: u16 = HANDLE_JUPITER + 65;
	pub const HANDLE_ERSA: u16 = HANDLE_JUPITER + 71;
	pub const HANDLE_S_2011_J_1: u16 = HANDLE_JUPITER + 72;
	pub const HANDLE_SATURN: u16 = 104;
}

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
	/// populates the database with celestial bodies from our solar system
	pub fn with_solar_system(self) -> Self {
		self.with_sol()
			.with_mercury()
			.with_venus()
			.with_earth()
			.with_mars()
			.with_jupiter()
			.with_saturn()
	}
	/// Adds our sun to the database
	pub fn with_sol(mut self) -> Self {
		let sun_handle = H::from_u16(handles::HANDLE_SOL).unwrap();
		let sun_info: Body<T> = Body::new_sol();
		let sun_entry = DatabaseEntry::new(sun_info).with_scale(T::from_f64(1.0 / 100_000_000.0).unwrap());
		self.add_entry(sun_handle.clone(), sun_entry);
		self
	}
	/// Adds the planet mercury to the database
	pub fn with_mercury(mut self) -> Self {
		let sun_handle = H::from_u16(handles::HANDLE_SOL).unwrap();
		let mercury_handle = H::from_u16(handles::HANDLE_MERCURY).unwrap();
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
		self.add_entry(mercury_handle, mercury_entry);
		self
	}
	/// Adds the planet venus to the database
	pub fn with_venus(mut self) -> Self {
		let sun_handle = H::from_u16(handles::HANDLE_SOL).unwrap();
		let venus_handle = H::from_u16(handles::HANDLE_VENUS).unwrap();
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
		self.add_entry(venus_handle, venus_entry);
		self
	}
	pub fn with_earth(mut self) -> Self {
		let sun_handle = H::from_u16(handles::HANDLE_SOL).unwrap();
		let earth_handle = H::from_u16(handles::HANDLE_EARTH).unwrap();
		let earth_info: Body<T> = Body::new_earth();
		let earth_orbit: OrbitalElements<T> = OrbitalElements::default()
			.with_semimajor_axis_km(T::from_f64(149_598_023.0).unwrap())
			.with_eccentricity(T::from_f64(0.0167086).unwrap())
			.with_inclination_deg(T::from_f64(0.00005).unwrap())
			.with_arg_of_periapsis_deg(T::from_f64(114.20783).unwrap())
			.with_long_of_ascending_node_deg(T::from_f64(-11.26064).unwrap());
		let earth_entry = DatabaseEntry::new(earth_info)
			.with_parent(sun_handle.clone(), earth_orbit)
			.with_mean_anomaly_deg(T::from_f64(358.617).unwrap());
		self.add_entry(earth_handle.clone(), earth_entry);
		let moon_handle = H::from_u16(handles::HANDLE_LUNA).unwrap();
		let moon_info: Body<T> = Body::default()
			.with_mass_kg(T::from_f64(7.346e22).unwrap())
			.with_radius_km(T::from_f64(1737.4).unwrap());
		let moon_orbit: OrbitalElements<T> = OrbitalElements::default()
			.with_semimajor_axis_km(T::from_f64(384_399.0).unwrap())
			.with_eccentricity(T::from_f64(0.0549).unwrap())
			.with_inclination_deg(T::from_f64(5.145).unwrap())
			.with_arg_of_periapsis_deg(T::from_f64(114.20783).unwrap())
			.with_long_of_ascending_node_deg(T::from_f64(-11.26064).unwrap());
		let moon_entry = DatabaseEntry::new(moon_info)
			.with_parent(earth_handle.clone(), moon_orbit)
			.with_mean_anomaly_deg(T::from_f64(90.0).unwrap());
		self.add_entry(moon_handle, moon_entry);
		self
	}
	pub fn with_mars(mut self) -> Self {
		let sun_handle = H::from_u16(handles::HANDLE_SOL).unwrap();
		// mars
		let mars_handle = H::from_u16(handles::HANDLE_MARS).unwrap();
		let mars_info: Body<T> = Body::default()
			.with_mass_kg(T::from_f64(6.4171e23).unwrap())
			.with_radius_km(T::from_f64(3389.5).unwrap());
		let mars_orbit: OrbitalElements<T> = OrbitalElements::default()
			.with_semimajor_axis_km(T::from_f64(227_939_366.0).unwrap())
			.with_eccentricity(T::from_f64(0.0934).unwrap())
			.with_inclination_deg(T::from_f64(1.850).unwrap())
			.with_arg_of_periapsis_deg(T::from_f64(286.5).unwrap())
			.with_long_of_ascending_node_deg(T::from_f64(49.57854).unwrap());
		let mars_entry = DatabaseEntry::new(mars_info)
			.with_parent(sun_handle.clone(), mars_orbit)
			.with_mean_anomaly_deg(T::from_f64(174.796).unwrap());
		self.add_entry(mars_handle.clone(), mars_entry);
		// phobos
		let phobos_handle = H::from_u16(handles::HANDLE_PHOBOS).unwrap();
		let phobos_info: Body<T> = Body::default()
			.with_mass_kg(T::from_f64(1.060e16).unwrap())
			.with_radius_km(T::from_f64(11.08).unwrap());
		let phobos_orbit: OrbitalElements<T> = OrbitalElements::default()
			.with_semimajor_axis_km(T::from_f64(9376.0).unwrap())
			.with_eccentricity(T::from_f64(0.0151).unwrap())
			.with_inclination_deg(T::from_f64(1.093).unwrap())
			.with_arg_of_periapsis_deg(T::from_f64(381.5236635).unwrap())
			.with_long_of_ascending_node_deg(T::from_f64(83.14323972).unwrap());
		let phobos_entry = DatabaseEntry::new(phobos_info)
			.with_parent(mars_handle.clone(), phobos_orbit)
			.with_mean_anomaly_deg(T::from_f64(90.0).unwrap());
		self.add_entry(phobos_handle, phobos_entry);
		// deimos
		let deimos_handle = H::from_u16(handles::HANDLE_DEIMOS).unwrap();
		let deimos_info: Body<T> = Body::default()
			.with_mass_kg(T::from_f64(1.060e16).unwrap())
			.with_radius_km(T::from_f64(11.08).unwrap());
		let deimos_orbit: OrbitalElements<T> = OrbitalElements::default()
			.with_semimajor_axis_km(T::from_f64(23463.2).unwrap())
			.with_eccentricity(T::from_f64(0.00033).unwrap())
			.with_inclination_deg(T::from_f64(0.93).unwrap())
			.with_arg_of_periapsis_deg(T::from_f64(386.1935449).unwrap())
			.with_long_of_ascending_node_deg(T::from_f64(80.97357149).unwrap());
		let deimos_entry = DatabaseEntry::new(deimos_info)
			.with_parent(mars_handle.clone(), deimos_orbit)
			.with_mean_anomaly_deg(T::from_f64(270.0).unwrap());
		self.add_entry(deimos_handle, deimos_entry);
		// return
		self
	}
	/// Adds the planet jupiter to the database, with its moons
	/// 
	/// Referencing wikipedia's [list of Jupiter's moons](https://en.wikipedia.org/wiki/Moons_of_Jupiter#List)
	pub fn with_jupiter(mut self) -> Self {
		let sun_handle = H::from_u16(handles::HANDLE_SOL).unwrap();
		// jupiter
		let jupiter_handle = H::from_u16(handles::HANDLE_JUPITER).unwrap();
		let jupiter_info: Body<T> = Body::default()
			.with_mass_kg(T::from_f64(1.8982e27).unwrap())
			.with_radius_km(T::from_f64(69911.5).unwrap());
		let jupiter_orbit: OrbitalElements<T> = OrbitalElements::default()
			.with_semimajor_axis_au(T::from_f64(5.2038).unwrap())
			.with_eccentricity(T::from_f64(0.0489).unwrap())
			.with_inclination_deg(T::from_f64(1.303).unwrap())
			.with_arg_of_periapsis_deg(T::from_f64(273.867).unwrap())
			.with_long_of_ascending_node_deg(T::from_f64(100.464).unwrap());
		let jupiter_entry = DatabaseEntry::new(jupiter_info)
			.with_parent(sun_handle.clone(), jupiter_orbit)
			.with_mean_anomaly_deg(T::from_f64(20.020).unwrap());
		self.add_entry(jupiter_handle.clone(), jupiter_entry);
		// Io
		let io_handle = H::from_u16(handles::HANDLE_IO).unwrap();
		let io_info: Body<T> = Body::default()
			.with_mass_kg(T::from_f64(8.93e22).unwrap())
			.with_radius_km(T::from_f64(1821.6).unwrap());
		let io_orbit: OrbitalElements<T> = OrbitalElements::default()
			.with_semimajor_axis_m(T::from_f64(422025278.692653).unwrap())
			.with_eccentricity(T::from_f64(0.00418867166362767).unwrap())
			.with_inclination_deg(T::from_f64(2.18312929).unwrap())
			.with_arg_of_periapsis_deg(T::from_f64(654.3518983).unwrap())
			.with_long_of_ascending_node_deg(T::from_f64(737.1542087).unwrap());
		let io_entry = DatabaseEntry::new(io_info)
			.with_parent(jupiter_handle.clone(), io_orbit)
			.with_mean_anomaly_deg(T::from_f64(90.0).unwrap());
		self.add_entry(io_handle, io_entry);
		// Europa
		let europa_handle = H::from_u16(handles::HANDLE_EUROPA).unwrap();
		let europa_info: Body<T> = Body::default()
			.with_mass_kg(T::from_f64(4.8e22).unwrap())
			.with_radius_m(T::from_f64(1565000.0).unwrap());
		let europa_orbit: OrbitalElements<T> = OrbitalElements::default()
			.with_semimajor_axis_m(T::from_f64(671193628.654398).unwrap())
			.with_eccentricity(T::from_f64(0.00940288418380329).unwrap())
			.with_inclination_deg(T::from_f64(2.216347171).unwrap())
			.with_arg_of_periapsis_deg(T::from_f64(468.8993005).unwrap())
			.with_long_of_ascending_node_deg(T::from_f64(350.5260572).unwrap());
		let europa_entry = DatabaseEntry::new(europa_info)
			.with_parent(jupiter_handle.clone(), europa_orbit)
			.with_mean_anomaly_deg(T::from_f64(270.0).unwrap());
		self.add_entry(europa_handle, europa_entry);
		// Ganymede
		let ganymede_handle = H::from_u16(handles::HANDLE_GANYMEDE).unwrap();
		let ganymede_info: Body<T> = Body::default()
			.with_mass_kg(T::from_f64(1.48e23).unwrap())
			.with_radius_km(T::from_f64(2634.0).unwrap());
		let ganymede_orbit: OrbitalElements<T> = OrbitalElements::default()
			.with_semimajor_axis_m(T::from_f64(1070615470.44541).unwrap())
			.with_eccentricity(T::from_f64(0.00158762974782861).unwrap())
			.with_inclination_deg(T::from_f64(2.0381662).unwrap())
			.with_arg_of_periapsis_deg(T::from_f64(621.291691).unwrap())
			.with_long_of_ascending_node_deg(T::from_f64(341.6959921).unwrap());
		let ganymede_entry = DatabaseEntry::new(ganymede_info)
			.with_parent(jupiter_handle.clone(), ganymede_orbit)
			.with_mean_anomaly_deg(T::from_f64(270.0).unwrap());
		self.add_entry(ganymede_handle, ganymede_entry);
		// Callisto
		let callisto_handle = H::from_u16(handles::HANDLE_CALLISTO).unwrap();
		let callisto_info: Body<T> = Body::default()
			.with_mass_kg(T::from_f64(1.48e23).unwrap())
			.with_radius_km(T::from_f64(2403.000).unwrap());
		let callisto_orbit: OrbitalElements<T> = OrbitalElements::default()
			.with_semimajor_axis_m(T::from_f64(1070615470.44541).unwrap())
			.with_eccentricity(T::from_f64(0.00158762974782861).unwrap())
			.with_inclination_deg(T::from_f64(2.0381662).unwrap())
			.with_arg_of_periapsis_deg(T::from_f64(621.291691).unwrap())
			.with_long_of_ascending_node_deg(T::from_f64(341.6959921).unwrap());
		let callisto_entry = DatabaseEntry::new(callisto_info)
			.with_parent(jupiter_handle.clone(), callisto_orbit)
			.with_mean_anomaly_deg(T::from_f64(270.0).unwrap());
		self.add_entry(callisto_handle, callisto_entry);
		// Amalthea
		let amalthea_handle = H::from_u16(handles::HANDLE_AMALTHEA).unwrap();
		let amalthea_info: Body<T> = Body::default()
			.with_mass_kg(T::from_f64(7.17e18).unwrap())
			.with_radius_km(T::from_f64(101.000).unwrap());
		let amalthea_orbit: OrbitalElements<T> = OrbitalElements::default()
			.with_semimajor_axis_m(T::from_f64(181159256.1).unwrap())
			.with_eccentricity(T::from_f64(0.000441428663648964).unwrap())
			.with_inclination_deg(T::from_f64(2.55350793607894).unwrap())
			.with_arg_of_periapsis_deg(T::from_f64(414.339943282274).unwrap())
			.with_long_of_ascending_node_deg(T::from_f64(342.032315906764).unwrap());
		let amalthea_entry = DatabaseEntry::new(amalthea_info)
			.with_parent(jupiter_handle.clone(), amalthea_orbit)
			.with_mean_anomaly_deg(T::from_f64(270.0).unwrap());
		self.add_entry(amalthea_handle, amalthea_entry);
		// Himalia
		let himalia_handle = H::from_u16(handles::HANDLE_HIMALIA).unwrap();
		let himalia_info: Body<T> = Body::default()
			.with_mass_kg(T::from_f64(9.56e18).unwrap())
			.with_radius_km(T::from_f64(93.150).unwrap());
		let himalia_orbit: OrbitalElements<T> = OrbitalElements::default()
			.with_semimajor_axis_m(T::from_f64(11394679431.4089).unwrap())
			.with_eccentricity(T::from_f64(0.148020288964713).unwrap())
			.with_inclination_deg(T::from_f64(30.4865631823591).unwrap())
			.with_arg_of_periapsis_deg(T::from_f64(405.592890277337).unwrap())
			.with_long_of_ascending_node_deg(T::from_f64(57.7865255776614).unwrap());
		let himalia_entry = DatabaseEntry::new(himalia_info)
			.with_parent(jupiter_handle.clone(), himalia_orbit)
			.with_mean_anomaly_deg(T::from_f64(270.0).unwrap());
		self.add_entry(himalia_handle, himalia_entry);
		// Elara
		let elara_handle = H::from_u16(handles::HANDLE_ELARA).unwrap();
		let elara_info: Body<T> = Body::default()
			.with_mass_kg(T::from_f64(7.77e17).unwrap())
			.with_radius_km(T::from_f64(38.500).unwrap());
		let elara_orbit: OrbitalElements<T> = OrbitalElements::default()
			.with_semimajor_axis_m(T::from_f64(11724775187.5364).unwrap())
			.with_eccentricity(T::from_f64(0.196015925266734).unwrap())
			.with_inclination_deg(T::from_f64(29.645438545611).unwrap())
			.with_long_of_ascending_node_deg(T::from_f64(104.680792927026).unwrap())
			.with_arg_of_periapsis_deg(T::from_f64(254.812870711218).unwrap());
		let elara_entry = DatabaseEntry::new(elara_info)
			.with_parent(jupiter_handle.clone(), elara_orbit)
			.with_mean_anomaly_deg(T::from_f64(270.0).unwrap());
		self.add_entry(elara_handle, elara_entry);
		// Pasiphae
		let pasiphae_handle = H::from_u16(handles::HANDLE_PASIPHAE).unwrap();
		let pasiphae_info: Body<T> = Body::default()
			.with_mass_kg(T::from_f64(1.91e17).unwrap())
			.with_radius_km(T::from_f64(25.700).unwrap());
		let pasiphae_orbit: OrbitalElements<T> = OrbitalElements::default()
			.with_semimajor_axis_m(T::from_f64(23398199225.7693).unwrap())
			.with_eccentricity(T::from_f64(0.36953258321634).unwrap())
			.with_inclination_deg(T::from_f64(141.719099777028).unwrap())
			.with_long_of_ascending_node_deg(T::from_f64(333.722656460893).unwrap())
			.with_arg_of_periapsis_deg(T::from_f64(529.781057110863).unwrap());
		let pasiphae_entry = DatabaseEntry::new(pasiphae_info)
			.with_parent(jupiter_handle.clone(), pasiphae_orbit)
			.with_mean_anomaly_deg(T::from_f64(270.0).unwrap());
		self.add_entry(pasiphae_handle, pasiphae_entry);
		// return
		self
	}
	/// Adds the planet Saturn to the database, with its moons
	/// 
	/// References wikipedia's [list of Saturn's moons](https://en.wikipedia.org/wiki/Moons_of_Saturn#List)
	pub fn with_saturn(mut self) -> Self {
		let sun_handle = H::from_u16(handles::HANDLE_SOL).unwrap();
		// saturn
		let saturn_handle = H::from_u16(handles::HANDLE_SATURN).unwrap();
		let saturn_info: Body<T> = Body::default()
			.with_mass_kg(T::from_f64(5.6834e26).unwrap())
			.with_radius_km(T::from_f64(69911.5).unwrap());
		let saturn_orbit: OrbitalElements<T> = OrbitalElements::default()
			.with_semimajor_axis_au(T::from_f64(9.5826).unwrap())
			.with_eccentricity(T::from_f64(0.0565).unwrap())
			.with_inclination_deg(T::from_f64(2.485).unwrap())
			.with_arg_of_periapsis_deg(T::from_f64(339.392).unwrap())
			.with_long_of_ascending_node_deg(T::from_f64(113.665).unwrap());
		let saturn_entry = DatabaseEntry::new(saturn_info)
			.with_parent(sun_handle.clone(), saturn_orbit)
			.with_mean_anomaly_deg(T::from_f64(317.020).unwrap());
		self.add_entry(saturn_handle.clone(), saturn_entry);
		// return
		self
	}
	/// Adds a new entry to the database
	pub fn add_entry(&mut self, handle: H, entry: DatabaseEntry<H, T>) {
		self.bodies.insert(handle, entry);
	}
	/// Gets the entry from the database with the given handle
	pub fn get_entry(&self, handle: &H) -> &DatabaseEntry<H, T> {
		self.bodies.get(handle).unwrap()
	}
	/// Gets the position of the given body at the given mean anomaly value
	pub fn position_at_mean_anomaly(&self, handle: &H, mean_anomaly: T) -> Vector3<T> where T: RealField + SimdValue + SimdRealField {
		let zero = T::from_f32(0.0).unwrap();
		let one = T::from_f32(1.0).unwrap();
		let two = T::from_f32(2.0).unwrap();
		let orbiting_body = self.bodies.get(&handle).unwrap();
		if let Some(orbit) = &orbiting_body.orbit {
			let true_anomaly = mean_anomaly + two * orbit.eccentricity * Float::sin(mean_anomaly) + T::from_f64(1.25).unwrap() * Float::powi(orbit.eccentricity, 2) * Float::sin(two * mean_anomaly);
			let radius = orbit.semimajor_axis * (one - Float::powi(orbit.eccentricity, 2)) / (one + orbit.eccentricity * Float::cos(true_anomaly));
			let rot_true_anomaly = Rotation3::new(Vector3::new(zero, one, zero) * true_anomaly);
			let rot_long_of_ascending_node = Rotation3::new(Vector3::new(zero, one, zero) * orbit.long_of_ascending_node);
			let dir_ascending_node = rot_long_of_ascending_node * Vector3::new(one, zero, zero);
			let dir_normal = Vector3::new(one, zero, zero).cross(&dir_ascending_node);
			let rot_inclination = Rotation3::new(dir_ascending_node * orbit.inclination);
			let rot_arg_of_periapsis = Rotation3::new(dir_normal * orbit.arg_of_periapsis);
			let direction = rot_inclination * rot_arg_of_periapsis * rot_true_anomaly * Vector3::new(one, zero, zero);
			return direction * radius;
		} else {
			return Vector3::new(zero, zero, zero);
		}
	}
	pub fn absolute_position_at_time(&self, handle: &H, time: T) -> Vector3<T> where T: RealField + SimdValue + SimdRealField {
		let zero = T::from_f32(0.0).unwrap();
		if let Some(entry) = self.bodies.get(&handle) {
			let mean_anomaly = entry.mean_anomaly_at_epoch;
			let parent_position = match &entry.parent {
				Some(parent_handle) => self.absolute_position_at_time(parent_handle, time),
				None => Vector3::new(zero, zero, zero),
			};
			return self.position_at_mean_anomaly(handle, mean_anomaly) + parent_position;
		} else {
			return Vector3::new(zero, zero, zero);
		}
	}
	/// Calculate the radius of the sphere of influence of the body with the given handle
	pub fn radius_soi(&self, handle: &H) -> T {
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