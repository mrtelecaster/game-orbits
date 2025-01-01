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
//! All of the math and terminology is based on [*Orbital Mechanics*](http://www.braeunig.us/space/orbmech.htm),
//! a web article by Robert A. Braeunig


mod constants; pub use constants::*;
mod structures; pub use structures::*;


/// Defining this so that I can swap `f32` with `f64` and back to compare
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

        /// [Problem 4.7](http://www.braeunig.us/space/problem.htm#4.7)
        /// 
        /// A satellite in earth's orbit has a semi-major axis of 6,700 km and
        /// an eccentricity of 0.01. Calculate the satellite's altitude at both
        /// perigee and apogee.
        #[test]
        fn problem_4_7() {
            let a: Float = 6700.0;
            let e: Float = 0.01;
            let r_p: Float = a * (1.0 - e);
            let r_a: Float = a * (1.0 + e);
            assert_ulps_eq!(6633.0, r_p);
            assert_ulps_eq!(6767.0, r_a);
        }

        /// [Problem 4.8](http://www.braeunig.us/space/problem.htm#4.8)
        /// 
        /// Calculate a satellite's perigee and apogee altitude from a given
        /// position and velocity
        #[test]
        fn problem_4_8() {
            let earth = Body::new_earth();
            let r_1: Float = 6_628_140.0;
            let v_1: Float = 7_900.0;
            let angle_deg: Float = 89.0;
            let angle_rad: Float = angle_deg * CONVERT_DEG_TO_RAD;
            let c: Float = (2.0 * earth.gm()) / (r_1 * v_1.powi(2));
            let sqrt = c.powi(2) - (4.0 * (1.0 - c) * -(angle_rad.sin().powi(2)));
            let denominator = 2.0 * (1.0 - c);
            let r_a = r_1 * (-c - sqrt.sqrt()) / denominator;
            let r_p = r_1 * (-c + sqrt.sqrt()) / denominator;
            let epsilon = 10.0;
            assert_ulps_eq!(6_601_750.0, r_p, epsilon=epsilon);
            assert_ulps_eq!(7_175_100.0, r_a, epsilon=epsilon);
        }

        /// [Problem 4.9](http://www.braeunig.us/space/problem.htm#4.9)
        /// 
        /// Calculate the eccentricity of the satellite from 4.8
        #[test]
        fn problem_4_9() {
            let earth = Body::new_earth();
            let r_1: Float = 6_628_140.0;
            let v_1: Float = 7_900.0;
            let angle_deg: Float = 89.0;
            let angle_rad: Float = angle_deg * CONVERT_DEG_TO_RAD;
            let sin: Float = angle_rad.sin().powi(2);
            let cos: Float = angle_rad.cos().powi(2);
            let sqrt: Float = (r_1 * v_1.powi(2) / earth.gm() - 1.0).powi(2) * sin + cos;
            let e: Float = sqrt.sqrt();
            assert_ulps_eq!(0.0416170, e, epsilon=0.00000005);
        }

        /// [Problem 4.10](http://www.braeunig.us/space/problem.htm#4.10)
        /// 
        /// Calculate the angle *ν* from the perigee point to launch point for
        /// the stellite in problem 4.8
        #[test]
        fn problem_4_10() {
            let earth = Body::new_earth();
            let r_1: Float = 6_628_140.0;
            let v_1: Float = 7_900.0;
            let angle_deg: Float = 89.0;
            let angle_rad: Float = angle_deg * CONVERT_DEG_TO_RAD;
            let x = (r_1 * v_1.powi(2)) / earth.gm();
            let sin = angle_rad.sin();
            let cos = angle_rad.cos();
            let tan_nu = (x * sin * cos) / (x * sin.powi(2) - 1.0);
            let nu_rad = tan_nu.atan();
            let nu_deg = nu_rad * CONVERT_RAD_TO_DEG;
            assert_ulps_eq!(25.794, nu_deg);
        }

		/// [Problem 4.11](http://www.braeunig.us/space/problem.htm#4.11)
        /// 
        /// Calculate the semi-major axis of the orbit for the satellite in problem 4.8
        #[test]
        fn problem_4_11() {
            let earth = Body::new_earth();
            let r_1: Float = 6_628_140.0;
            let v_1: Float = 7_900.0;
            let a: Float = 1.0 / (2.0 / r_1 - v_1.powi(2) / earth.gm());
            assert_ulps_eq!(6_888_430.0, a);
        }

		/// [Problem 4.12](http://www.braeunig.us/space/problem.htm#4.12)
        /// 
        /// For the satellite in problem 4.8, burnout occurs 2000-10-20, 15:00 UT. The geocentric
		/// coordinates at burnout are 32° N latitude, 60° W longitude, and the azimuth heading
		/// is 86°.  Calculate the orbit's inclination, argument of perigee, and longitude of
		/// ascending node.
        #[test]
        fn problem_4_12() {
            let beta: Float = 86.0 * CONVERT_DEG_TO_RAD;
			let delta: Float = 32.0 * CONVERT_DEG_TO_RAD;
			let lambda_2: Float = -60.0 * CONVERT_DEG_TO_RAD;
			let nu: Float = 25.794 * CONVERT_DEG_TO_RAD;
			let epsilon_angle: Float = 0.0005;
			let i: Float = (delta.cos() * beta.sin()).acos();
            assert_ulps_eq!(32.223, i * CONVERT_RAD_TO_DEG, epsilon=epsilon_angle);
			let iota: Float = (delta.tan() / beta.cos()).atan();
			assert_ulps_eq!(83.630, iota * CONVERT_RAD_TO_DEG, epsilon=epsilon_angle);
			let omega: Float = iota - nu;
			assert_ulps_eq!(57.836, omega * CONVERT_RAD_TO_DEG, epsilon=epsilon_angle);
			let d_lambda: Float = (delta.sin() * beta.tan()).atan();
			assert_ulps_eq!(82.483, d_lambda * CONVERT_RAD_TO_DEG, epsilon=epsilon_angle);
			let lambda_1: Float = lambda_2 - d_lambda;
			assert_ulps_eq!(-142.483, lambda_1 * CONVERT_RAD_TO_DEG, epsilon=epsilon_angle);
        }
    }
}
