//! Constants used for math and unit conversion

pub mod f64 {
	use std::f64::consts::TAU;

	/// Gravitational Constant, *G* (N * m ^ 2 / kg ^ 2)
	pub const CONSTANT_OF_GRAVITATION: f64 = 6.6743015e-11;
	/// Gravitational Constant, *G* (N * m ^ 2 / kg ^ 2)
	pub const G: f64 = CONSTANT_OF_GRAVITATION;

	pub const CONVERT_DEG_TO_RAD: f64 = TAU / 360.0;
	pub const CONVERT_RAD_TO_DEG: f64 = 360.0 / TAU;
	pub const CONVERT_KM_TO_M: f64 = 1000.0;
	pub const CONVERT_M_TO_KM: f64 = 0.001;

	pub const RADIUS_EARTH_EQUATOR_KM: f64 = 6378.137;
	pub const RADIUS_EARTH_POLAR_KM: f64 = 6356.752;
	pub const MASS_EARTH_KG: f64 = 5.972168e24;
}

pub mod f32 {
	use super::f64 as constant;

	pub const CONSTANT_OF_GRAVITATION: f32 = constant::CONSTANT_OF_GRAVITATION as f32;
	pub const CONVERT_DEG_TO_RAD: f32 = constant::CONVERT_DEG_TO_RAD as f32;
	pub const CONVERT_RAD_TO_DEG: f32 = constant::CONVERT_RAD_TO_DEG as f32;
	pub const CONVERT_KM_TO_M: f32 = constant::CONVERT_KM_TO_M as f32;
	pub const CONVERT_M_TO_KM: f32 = constant::CONVERT_M_TO_KM as f32;
	pub const RADIUS_EARTH_EQUATOR_KM: f32 = constant::RADIUS_EARTH_EQUATOR_KM as f32;
	pub const RADIUS_EARTH_POLAR_KM: f32 = constant::RADIUS_EARTH_POLAR_KM as f32;
}