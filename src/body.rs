//! Data structures used by the library
use num_traits::{Float, FromPrimitive};
use crate::constants::f64 as constants;


/// A body in space represented as an idealized sphere
#[derive(Clone)]
pub struct Body<T> {
    /// Mass of this body in kilograms (kg)
    mass_kg: T,
    /// Equatorial radius of this body in kilometers (km)
    radius_equator_km: T,
    /// Polar radius of this body in kilometers (km)
    radius_polar_km: T,
	/// Axial tilt of the body relative to its orbital plane
	axial_tilt_deg: T,
}
impl<T> Body<T> where T: Float + FromPrimitive
{
    /// Create a new body with the given mass and radius properties
    pub fn new(mass_kg: T, radius_equator_km: T, radius_polar_km: T, axial_tilt_deg: T) -> Self {
        Self{ mass_kg: mass_kg, radius_equator_km, radius_polar_km, axial_tilt_deg }
    }
    /// Create a new body with the properties of [the planet Earth](https://en.wikipedia.org/wiki/Earth)
    pub fn new_earth() -> Self where T: FromPrimitive {
        Self::new(
			T::from_f64(constants::MASS_EARTH_KG).unwrap(),
			T::from_f64(constants::RADIUS_EARTH_EQUATOR_KM).unwrap(),
			T::from_f64(constants::RADIUS_EARTH_POLAR_KM).unwrap(),
			T::from_f64(23.4392811).unwrap(),
		)
    }
	/// Create a new body with the properties of [our sun]()
	pub fn new_sol() -> Self where T: FromPrimitive {
		let flattening_factor = 1.0 - 0.00005;
		Self::new(
			T::from_f64(constants::MASS_SUN_KG).unwrap(),
			T::from_f64(constants::RADIUS_SUN_M * constants::CONVERT_M_TO_KM).unwrap(),
			T::from_f64(constants::RADIUS_SUN_M * constants::CONVERT_M_TO_KM * flattening_factor).unwrap(),
			T::from_f32(0.0).unwrap(),
		)
	}
	pub fn with_mass_kg(mut self, mass: T) -> Self {
		self.mass_kg = mass;
		self
	}
	pub fn with_mass_earths(mut self, mass: T) -> Self {
		self.mass_kg = mass * T::from_f64(constants::CONVERT_EARTH_MASS_TO_KG).unwrap();
		self
	}
	/// Sets both the polar and equatorial radius to the given value
	pub fn with_radius_km(mut self, radius: T) -> Self {
		self.radius_polar_km = radius;
		self.radius_equator_km = radius;
		self
	}
	pub fn with_radius_m(mut self, radius: T) -> Self {
		let scale_factor = T::from_f64(constants::CONVERT_M_TO_KM).unwrap();
		self.radius_polar_km = radius * scale_factor;
		self.radius_equator_km = radius * scale_factor;
		self
	}
	pub fn with_radii_km(mut self, equatorial: T, polar: T) -> Self {
		self.radius_polar_km = polar;
		self.radius_equator_km = equatorial;
		self
	}
	pub fn with_axial_tilt_deg(mut self, axial_tilt: T) -> Self {
		self.axial_tilt_deg = axial_tilt;
		self
	}
    /// Gets the mass of this body in kilograms, *kg*
    pub fn mass_kg(&self) -> T {
        self.mass_kg
    }
    /// Gets the radius of this body in kilometers, *km*
    pub fn radius_equator_km(&self) -> T {
        self.radius_equator_km
    }
    /// Gets the polar radius of this body
    pub fn radius_polar_km(&self) -> T {
        self.radius_polar_km
    }
	pub fn radius_avg_km(&self) -> T {
		(self.radius_polar_km + self.radius_equator_km) / T::from_f32(2.0).unwrap()
	}
	pub fn radius_avg_m(&self) -> T {
		self.radius_avg_km() * T::from_f64(constants::CONVERT_KM_TO_M).unwrap()
	}
    /// Gets the radius of this body in meters, *m*
    pub fn radius_equator_m(&self) -> T {
        self.radius_equator_km * T::from_f64(constants::CONVERT_KM_TO_M).unwrap()
    }
    /// Calculates the body's *GM*, its mass times the Gravitational Constant *G*
    pub fn gm(&self) -> T {
        self.mass_kg * T::from_f64(constants::CONST_G).unwrap()
    }
	/// Returns the distance at which the force of gravity equals the given value
	/// 
	/// d = sqrt(GM/F)
	pub fn distance_of_gravity(&self, gravity: T) -> T {
		let g = T::from_f64(constants::CONST_G).unwrap();
		((g * self.mass_kg) / gravity).sqrt()
	}
	/// Calculate the force of gravity towards this body at the given distance
	/// 
	/// F = GM/d^2
	pub fn gravity_at_distance(&self, distance: T) -> T {
		let g = T::from_f64(constants::CONST_G).unwrap();
		(g * self.mass_kg) / distance.powi(2)
	}
	/// Returns this body's axial tilt in radians
	pub fn axial_tilt_rad(&self) -> T {
		self.axial_tilt_deg * T::from_f64(constants::CONVERT_DEG_TO_RAD).unwrap()
	}
}
impl<T> Default for Body<T> where T: Float + FromPrimitive {
	fn default() -> Self {
		let zero = T::from_f64(0.0).unwrap();
		Self::new(zero, zero, zero, zero)
	}
}


#[cfg(test)]
mod tests {
    use super::*;
    use approx::assert_ulps_eq;

	#[test]
	fn gm() {
		assert_ulps_eq!(3.986005e14, Body::new_earth().gm(), epsilon = 2000000.0);
	}

	#[test]
	fn gravity() {
		let earth: Body<f32> = Body::new_earth();
		let surface_altitude = constants::RADIUS_EARTH_MEAN_KM * constants::CONVERT_KM_TO_M;
		assert_ulps_eq!(9.81, earth.gravity_at_distance(surface_altitude as f32), epsilon=0.05);
		assert_ulps_eq!(surface_altitude as f32, earth.distance_of_gravity(9.81), epsilon=5000.0);
	}

	#[test]
	fn sun_sphere_of_influence() {
		let sun: Body<f32> = Body::new_sol();
		let gravity = 0.0000005; // force of gravity that results in a SOI distance larger than the heliopause
		let distance_m = sun.distance_of_gravity(gravity);
		let distance_au = distance_m * constants::CONVERT_M_TO_AU as f32;
		let minimum_au = 100.0; // distance of heliopause
		assert!(minimum_au < distance_au, "Expected distance of gravity to be greater than {:.2} AU, but {:.2} AU was returned", minimum_au, distance_au);
	}
}