use std::{collections::{hash_map::Iter, HashMap}, hash::Hash, ops::SubAssign};
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
	pub const HANDLE_LUNA: u16 = HANDLE_EARTH + 1;
	pub const HANDLE_MARS: u16 = HANDLE_EARTH + 2;
	pub const HANDLE_PHOBOS: u16 = HANDLE_MARS + 1;
	pub const HANDLE_DEIMOS: u16 = HANDLE_MARS + 2;
	pub const HANDLE_JUPITER: u16 = HANDLE_MARS + 3;
	pub const HANDLE_IO: u16 = HANDLE_JUPITER + 1;
	pub const HANDLE_EUROPA: u16 = HANDLE_JUPITER + 2;
	pub const HANDLE_GANYMEDE: u16 = HANDLE_JUPITER + 3;
	pub const HANDLE_CALLISTO: u16 = HANDLE_JUPITER + 4;
	pub const HANDLE_AMALTHEA: u16 = HANDLE_JUPITER + 5;
	pub const HANDLE_HIMALIA: u16 = HANDLE_JUPITER + 6;
	pub const HANDLE_ELARA: u16 = HANDLE_JUPITER + 7;
	pub const HANDLE_PASIPHAE: u16 = HANDLE_JUPITER + 8;
	pub const HANDLE_SINOPE: u16 = HANDLE_JUPITER + 9;
	pub const HANDLE_LYSITHEA: u16 = HANDLE_JUPITER + 10;
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
	pub const HANDLE_SATURN: u16 = HANDLE_JUPITER + 97;
	pub const HANDLE_MIMAS: u16 = HANDLE_SATURN + 1;
	pub const HANDLE_ENCELADUS: u16 = HANDLE_SATURN + 2;
	pub const HANDLE_TETHYS: u16 = HANDLE_SATURN + 3;
	pub const HANDLE_DIONE: u16 = HANDLE_SATURN + 4;
	pub const HANDLE_RHEA: u16 = HANDLE_SATURN + 5;
	pub const HANDLE_TITAN: u16 = HANDLE_SATURN + 6;
	pub const HANDLE_HYPERION: u16 = HANDLE_SATURN + 7;
	pub const HANDLE_IAPETUS: u16 = HANDLE_SATURN + 8;
	pub const HANDLE_PHOEBE: u16 = HANDLE_SATURN + 9;
	pub const HANDLE_JANUS: u16 = HANDLE_SATURN + 10;
	pub const HANDLE_GEIRROD: u16 = HANDLE_SATURN + 66;
	pub const HANDLE_URANUS: u16 = HANDLE_SATURN + 148;
	pub const HANDLE_ARIEL: u16 = HANDLE_URANUS + 1;
	pub const HANDLE_UMBRIEL: u16 = HANDLE_URANUS + 2;
	pub const HANDLE_TITANIA: u16 = HANDLE_URANUS + 3;
	pub const HANDLE_OBERON: u16 = HANDLE_URANUS + 4;
	pub const HANDLE_MIRANDA: u16 = HANDLE_URANUS + 5;
	pub const HANDLE_CUPID: u16 = HANDLE_URANUS + 27;
	pub const HANDLE_NEPTUNE: u16 = HANDLE_URANUS + 28;
	pub const HANDLE_TRITON: u16 = HANDLE_NEPTUNE + 1;
	pub const HANDLE_NEREID: u16 = HANDLE_NEPTUNE + 2;
	pub const HANDLE_NAIAD: u16 = HANDLE_NEPTUNE + 3;
	pub const HANDLE_THALASSA: u16 = HANDLE_NEPTUNE + 4;
	pub const HANDLE_DESPINA: u16 = HANDLE_NEPTUNE + 5;
	pub const HANDLE_GALATEA: u16 = HANDLE_NEPTUNE + 6;
	pub const HANDLE_LARISSA: u16 = HANDLE_NEPTUNE + 7;
	pub const HANDLE_PROTEUS: u16 = HANDLE_NEPTUNE + 8;
	pub const HANDLE_HALIMEDE: u16 = HANDLE_NEPTUNE + 9;
	pub const HANDLE_PSAMATHE: u16 = HANDLE_NEPTUNE + 10;
	pub const HANDLE_SAO: u16 = HANDLE_NEPTUNE + 11;
	pub const HANDLE_LAOMEDEIA: u16 = HANDLE_NEPTUNE + 12;
	pub const HANDLE_NESO: u16 = HANDLE_NEPTUNE + 13;
	pub const HANDLE_HIPPOCAMP: u16 = HANDLE_NEPTUNE + 14;
	pub const HANDLE_PLUTO: u16 = HANDLE_NEPTUNE + 17;
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
impl<H, T> Database<H, T> where H: Clone + Eq + Hash + FromPrimitive, T: Clone + Float + FromPrimitive + SubAssign {
	/// populates the database with celestial bodies from our solar system
	pub fn with_solar_system(self) -> Self {
		self.with_sol()
			.with_mercury()
			.with_venus()
			.with_earth()
			.with_mars()
			.with_jupiter()
			.with_saturn()
			.with_uranus()
			.with_neptune()
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
		// Sinope
		let sinope_handle = H::from_u16(handles::HANDLE_SINOPE).unwrap();
		let sinope_info: Body<T> = Body::default()
			.with_mass_kg(T::from_f64(7.77e16).unwrap())
			.with_radius_km(T::from_f64(18.1).unwrap());
		let sinope_orbit: OrbitalElements<T> = OrbitalElements::default()
			.with_semimajor_axis_m(T::from_f64(23731586385.2044).unwrap())
			.with_eccentricity(T::from_f64(0.286212248401311).unwrap())
			.with_inclination_deg(T::from_f64(153.516632270518).unwrap())
			.with_long_of_ascending_node_deg(T::from_f64(326.138400070621).unwrap())
			.with_arg_of_periapsis_deg(T::from_f64(330.01471478535).unwrap());
		let sinope_entry = DatabaseEntry::new(sinope_info)
			.with_parent(jupiter_handle.clone(), sinope_orbit)
			.with_mean_anomaly_deg(T::from_f64(578.187135014671).unwrap());
		self.add_entry(sinope_handle, sinope_entry);
		// Lysithea
		let lysithea_handle = H::from_u16(handles::HANDLE_LYSITHEA).unwrap();
		let lysithea_info: Body<T> = Body::default()
			.with_mass_kg(T::from_f64(7.77e16).unwrap())
			.with_radius_km(T::from_f64(18.2).unwrap());
		let lysithea_orbit: OrbitalElements<T> = OrbitalElements::default()
			.with_semimajor_axis_m(T::from_f64(11681680564.3821).unwrap())
			.with_eccentricity(T::from_f64(0.133982901517185).unwrap())
			.with_inclination_deg(T::from_f64(27.1161743142435).unwrap())
			.with_long_of_ascending_node_deg(T::from_f64(1.25211821789787).unwrap())
			.with_arg_of_periapsis_deg(T::from_f64(64.8726214272199).unwrap());
		let lysithea_entry = DatabaseEntry::new(lysithea_info)
			.with_parent(jupiter_handle.clone(), lysithea_orbit)
			.with_mean_anomaly_deg(T::from_f64(158.993906489824).unwrap());
		self.add_entry(lysithea_handle, lysithea_entry);
		// Carme
		let carme_handle = H::from_u16(handles::HANDLE_CARME).unwrap();
		let carme_info: Body<T> = Body::default()
			.with_mass_kg(T::from_f64(9.56e16).unwrap())
			.with_radius_km(T::from_f64(39.2).unwrap());
		let carme_orbit: OrbitalElements<T> = OrbitalElements::default()
			.with_semimajor_axis_m(T::from_f64(22846253568.9564).unwrap())
			.with_eccentricity(T::from_f64(0.222748653886903).unwrap())
			.with_inclination_deg(T::from_f64(164.964353121975).unwrap())
			.with_long_of_ascending_node_deg(T::from_f64(143.056427256701).unwrap())
			.with_arg_of_periapsis_deg(T::from_f64(199.239805499578).unwrap());
		let carme_entry = DatabaseEntry::new(carme_info)
			.with_parent(jupiter_handle.clone(), carme_orbit)
			.with_mean_anomaly_deg(T::from_f64(545.059221473009).unwrap());
		self.add_entry(carme_handle, carme_entry);
		// Ananke
		let ananke_handle = H::from_u16(handles::HANDLE_ANANKE).unwrap();
		let ananke_info: Body<T> = Body::default()
			.with_mass_kg(T::from_f64(3.82e16).unwrap())
			.with_radius_km(T::from_f64(14.9).unwrap());
		let ananke_orbit: OrbitalElements<T> = OrbitalElements::default()
			.with_semimajor_axis_m(T::from_f64(21178519961.0608).unwrap())
			.with_eccentricity(T::from_f64(0.360749649973783).unwrap())
			.with_inclination_deg(T::from_f64(151.631855563574).unwrap())
			.with_long_of_ascending_node_deg(T::from_f64(39.1941066220987).unwrap())
			.with_arg_of_periapsis_deg(T::from_f64(131.881909593109).unwrap());
		let ananke_entry = DatabaseEntry::new(ananke_info)
			.with_parent(jupiter_handle.clone(), ananke_orbit)
			.with_mean_anomaly_deg(T::from_f64(365.178243021899).unwrap());
		self.add_entry(ananke_handle, ananke_entry);
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
		// Mimas
		let mimas_handle = H::from_u16(handles::HANDLE_MIMAS).unwrap();
		let mimas_info: Body<T> = Body::default()
			.with_mass_kg(T::from_f64(3.8e19).unwrap())
			.with_radius_km(T::from_f64(196.000).unwrap());
		let mimas_orbit: OrbitalElements<T> = OrbitalElements::default()
			.with_semimajor_axis_m(T::from_f64(186037830.154953).unwrap())
			.with_eccentricity(T::from_f64(0.0215133482144328).unwrap())
			.with_inclination_deg(T::from_f64(29.18891093).unwrap())
			.with_long_of_ascending_node_deg(T::from_f64(167.3070822).unwrap())
			.with_arg_of_periapsis_deg(T::from_f64(449.3704298).unwrap());
		let mimas_entry = DatabaseEntry::new(mimas_info)
			.with_parent(saturn_handle.clone(), mimas_orbit)
			.with_mean_anomaly_deg(T::from_f64(772.976419).unwrap());
		self.add_entry(mimas_handle, mimas_entry);
		// Enceladus
		let enceladus_handle = H::from_u16(handles::HANDLE_ENCELADUS).unwrap();
		let enceladus_info: Body<T> = Body::default()
			.with_mass_kg(T::from_f64(7.3e19).unwrap())
			.with_radius_km(T::from_f64(249.000).unwrap());
		let enceladus_orbit: OrbitalElements<T> = OrbitalElements::default()
			.with_semimajor_axis_m(T::from_f64(238408052.167797).unwrap())
			.with_eccentricity(T::from_f64(0.000372459385461708).unwrap())
			.with_inclination_deg(T::from_f64(28.04279097).unwrap())
			.with_long_of_ascending_node_deg(T::from_f64(169.5204865).unwrap())
			.with_arg_of_periapsis_deg(T::from_f64(264.6781976).unwrap());
		let enceladus_entry = DatabaseEntry::new(enceladus_info)
			.with_parent(saturn_handle.clone(), enceladus_orbit)
			.with_mean_anomaly_deg(T::from_f64(384.1198896).unwrap());
		self.add_entry(enceladus_handle, enceladus_entry);
		// Tethys
		let tethys_handle = H::from_u16(handles::HANDLE_TETHYS).unwrap();
		let tethys_info: Body<T> = Body::default()
			.with_mass_kg(T::from_f64(6.22e20).unwrap())
			.with_radius_km(T::from_f64(530.000).unwrap());
		let tethys_orbit: OrbitalElements<T> = OrbitalElements::default()
			.with_semimajor_axis_m(T::from_f64(294982634.56239).unwrap())
			.with_eccentricity(T::from_f64(0.00107532665445937).unwrap())
			.with_inclination_deg(T::from_f64(26.97242049).unwrap())
			.with_long_of_ascending_node_deg(T::from_f64(169.1532561).unwrap())
			.with_arg_of_periapsis_deg(T::from_f64(496.8246271).unwrap());
		let tethys_entry = DatabaseEntry::new(tethys_info)
			.with_parent(saturn_handle.clone(), tethys_orbit)
			.with_mean_anomaly_deg(T::from_f64(502.6123366).unwrap());
		self.add_entry(tethys_handle, tethys_entry);
		// Dione
		let dione_handle = H::from_u16(handles::HANDLE_DIONE).unwrap();
		let dione_info: Body<T> = Body::default()
			.with_mass_kg(T::from_f64(1.05e21).unwrap())
			.with_radius_km(T::from_f64(560.000).unwrap());
		let dione_orbit: OrbitalElements<T> = OrbitalElements::default()
			.with_semimajor_axis_m(T::from_f64(377653774.68302).unwrap())
			.with_eccentricity(T::from_f64(0.00273184023667722).unwrap())
			.with_inclination_deg(T::from_f64(28.05084794).unwrap())
			.with_long_of_ascending_node_deg(T::from_f64(169.5723087).unwrap())
			.with_arg_of_periapsis_deg(T::from_f64(5080.2590124).unwrap());
		let dione_entry = DatabaseEntry::new(dione_info)
			.with_parent(saturn_handle.clone(), dione_orbit)
			.with_mean_anomaly_deg(T::from_f64(856.824114).unwrap());
		self.add_entry(dione_handle, dione_entry);
		// Rhea
		let rhea_handle = H::from_u16(handles::HANDLE_RHEA).unwrap();
		let rhea_info: Body<T> = Body::default()
			.with_mass_kg(T::from_f64(2.49e21).unwrap())
			.with_radius_km(T::from_f64(764.000).unwrap());
		let rhea_orbit: OrbitalElements<T> = OrbitalElements::default()
			.with_semimajor_axis_m(T::from_f64(527225476.502164).unwrap())
			.with_eccentricity(T::from_f64(0.000909561682184622).unwrap())
			.with_inclination_deg(T::from_f64(27.94971857).unwrap())
			.with_long_of_ascending_node_deg(T::from_f64(168.8079837).unwrap())
			.with_arg_of_periapsis_deg(T::from_f64(360.9692475).unwrap());
		let rhea_entry = DatabaseEntry::new(rhea_info)
			.with_parent(saturn_handle.clone(), rhea_orbit)
			.with_mean_anomaly_deg(T::from_f64(448.7342263).unwrap());
		self.add_entry(rhea_handle, rhea_entry);
		// Titan
		let titan_handle = H::from_u16(handles::HANDLE_TITAN).unwrap();
		let titan_info: Body<T> = Body::default()
			.with_mass_kg(T::from_f64(1.35e23).unwrap())
			.with_radius_km(T::from_f64(2575.000).unwrap());
		let titan_orbit: OrbitalElements<T> = OrbitalElements::default()
			.with_semimajor_axis_m(T::from_f64(1221971852.3956).unwrap())
			.with_eccentricity(T::from_f64(0.0286455635677465).unwrap())
			.with_inclination_deg(T::from_f64(27.71621075).unwrap())
			.with_long_of_ascending_node_deg(T::from_f64(169.1427802).unwrap())
			.with_arg_of_periapsis_deg(T::from_f64(336.2491384).unwrap());
		let titan_entry = DatabaseEntry::new(titan_info)
			.with_parent(saturn_handle.clone(), titan_orbit)
			.with_mean_anomaly_deg(T::from_f64(143.0542442).unwrap());
		self.add_entry(titan_handle, titan_entry);
		// Hyperion
		let hyperion_handle = H::from_u16(handles::HANDLE_HYPERION).unwrap();
		let hyperion_info: Body<T> = Body::default()
			.with_mass_kg(T::from_f64(1.77e19).unwrap())
			.with_radius_km(T::from_f64(143.000).unwrap());
		let hyperion_orbit: OrbitalElements<T> = OrbitalElements::default()
			.with_semimajor_axis_m(T::from_f64(1447200000.0).unwrap())
			.with_eccentricity(T::from_f64(0.0757).unwrap())
			.with_inclination_deg(T::from_f64(27.0344979012323).unwrap())
			.with_long_of_ascending_node_deg(T::from_f64(168.9).unwrap())
			.with_arg_of_periapsis_deg(T::from_f64(182.895).unwrap());
		let hyperion_entry = DatabaseEntry::new(hyperion_info)
			.with_parent(saturn_handle.clone(), hyperion_orbit)
			.with_mean_anomaly_deg(T::from_f64(301.6).unwrap());
		self.add_entry(hyperion_handle, hyperion_entry);
		// Iapetus
		let iapetus_handle = H::from_u16(handles::HANDLE_IAPETUS).unwrap();
		let iapetus_info: Body<T> = Body::default()
			.with_mass_kg(T::from_f64(1.6e22).unwrap())
			.with_radius_km(T::from_f64(730.000).unwrap());
		let iapetus_orbit: OrbitalElements<T> = OrbitalElements::default()
			.with_semimajor_axis_m(T::from_f64(3563513670.80278).unwrap())
			.with_eccentricity(T::from_f64(0.0274067153032204).unwrap())
			.with_inclination_deg(T::from_f64(17.25375588).unwrap())
			.with_long_of_ascending_node_deg(T::from_f64(139.3182554).unwrap())
			.with_arg_of_periapsis_deg(T::from_f64(369.2974058).unwrap());
		let iapetus_entry = DatabaseEntry::new(iapetus_info)
			.with_parent(saturn_handle.clone(), iapetus_orbit)
			.with_mean_anomaly_deg(T::from_f64(551.098555).unwrap());
		self.add_entry(iapetus_handle, iapetus_entry);
		// Phoebe
		let phoebe_handle = H::from_u16(handles::HANDLE_PHOEBE).unwrap();
		let phoebe_info: Body<T> = Body::default()
			.with_mass_kg(T::from_f64(7.8e15).unwrap())
			.with_radius_km(T::from_f64(3.000).unwrap());
		let phoebe_orbit: OrbitalElements<T> = OrbitalElements::default()
			.with_semimajor_axis_m(T::from_f64(12995759988.095).unwrap())
			.with_eccentricity(T::from_f64(0.0000156144511577606).unwrap())
			.with_inclination_deg(T::from_f64(0.0151781240198135).unwrap())
			.with_long_of_ascending_node_deg(T::from_f64(208.626701831817).unwrap())
			.with_arg_of_periapsis_deg(T::from_f64(104.242486953736).unwrap());
		let phoebe_entry = DatabaseEntry::new(phoebe_info)
			.with_parent(saturn_handle.clone(), phoebe_orbit)
			.with_mean_anomaly_deg(T::from_f64(108.701283931732).unwrap());
		self.add_entry(phoebe_handle, phoebe_entry);
		// Janus
		let janus_handle = H::from_u16(handles::HANDLE_JANUS).unwrap();
		let janus_info: Body<T> = Body::default()
			.with_mass_kg(T::from_f64(7.0e15).unwrap())
			.with_radius_km(T::from_f64(3.000).unwrap());
		let janus_orbit: OrbitalElements<T> = OrbitalElements::default()
			.with_semimajor_axis_m(T::from_f64(151460988.095).unwrap())
			.with_eccentricity(T::from_f64(0.0000000144511577606).unwrap())
			.with_inclination_deg(T::from_f64(0.000007105811891727).unwrap())
			.with_long_of_ascending_node_deg(T::from_f64(208.626701831817).unwrap())
			.with_arg_of_periapsis_deg(T::from_f64(104.242486953736).unwrap());
		let janus_entry = DatabaseEntry::new(janus_info)
			.with_parent(saturn_handle.clone(), janus_orbit)
			.with_mean_anomaly_deg(T::from_f64(108.701283931732).unwrap());
		self.add_entry(janus_handle, janus_entry);
		// return
		self
	}
	/// Adds Uranus and its moons to the system
	/// 
	/// References [Wikipedia's list of Uranian moons](https://en.wikipedia.org/wiki/Moons_of_Uranus#List)
	pub fn with_uranus(mut self) -> Self {
		let sun_handle = H::from_u16(handles::HANDLE_SOL).unwrap();
		// Uranus
		let uranus_handle = H::from_u16(handles::HANDLE_URANUS).unwrap();
		let uranus_info: Body<T> = Body::default()
			.with_mass_kg(T::from_f64(8.6810e25).unwrap())
			.with_radius_km(T::from_f64(25362.0).unwrap());
		let uranus_orbit: OrbitalElements<T> = OrbitalElements::default()
			.with_semimajor_axis_au(T::from_f64(19.19126).unwrap())
			.with_eccentricity(T::from_f64(0.04717).unwrap())
			.with_inclination_deg(T::from_f64(0.773).unwrap())
			.with_arg_of_periapsis_deg(T::from_f64(96.998857).unwrap())
			.with_long_of_ascending_node_deg(T::from_f64(74.006).unwrap());
		let uranus_entry = DatabaseEntry::new(uranus_info)
			.with_parent(sun_handle.clone(), uranus_orbit)
			.with_mean_anomaly_deg(T::from_f64(142.238600).unwrap());
		self.add_entry(uranus_handle.clone(), uranus_entry);
		// Ariel
		let ariel_handle = H::from_u16(handles::HANDLE_ARIEL).unwrap();
		let ariel_info: Body<T> = Body::default()
			.with_mass_kg(T::from_f64(1.27e21).unwrap())
			.with_radius_km(T::from_f64(578.9).unwrap());
		let ariel_orbit: OrbitalElements<T> = OrbitalElements::default()
			.with_semimajor_axis_m(T::from_f64(190940711.743871).unwrap())
			.with_eccentricity(T::from_f64(0.00137850353892181).unwrap())
			.with_inclination_deg(T::from_f64(97.79230874).unwrap())
			.with_long_of_ascending_node_deg(T::from_f64(167.6951854).unwrap())
			.with_arg_of_periapsis_deg(T::from_f64(236.6892802).unwrap());
		let ariel_entry = DatabaseEntry::new(ariel_info)
			.with_parent(uranus_handle.clone(), ariel_orbit)
			.with_mean_anomaly_deg(T::from_f64(583.1923962).unwrap());
		self.add_entry(ariel_handle, ariel_entry);
		// Umbriel
		let umbriel_handle = H::from_u16(handles::HANDLE_UMBRIEL).unwrap();
		let umbriel_info: Body<T> = Body::default()
			.with_mass_kg(T::from_f64(1.27e21).unwrap())
			.with_radius_km(T::from_f64(584.7).unwrap());
		let umbriel_orbit: OrbitalElements<T> = OrbitalElements::default()
			.with_semimajor_axis_m(T::from_f64(266004056.284577).unwrap())
			.with_eccentricity(T::from_f64(0.00436450298644918).unwrap())
			.with_inclination_deg(T::from_f64(97.682239322).unwrap())
			.with_long_of_ascending_node_deg(T::from_f64(167.7113413).unwrap())
			.with_arg_of_periapsis_deg(T::from_f64(521.5502336).unwrap());
		let umbriel_entry = DatabaseEntry::new(umbriel_info)
			.with_parent(uranus_handle.clone(), umbriel_orbit)
			.with_mean_anomaly_deg(T::from_f64(837.2597847).unwrap());
		self.add_entry(umbriel_handle, umbriel_entry);
		// Titania
		let titania_handle = H::from_u16(handles::HANDLE_TITANIA).unwrap();
		let titania_info: Body<T> = Body::default()
			.with_mass_kg(T::from_f64(3.49e21).unwrap())
			.with_radius_km(T::from_f64(788.9).unwrap());
		let titania_orbit: OrbitalElements<T> = OrbitalElements::default()
			.with_semimajor_axis_m(T::from_f64(436347342.837041).unwrap())
			.with_eccentricity(T::from_f64(0.00275764018002836).unwrap())
			.with_inclination_deg(T::from_f64(97.78930872).unwrap())
			.with_long_of_ascending_node_deg(T::from_f64(167.6116584).unwrap())
			.with_arg_of_periapsis_deg(T::from_f64(399.5640193).unwrap());
		let titania_entry = DatabaseEntry::new(titania_info)
			.with_parent(uranus_handle.clone(), titania_orbit)
			.with_mean_anomaly_deg(T::from_f64(496.5752932).unwrap());
		self.add_entry(titania_handle, titania_entry);
		// Oberon
		let oberon_handle = H::from_u16(handles::HANDLE_OBERON).unwrap();
		let oberon_info: Body<T> = Body::default()
			.with_mass_kg(T::from_f64(3.03e21).unwrap())
			.with_radius_km(T::from_f64(761.4).unwrap());
		let oberon_orbit: OrbitalElements<T> = OrbitalElements::default()
			.with_semimajor_axis_m(T::from_f64(583560909.561177).unwrap())
			.with_eccentricity(T::from_f64(0.00110658045344143).unwrap())
			.with_inclination_deg(T::from_f64(97.87882122).unwrap())
			.with_long_of_ascending_node_deg(T::from_f64(167.7422432).unwrap())
			.with_arg_of_periapsis_deg(T::from_f64(288.925047).unwrap());
		let oberon_entry = DatabaseEntry::new(oberon_info)
			.with_parent(uranus_handle.clone(), oberon_orbit)
			.with_mean_anomaly_deg(T::from_f64(472.6703921).unwrap());
		self.add_entry(oberon_handle, oberon_entry);
		// Miranda
		let miranda_handle = H::from_u16(handles::HANDLE_MIRANDA).unwrap();
		let miranda_info: Body<T> = Body::default()
			.with_mass_kg(T::from_f64(6.33e19).unwrap())
			.with_radius_km(T::from_f64(235.800).unwrap());
		let miranda_orbit: OrbitalElements<T> = OrbitalElements::default()
			.with_semimajor_axis_m(T::from_f64(129.87e6).unwrap())
			.with_eccentricity(T::from_f64(0.0014).unwrap())
			.with_inclination_deg(T::from_f64(96.44799101).unwrap())
			.with_long_of_ascending_node_deg(T::from_f64(163.4949965).unwrap())
			.with_arg_of_periapsis_deg(T::from_f64(242.2809905).unwrap());
		let miranda_entry = DatabaseEntry::new(miranda_info)
			.with_parent(uranus_handle.clone(), miranda_orbit)
			.with_mean_anomaly_deg(T::from_f64(143.0330121).unwrap());
		self.add_entry(miranda_handle, miranda_entry);
		// return
		self
	}
	/// Adds Neptune and its moons to the system
	/// 
	/// References [Wikipedia's list of Neptunian moons](https://en.wikipedia.org/wiki/Moons_of_Neptune#List)
	pub fn with_neptune(mut self) -> Self {
		let sun_handle = H::from_u16(handles::HANDLE_SOL).unwrap();
		// saturn
		let neptune_handle = H::from_u16(handles::HANDLE_NEPTUNE).unwrap();
		let neptune_info: Body<T> = Body::default()
			.with_mass_kg(T::from_f64(5.6834e26).unwrap())
			.with_radius_km(T::from_f64(69911.5).unwrap());
		let neptune_orbit: OrbitalElements<T> = OrbitalElements::default()
			.with_semimajor_axis_au(T::from_f64(30.07).unwrap())
			.with_eccentricity(T::from_f64(0.008678).unwrap())
			.with_inclination_deg(T::from_f64(1.770).unwrap())
			.with_arg_of_periapsis_deg(T::from_f64(273.187).unwrap())
			.with_long_of_ascending_node_deg(T::from_f64(131.783).unwrap());
		let neptune_entry = DatabaseEntry::new(neptune_info)
			.with_parent(sun_handle.clone(), neptune_orbit)
			.with_mean_anomaly_deg(T::from_f64(317.020).unwrap());
		self.add_entry(neptune_handle.clone(), neptune_entry);
		// Triton
		let triton_handle = H::from_u16(handles::HANDLE_TRITON).unwrap();
		let triton_info: Body<T> = Body::default()
			.with_mass_kg(T::from_f64(2.14e22).unwrap())
			.with_radius_km(T::from_f64(1352.500).unwrap());
		let triton_orbit: OrbitalElements<T> = OrbitalElements::default()
			.with_semimajor_axis_m(T::from_f64(354765668.747018).unwrap())
			.with_eccentricity(T::from_f64(0.0000177503155008841).unwrap())
			.with_inclination_deg(T::from_f64(129.9699061).unwrap())
			.with_long_of_ascending_node_deg(T::from_f64(217.2530657).unwrap())
			.with_arg_of_periapsis_deg(T::from_f64(521.6797862 - 360.0).unwrap());
		let triton_entry = DatabaseEntry::new(triton_info)
			.with_parent(neptune_handle.clone(), triton_orbit)
			.with_mean_anomaly_deg(T::from_f64(829.2581612).unwrap());
		self.add_entry(triton_handle, triton_entry);
		// Nereid
		let nereid_handle = H::from_u16(handles::HANDLE_NEREID).unwrap();
		let nereid_info: Body<T> = Body::default()
			.with_mass_kg(T::from_f64(1.317e19).unwrap())
			.with_radius_km(T::from_f64(165.0).unwrap());
		let nereid_orbit: OrbitalElements<T> = OrbitalElements::default()
			.with_semimajor_axis_m(T::from_f64(5515375933.0092).unwrap())
			.with_eccentricity(T::from_f64(0.747077257017379).unwrap())
			.with_inclination_deg(T::from_f64(5.0672309310494).unwrap())
			.with_long_of_ascending_node_deg(T::from_f64(320.104934616101).unwrap())
			.with_arg_of_periapsis_deg(T::from_f64(616.561942032962 - 360.0).unwrap());
		let nereid_entry = DatabaseEntry::new(nereid_info)
			.with_parent(neptune_handle.clone(), nereid_orbit)
			.with_mean_anomaly_deg(T::from_f64(684.0532414137 - 360.0).unwrap());
		self.add_entry(nereid_handle, nereid_entry);
		// Naiad
		let naiad_handle = H::from_u16(handles::HANDLE_NAIAD).unwrap();
		let naiad_info: Body<T> = Body::default()
			.with_mass_kg(T::from_f64(5.8e15).unwrap())
			.with_radius_km(T::from_f64(2.0).unwrap());
		let naiad_orbit: OrbitalElements<T> = OrbitalElements::default()
			.with_semimajor_axis_km(T::from_f64(48227784.2).unwrap())
			.with_eccentricity(T::from_f64(0.000000447511577606).unwrap())
			.with_inclination_deg(T::from_f64(0.0272397898144598).unwrap())
			.with_long_of_ascending_node_deg(T::from_f64(208.626701831817).unwrap())
			.with_arg_of_periapsis_deg(T::from_f64(104.242486953736).unwrap());
		let naiad_entry = DatabaseEntry::new(naiad_info)
			.with_parent(neptune_handle.clone(), naiad_orbit)
			.with_mean_anomaly_deg(T::from_f64(108.701283931732).unwrap());
		self.add_entry(naiad_handle, naiad_entry);
		// Thalassa
		let thalassa_handle = H::from_u16(handles::HANDLE_THALASSA).unwrap();
		let thalassa_info: Body<T> = Body::default()
			.with_mass_kg(T::from_f64(5.8e15).unwrap())
			.with_radius_km(T::from_f64(2.0).unwrap());
		let thalassa_orbit: OrbitalElements<T> = OrbitalElements::default()
			.with_semimajor_axis_km(T::from_f64(50141475.7560609).unwrap())
			.with_eccentricity(T::from_f64(0.001370609133743).unwrap())
			.with_inclination_deg(T::from_f64(28.635825609126).unwrap())
			.with_long_of_ascending_node_deg(T::from_f64(49.1486489463042).unwrap())
			.with_arg_of_periapsis_deg(T::from_f64(178.660268240832).unwrap());
		let thalassa_entry = DatabaseEntry::new(thalassa_info)
			.with_parent(neptune_handle.clone(), thalassa_orbit)
			.with_mean_anomaly_deg(T::from_f64(187.573079498586).unwrap());
		self.add_entry(thalassa_handle, thalassa_entry);
		// Despina
		let despina_handle = H::from_u16(handles::HANDLE_DESPINA).unwrap();
		let despina_info: Body<T> = Body::default()
			.with_mass_kg(T::from_f64(2.21e16).unwrap())
			.with_radius_km(T::from_f64(12.0).unwrap());
		let despina_orbit: OrbitalElements<T> = OrbitalElements::default()
			.with_semimajor_axis_km(T::from_f64(60227784.2).unwrap())
			.with_eccentricity(T::from_f64(0.0000000244511577606).unwrap())
			.with_inclination_deg(T::from_f64(0.001238965071423).unwrap())
			.with_long_of_ascending_node_deg(T::from_f64(208.626701831817).unwrap())
			.with_arg_of_periapsis_deg(T::from_f64(104.242486953736).unwrap());
		let despina_entry = DatabaseEntry::new(despina_info)
			.with_parent(neptune_handle.clone(), despina_orbit)
			.with_mean_anomaly_deg(T::from_f64(108.701283931732).unwrap());
		self.add_entry(despina_handle, despina_entry);
		// Galatea
		let galatea_handle = H::from_u16(handles::HANDLE_GALATEA).unwrap();
		let galatea_info: Body<T> = Body::default()
			.with_mass_kg(T::from_f64(5.955e16).unwrap())
			.with_radius_km(T::from_f64(79.1).unwrap());
		let galatea_orbit: OrbitalElements<T> = OrbitalElements::default()
			.with_semimajor_axis_km(T::from_f64(62097694.895992).unwrap())
			.with_eccentricity(T::from_f64(0.00176342814065272).unwrap())
			.with_inclination_deg(T::from_f64(28.5712798372164).unwrap())
			.with_long_of_ascending_node_deg(T::from_f64(48.6938364381423).unwrap())
			.with_arg_of_periapsis_deg(T::from_f64(188.29717200708).unwrap());
		let galatea_entry = DatabaseEntry::new(galatea_info)
			.with_parent(neptune_handle.clone(), galatea_orbit)
			.with_mean_anomaly_deg(T::from_f64(216.667607835566).unwrap());
		self.add_entry(galatea_handle, galatea_entry);
		// Larissa
		let larissa_handle = H::from_u16(handles::HANDLE_LARISSA).unwrap();
		let larissa_info: Body<T> = Body::default()
			.with_mass_kg(T::from_f64(8.563e16).unwrap())
			.with_radius_km(T::from_f64(99.96).unwrap());
		let larissa_orbit: OrbitalElements<T> = OrbitalElements::default()
			.with_semimajor_axis_km(T::from_f64(73591064.2683372).unwrap())
			.with_eccentricity(T::from_f64(0.001696576604903).unwrap())
			.with_inclination_deg(T::from_f64(28.3531487332235).unwrap())
			.with_long_of_ascending_node_deg(T::from_f64(48.9078558843833).unwrap())
			.with_arg_of_periapsis_deg(T::from_f64(378.844329275267).unwrap());
		let larissa_entry = DatabaseEntry::new(larissa_info)
			.with_parent(neptune_handle.clone(), larissa_orbit)
			.with_mean_anomaly_deg(T::from_f64(428.613425343462).unwrap());
		self.add_entry(larissa_handle, larissa_entry);
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
	/// Get a list of handles for satellites of the body with the input handle.
	pub fn get_satellites(&self, body: H) -> Vec<H> {
		let mut satellites: Vec<H> = Vec::new();
		for (handle, entry) in self.iter() {
			if let Some(parent_handle) = &entry.parent {
				if *parent_handle == body {
					satellites.push(handle.clone());
				}
			}
		}
		satellites
	}
	/// Get the heirarchy of parent bodies of the input body
	pub fn get_parents(&self, body: H) -> Vec<H> {
		let body_entry = self.get_entry(&body);
		if let Some(parent_handle) = &body_entry.parent {
			let mut heirarchy = self.get_parents(parent_handle.clone());
			heirarchy.push(body);
			return heirarchy;
		} else {
			return vec![body];
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
impl<H, T> DatabaseEntry<H, T> where T: Float + FromPrimitive + SubAssign {
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
		let circle = T::from_f64(360.0).unwrap();
		while self.mean_anomaly_at_epoch > circle {
			self.mean_anomaly_at_epoch -= circle;
		}
		self
	}
}


#[cfg(test)]
mod tests {
	use super::*;
	use super::handles::*;

	#[test]
	fn get_satellites() {
		let database = Database::<u16, f32>::default().with_solar_system();
		let satellites = database.get_satellites(HANDLE_EARTH);
		assert_eq!(1, satellites.len());
		assert!(satellites.contains(&HANDLE_LUNA));
		let satellites = database.get_satellites(HANDLE_MARS);
		assert_eq!(2, satellites.len());
		assert!(satellites.contains(&HANDLE_PHOBOS));
		assert!(satellites.contains(&HANDLE_DEIMOS));
	}

	#[test]
	fn get_parents() {
		let database = Database::<u16, f32>::default().with_solar_system();
		let heirarchy = database.get_parents(HANDLE_SOL);
		assert_eq!(1, heirarchy.len());
		assert_eq!(HANDLE_SOL, heirarchy[0]);
		let heirarchy = database.get_parents(HANDLE_MARS);
		assert_eq!(2, heirarchy.len());
		assert_eq!(HANDLE_SOL, heirarchy[0]);
		assert_eq!(HANDLE_MARS, heirarchy[1]);
		let heirarchy = database.get_parents(HANDLE_DEIMOS);
		assert_eq!(3, heirarchy.len());
		assert_eq!(HANDLE_SOL, heirarchy[0]);
		assert_eq!(HANDLE_MARS, heirarchy[1]);
		assert_eq!(HANDLE_DEIMOS, heirarchy[2]);
	}
}