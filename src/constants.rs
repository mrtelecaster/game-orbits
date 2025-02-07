//! Constants used for math and unit conversion
//! 
//! Constants are defined as [mod@f64] for maximum precision, then cast to lower precision for constants
//! in other modules.
//! 
//! ### Nomenclature
//! 
//! Constants are grouped together by their general purpose, denoted by the prefix used
//! 
//! #### Coefficients and general constants `CONST_*`
//! 
//! Constants beginning with `CONST_*` represent a single constant coefficient, expressed in SI
//! units. For example, the constant [`CONST_G`](f64::CONST_G) represents the gravitational constant
//! *G* which is used in a lot of orbital math.
//! 
//! #### Conversion `CONVERT_*`
//! 
//! Constants beginning with `CONVERT_X_TO_Y` are used for conversion between different units `X`
//! and `Y`. Multiply these constants with a number of unit `X` to get the equivalent value as unit `Y`. For
//! example, multiplying a number in kilometers (*km*) by the constant [`CONVERT_KM_TO_M`](f64::CONVERT_KM_TO_M)
//! will give you the value of the original number in meters (*m*)
//! 
//! #### Other
//! 
//! The remaining constants represent measurement values of various things that might be commonly
//! used by users of this library. For example, mass and size of planets in our solar system. The
//! name of the constant specifies both what the measurement represents using the prefix (e.g.
//! `MASS_*` for mass constants and `RADIUS_*` for a planetary radius), and the unit of the
//! measurement using the suffix (e.g. `*_KM` for kilometers)

pub mod f64 {
	use std::f64::consts::TAU;

	/// Gravitational Constant *G* (N * m ^ 2 / kg ^ 2)
	pub const CONST_GRAVITATION: f64 = 6.6743015e-11;
	/// Gravitational Constant *G* (N * m ^ 2 / kg ^ 2)
	pub const CONST_G: f64 = CONST_GRAVITATION;

	pub const CONVERT_AU_TO_KM: f64 = CONVERT_AU_TO_M * CONVERT_M_TO_KM;
	pub const CONVERT_AU_TO_M: f64 = 149597870700.0;
	pub const CONVERT_KM_TO_AU: f64 = 1.0 / CONVERT_AU_TO_KM;
	pub const CONVERT_DEG_TO_RAD: f64 = TAU / 360.0;
	pub const CONVERT_RAD_TO_DEG: f64 = 360.0 / TAU;
	pub const CONVERT_KM_TO_M: f64 = 1000.0;
	pub const CONVERT_M_TO_AU: f64 = 1.0 / CONVERT_AU_TO_M;
	pub const CONVERT_M_TO_KM: f64 = 0.001;
	pub const CONVERT_EARTH_MASS_TO_KG: f64 = 5.972168e24;
	pub const CONVERT_SUN_MASS_TO_KG: f64 = 1.9885e30;

	pub const RADIUS_EARTH_EQUATOR_KM: f64 = 6378.137;
	pub const RADIUS_EARTH_POLAR_KM: f64 = 6356.752;
	pub const RADIUS_EARTH_MEAN_KM: f64 = 6371.0;
	pub const RADIUS_SUN_M: f64 = 6.957e8;
	pub const MASS_EARTH_KG: f64 = 5.972168e24;
	pub const MASS_SUN_KG: f64 = 1.9885e30;
}

pub mod f32 {
	use super::f64 as constant;

	/// Gravitational Constant *G* (N * m ^ 2 / kg ^ 2)
	pub const CONSTANT_OF_GRAVITATION: f32 = constant::CONST_GRAVITATION as f32;
	/// Gravitational Constant *G* (N * m ^ 2 / kg ^ 2)
	pub const G: f32 = CONSTANT_OF_GRAVITATION;

	pub const CONVERT_DEG_TO_RAD: f32 = constant::CONVERT_DEG_TO_RAD as f32;
	pub const CONVERT_RAD_TO_DEG: f32 = constant::CONVERT_RAD_TO_DEG as f32;
	pub const CONVERT_KM_TO_M: f32 = constant::CONVERT_KM_TO_M as f32;
	pub const CONVERT_M_TO_KM: f32 = constant::CONVERT_M_TO_KM as f32;

	pub const RADIUS_EARTH_EQUATOR_KM: f32 = constant::RADIUS_EARTH_EQUATOR_KM as f32;
	pub const RADIUS_EARTH_POLAR_KM: f32 = constant::RADIUS_EARTH_POLAR_KM as f32;
}
