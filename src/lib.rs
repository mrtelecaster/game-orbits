//! Orbital mechanics library for games
//! 
//! Used for simulating celestial bodies' orbits using basic Keplerian mechanics
//! and return information on the relative positions and directions between
//! those bodies from various points of reference for rendering in game engines.
//! 
//! This library is inteded for use either with [the Bevy engine](https://bevyengine.org)
//! or with an as-yet-unbuilt wrapper library for [the Godot engine](https://godotengine.org/)
//! for a personal project of mine.
//! 
//! All of the math and terminology is based on [*Orbital Mechanics*](), a web
//! article by Robert A. Braeunig


mod constants; pub use constants::*;
mod structures; pub use structures::*;


pub type Float = f32;


#[cfg(test)]
mod tests {
    use super::*;
    use approx::assert_ulps_eq;

    /// Using this library to solve the [example problems from the article](http://www.braeunig.us/space/problem.htm)
    /// 
    /// This confirms that the parts of the math based on the article give
    /// correct results, providing a baseline for the rest of the library
    mod problems {
        use std::f32::consts::PI;

        const EARTH_MASS_KG: Float = 5.9737e24;
        const EARTH_RADIUS_KM: Float = 6_378.14;
        const G: Float = 6.67259e-11;

        use super::*;

        /// Illustrates the difference in precision between the numbers used in
        /// the problem vs real world numbers
        /// 
        /// Gives some context to the use of the epsilon values used in the
        /// other unit tests
        #[test]
        fn gm() {
            let problem_gm = 3.986005e14;
            let earth = Body::new(EARTH_MASS_KG, EARTH_RADIUS_KM);
            let gm = earth.mass_kg() * G;
            assert_ulps_eq!(problem_gm, gm, epsilon=0.001);
            let gm = earth.gm();
            assert_ulps_eq!(problem_gm, gm, epsilon=2.0e11);
            let earth = Body::new_earth();
            let gm = earth.gm();
            assert_ulps_eq!(problem_gm, gm, epsilon=2.0e11);
        }

        /// [Problem 4.1](http://www.braeunig.us/space/problem.htm#4.1)
        #[test]
        fn problem_4_1() {
            let earth: Body = Body::new(EARTH_MASS_KG, EARTH_RADIUS_KM);
            let gm: Float = earth.gm();
            let altitude_m: Float = 200_000.0;
            let r = earth.radius_m() + altitude_m;
            let v: Float = (gm / r).sqrt();
            assert_ulps_eq!(7784.0, v, epsilon = 2.0)
        }

        /// [Problem 4.2](http://www.braeunig.us/space/problem.htm#4.2)
        #[test]
        fn problem_4_2() {
            let earth: Body = Body::new(EARTH_MASS_KG, EARTH_RADIUS_KM);
            let gm: Float = earth.gm();
            let altitude_m: Float = 200_000.0;
            let r = earth.radius_m() + altitude_m;
            let period_s: Float = (4.0 * PI.powi(2) * r.powi(3) / gm).sqrt();
            assert_ulps_eq!(5310.0, period_s, epsilon = 2.0)
        }

        /// [Problem 4.3](http://www.braeunig.us/space/problem.htm#4.3)
        #[test]
        fn problem_4_3() {
            let period_s: Float = 86_164.1;
            let earth: Body = Body::new_earth();
            let gm: Float = earth.gm();
            let r_m = (period_s.powi(2) * gm / (4.0 * PI.powi(2))).powf(1.0/3.0);
            assert_ulps_eq!(42_164_170.0, r_m, epsilon = 50.0)
        }

        /// [Problem 4.4](http://www.braeunig.us/space/problem.htm#4.4)
        #[test]
        fn problem_4_4() {
            let earth: Body = Body::new(EARTH_MASS_KG, EARTH_RADIUS_KM);
            let r_p: Float = earth.radius_m() + 250_000.0;
            let r_a: Float = earth.radius_m() + 500_000.0;
            let gm: Float = earth.gm();
            let numerator_p: Float = 2.0 * gm * r_a;
            let numerator_a: Float = 2.0 * gm * r_p;
            let denominator_p: Float = r_p * (r_a + r_p);
            let denominator_a: Float = r_a * (r_a + r_p);
            let v_p: Float = (numerator_p / denominator_p).sqrt();
            let v_a: Float = (numerator_a / denominator_a).sqrt();
            let epsilon = 2.0;
            assert_ulps_eq!(7_826.0, v_p, epsilon=epsilon);
            assert_ulps_eq!(7_542.0, v_a, epsilon=epsilon);
        }

        /// [Problem 4.5](http://www.braeunig.us/space/problem.htm#4.5)
        #[test]
        fn problem_4_5() {
            let earth: Body = Body::new(EARTH_MASS_KG, EARTH_RADIUS_KM);
            let gm: Float = earth.gm();
            let alt_p: Float = 200_000.0; // Altitude at periapsis in meters
            let v_p: Float = 7850.0; // Velocity at periapsis in meters per second
            let r_p: Float = earth.radius_m() + alt_p; // Radius of orbit at periapsis in meters
            let denominator: Float = 2.0 * gm / (r_p * v_p.powi(2)) - 1.0;
            let r_a: Float = r_p / denominator;
            let alt_a: Float = r_a - earth.radius_m();
            assert_ulps_eq!(6_805_140.0, r_a, epsilon = 5000.0);
            assert_ulps_eq!(427_000.0, alt_a, epsilon = 5000.0);
        }

        /// [Problem 4.6](http://www.braeunig.us/space/problem.htm#4.6)
        #[test]
        fn problem_4_6() {
            let earth = Body::new_earth();
            let r_p: Float = 6_578_140.0;
            let v_p: Float = 7_850.0;
            let numerator = r_p * v_p.powi(2);
            let denominator = earth.gm();
            let e = numerator / denominator - 1.0;
            assert_ulps_eq!(0.01696, e, epsilon=0.000002);
        }
    }
}
