//! Data structures used by the library
use std::ops::Mul;
use num_traits::FromPrimitive;
use crate::{constants::f64 as constants};


/// Keplerian elements that define an orbit
pub struct OrbitalElements<T> {
    /// Semi-major axis, *a*
    semimajor_axis: T,
    /// Eccentricity, *e*
    eccentricity: T,
    /// Inclination, *i*
    inclination: T,
    /// Argument of Periapsis, *ω*
    arg_of_periapsis: T,
    /// Time of Periapsis Passage, *T*
    time_of_periapsis_passage: T,
    /// Longitude of Ascending Node, *Ω*
    long_of_ascending_node: T,
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
impl<T> Body<T> where T: Copy
{
    /// Create a new body with the given mass and radius properties
    pub fn new(mass_kg: T, radius_equator_km: T, radius_polar_km: T) -> Self {
        Self{ mass_kg: mass_kg, radius_equator_km, radius_polar_km }
    }
    /// Create a new body with the properties of the planet [Earth](https://en.wikipedia.org/wiki/Earth)
    pub fn new_earth() -> Self where T: FromPrimitive {
        Self::new(
			T::from_f64(constants::MASS_EARTH_KG).unwrap(),
			T::from_f64(constants::RADIUS_EARTH_EQUATOR_KM).unwrap(),
			T::from_f64(constants::RADIUS_EARTH_POLAR_KM).unwrap(),
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
    pub fn radius_equator_m(&self) -> T where T: FromPrimitive + Mul<T, Output=T> {
        self.radius_equator_km * T::from_f64(constants::CONVERT_KM_TO_M).unwrap()
    }
    /// Calculates the body's *GM*, its mass times the Gravitational Constant *G*
    pub fn gm(&self) -> T where T: FromPrimitive + Mul<T, Output=T> {
        self.mass_kg * T::from_f64(constants::CONST_G).unwrap()
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
    }
}