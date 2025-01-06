//! Constants used for math and unit conversion

use std::f32::consts::TAU;
use crate::Float;


/// Gravitational Constant, *G* (N * m ^ 2 / kg ^ 2)
pub const CONSTANT_OF_GRAVITATION: Float = 6.6743015e-11;
/// Gravitational Constant, *G* (N * m ^ 2 / kg ^ 2)
pub const G: Float = CONSTANT_OF_GRAVITATION;

pub const CONVERT_DEG_TO_RAD: Float = TAU / 360.0;
pub const CONVERT_RAD_TO_DEG: Float = 360.0 / TAU;
pub const CONVERT_KM_TO_M: Float = 1000.0;

pub const RADIUS_EARTH_EQUATOR_KM: Float = 6378.137;
pub const RADIUS_EARTH_POLAR_KM: Float = 6356.752;
pub const MASS_EARTH_KG: Float = 5.972168e24;