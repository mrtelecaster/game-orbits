use std::ops::SubAssign;
use num_traits::{Float, FromPrimitive};
use crate::constants::f64::*;

/// Keplerian elements that define an orbit
#[derive(Clone, Copy)]
pub struct OrbitalElements<T> {
    /// Semi-major axis, *a* in meters (m)
    pub semimajor_axis: T,
    /// Eccentricity, *e*
    pub eccentricity: T,
    /// Inclination, *i*
    pub inclination: T,
    /// Argument of Periapsis, *ω*
    pub arg_of_periapsis: T,
    /// Time of Periapsis Passage, *T*
    pub time_of_periapsis_passage: T,
    /// Longitude of Ascending Node, *Ω*
    pub long_of_ascending_node: T,
}
impl<T> OrbitalElements<T> where T: Float + FromPrimitive + SubAssign {
	/// Sets the orbit's semimajor axis *a* in kilometers (km)
	pub fn with_semimajor_axis_km(mut self, a: T) -> Self {
		self.semimajor_axis = a * T::from_f64(CONVERT_KM_TO_M).unwrap();
		self
	}
	pub fn with_semimajor_axis_au(mut self, a: T) -> Self {
		self.semimajor_axis = a * T::from_f64(CONVERT_AU_TO_M).unwrap();
		self
	}
	pub fn with_semimajor_axis_m(mut self, a: T) -> Self {
		self.semimajor_axis = a;
		self
	}
	/// Sets the orbit's eccentricity
	pub fn with_eccentricity(mut self, e: T) -> Self {
		self.eccentricity = e;
		self
	}
	/// Sets the orbit's inclination *i* in degrees
	pub fn with_inclination_deg(mut self, deg: T) -> Self {
		self.inclination = deg * T::from_f64(CONVERT_DEG_TO_RAD).unwrap();
		let circle = T::from_f32(360.0).unwrap();
		while self.inclination > circle {
			self.inclination -= circle;
		}
		self
	}
	/// Sets the orbit's argument of periapsis *ω* in degrees
	pub fn with_arg_of_periapsis_deg(mut self, deg: T) -> Self {
		self.arg_of_periapsis = deg * T::from_f64(CONVERT_DEG_TO_RAD).unwrap();
		let circle = T::from_f32(360.0).unwrap();
		while self.arg_of_periapsis > circle {
			self.arg_of_periapsis -= circle;
		}
		self
	}
	/// Sets the orbit's longitude of ascending node *Ω* in degrees
	pub fn with_long_of_ascending_node_deg(mut self, deg: T) -> Self {
		self.long_of_ascending_node = deg * T::from_f64(CONVERT_DEG_TO_RAD).unwrap();
		let circle = T::from_f32(360.0).unwrap();
		while self.long_of_ascending_node > circle {
			self.long_of_ascending_node -= circle;
		}
		self
	}
}
impl<T> Default for OrbitalElements<T> where T: Copy + FromPrimitive {
	fn default() -> Self {
		let zero = T::from_f32(0.0).unwrap();
		Self {
			semimajor_axis: zero,
			eccentricity: zero,
			inclination: zero,
			arg_of_periapsis: zero,
			time_of_periapsis_passage: zero,
			long_of_ascending_node: zero,
		}
	}
}