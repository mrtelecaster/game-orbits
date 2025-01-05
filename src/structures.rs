//! Data structures used by the library

use crate::{Float, constants};


/// Keplerian elements that define an orbit
pub struct OrbitalElements {
    /// Semi-major axis, *a*
    semimajor_axis: Float,
    /// Eccentricity, *e*
    eccentricity: Float,
    /// Inclination, *i*
    inclination: Float,
    /// Argument of Periapsis, *ω*
    arg_of_periapsis: Float,
    /// Time of Periapsis Passage, *T*
    time_of_periapsis_passage: Float,
    /// Longitude of Ascending Node, *Ω*
    long_of_ascending_node: Float,
}
impl OrbitalElements {
    pub fn new(
        semimajor_axis: Float, eccentricity: Float, inclination: Float, arg_of_periapsis: Float,
        time_of_periapsis_passage: Float, long_of_ascending_node: Float,
    ) -> Self {
        Self{
            semimajor_axis, eccentricity, inclination, arg_of_periapsis,
            time_of_periapsis_passage, long_of_ascending_node,
        }
    }
}


/// A body in space represented as an idealized sphere
pub struct Body {
    /// Mass of this body in kilograms (kg)
    mass_kg: Float,
    /// Equatorial radius of this body in kilometers (km)
    radius_equator_km: Float,
    /// Polar radius of this body in kilometers (km)
    radius_polar_km: Float,
}
impl Body
{
    /// Create a new body with the given mass and radius properties
    pub fn new(mass_kg: Float, radius_equator_km: Float, radius_polar_km: Float) -> Self {
        Self{ mass_kg: mass_kg.into(), radius_equator_km, radius_polar_km }
    }
    /// Create a new body with the properties of the planet [Earth](https://en.wikipedia.org/wiki/Earth)
    pub fn new_earth() -> Self {
        Self::new(constants::MASS_EARTH_KG, constants::RADIUS_EARTH_EQUATOR_KM, constants::RADIUS_EARTH_POLAR_KM)
    }
    /// Gets the mass of this body in kilograms, *kg*
    pub fn mass_kg(&self) -> Float {
        self.mass_kg
    }
    /// Gets the radius of this body in kilometers, *km*
    pub fn radius_equator_km(&self) -> Float {
        self.radius_equator_km
    }
    /// Gets the radius of this body in meters, *m*
    pub fn radius_equator_m(&self) -> Float {
        self.radius_equator_km * constants::CONVERT_KM_TO_M
    }
    /// Calculates the body's *GM*, its mass times the Gravitational Constant *G*
    pub fn gm(&self) -> Float {
        self.mass_kg * constants::G
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