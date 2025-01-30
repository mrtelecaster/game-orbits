//! Data structures used by the library
use num_traits::{Float, FromPrimitive};
use crate::{constants::f64 as constants};


/// Keplerian elements that define an orbit
pub struct OrbitalElements<T> {
    /// Semi-major axis, *a*
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
impl<T> OrbitalElements<T> {
    pub fn new(
        semimajor_axis: T, eccentricity: T, inclination: T, arg_of_periapsis: T,
        time_of_periapsis_passage: T, long_of_ascending_node: T,
    ) -> Self {
        Self{
            semimajor_axis, eccentricity, inclination, arg_of_periapsis,
            time_of_periapsis_passage, long_of_ascending_node,
        }
    }
}


/// A body in space represented as an idealized sphere
pub struct Body<T> {
    /// Mass of this body in kilograms (kg)
    mass_kg: T,
    /// Equatorial radius of this body in kilometers (km)
    radius_equator_km: T,
    /// Polar radius of this body in kilometers (km)
    radius_polar_km: T,
}
impl<T> Body<T> where T: Float + FromPrimitive
{
    /// Create a new body with the given mass and radius properties
    pub fn new(mass_kg: T, radius_equator_km: T, radius_polar_km: T) -> Self {
        Self{ mass_kg: mass_kg, radius_equator_km, radius_polar_km }
    }
    /// Create a new body with the properties of [the planet Earth](https://en.wikipedia.org/wiki/Earth)
    pub fn new_earth() -> Self where T: FromPrimitive {
        Self::new(
			T::from_f64(constants::MASS_EARTH_KG).unwrap(),
			T::from_f64(constants::RADIUS_EARTH_EQUATOR_KM).unwrap(),
			T::from_f64(constants::RADIUS_EARTH_POLAR_KM).unwrap(),
		)
    }
	/// Create a new body with the properties of [our sun]()
	pub fn new_sol() -> Self where T: FromPrimitive {
		Self::new(
			T::from_f64(constants::MASS_SUN_KG).unwrap(),
			T::from_f64(constants::RADIUS_SUN_KM).unwrap(),
			T::from_f64(constants::RADIUS_SUN_KM).unwrap(),
		)
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
}


#[cfg(test)]
mod tests {
    use super::*;
    use approx::assert_ulps_eq;

    mod body {
        use super::*;

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
    }
}