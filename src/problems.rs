//! Implementations of [the practice math problems](http://www.braeunig.us/space/problem.htm#4.14)
//! from [the *Orbital Mechanics* article](http://www.braeunig.us/space/orbmech.htm)
//! that the math in this library is based on.
//! 
//! The implementations here compare the results to the solutions from the practice problems.
//! However, the implementations use accurate values to real life while the solutions rely on
//! simplified values, meaning that there can occasionally be a large difference in the value
//! calculated by my implementation vs the solution to the problem from the website. If any of the
//! `epsilon` values used for comparison seem a little larger than they should be, this is why. The
//! calculations themselves aren't inaccurate, but they're calculated from different starting values
//! than those used for the problem solutions.
//! 
//! These tests aren't here to ensure that my math gets the same results as the problem solutions,
//! it's here just to verify that I've implemented the right function and haven't made any errors
//! interpreting the math equations into code.

use std::f32::consts::PI;
use approx::assert_ulps_eq;
use crate::{Body, constants::f32::*};


/// Illustrates the difference in precision between the numbers used in
/// the problem vs real world numbers
/// 
/// Gives some context to the use of the epsilon values used in the
/// other unit tests
#[test]
fn gm() {
    let problem_gm = 3.986005e14;
    let earth: Body<f32> = Body::new_earth();
    let gm: f32 = earth.mass_kg() * G;
    assert_ulps_eq!(problem_gm, gm, epsilon=0.001);
    let gm = earth.gm();
    assert_ulps_eq!(problem_gm, gm, epsilon=2.0e11);
    let earth = Body::new_earth();
    let gm = earth.gm();
    assert_ulps_eq!(problem_gm, gm, epsilon=2.0e11);
}

/// [Problem 4.1](http://www.braeunig.us/space/problem.htm#4.1)
#[test]
fn p_4_1() {
    let earth: Body<f32> = Body::new_earth();
    let gm: f32 = earth.gm();
    let altitude_m: f32 = 200_000.0;
    let r = earth.radius_equator_m() + altitude_m;
    let v: f32 = (gm / r).sqrt();
    assert_ulps_eq!(7784.0, v, epsilon = 2.0)
}

/// Problems from the [Motions of Planets and Satellites](http://www.braeunig.us/space/orbmech.htm#motions) section
mod motions
{
    use super::*;

    /// [Problem 4.2](http://www.braeunig.us/space/problem.htm#4.2)
    #[test]
    fn p_4_2() {
        let earth: Body<f32> = Body::new_earth();
        let gm: f32 = earth.gm();
        let altitude_m: f32 = 200_000.0;
        let r = earth.radius_equator_m() + altitude_m;
        let period_s: f32 = (4.0 * PI.powi(2) * r.powi(3) / gm).sqrt();
        assert_ulps_eq!(5310.0, period_s, epsilon = 2.0)
    }

    /// [Problem 4.3](http://www.braeunig.us/space/problem.htm#4.3)
    #[test]
    fn p_4_3() {
        let period_s: f32 = 86_164.1;
        let earth: Body<f32> = Body::new_earth();
        let gm: f32 = earth.gm();
        let r_m = (period_s.powi(2) * gm / (4.0 * PI.powi(2))).powf(1.0/3.0);
        assert_ulps_eq!(42_164_170.0, r_m, epsilon = 50.0)
    }

    /// [Problem 4.4](http://www.braeunig.us/space/problem.htm#4.4)
    #[test]
    fn p_4_4() {
        let earth: Body<f32> = Body::new_earth();
        let r_p: f32 = earth.radius_equator_m() + 250_000.0;
        let r_a: f32 = earth.radius_equator_m() + 500_000.0;
        let gm: f32 = earth.gm();
        let numerator_p: f32 = 2.0 * gm * r_a;
        let numerator_a: f32 = 2.0 * gm * r_p;
        let denominator_p: f32 = r_p * (r_a + r_p);
        let denominator_a: f32 = r_a * (r_a + r_p);
        let v_p: f32 = (numerator_p / denominator_p).sqrt();
        let v_a: f32 = (numerator_a / denominator_a).sqrt();
        let epsilon = 2.0;
        assert_ulps_eq!(7_826.0, v_p, epsilon=epsilon);
        assert_ulps_eq!(7_542.0, v_a, epsilon=epsilon);
    }

    /// [Problem 4.5](http://www.braeunig.us/space/problem.htm#4.5)
    #[test]
    fn p_4_5() {
        let earth: Body<f32> = Body::new_earth();
        let gm: f32 = earth.gm();
        let alt_p: f32 = 200_000.0; // Altitude at periapsis in meters
        let v_p: f32 = 7850.0; // Velocity at periapsis in meters per second
        let r_p: f32 = earth.radius_equator_m() + alt_p; // Radius of orbit at periapsis in meters
        let denominator: f32 = 2.0 * gm / (r_p * v_p.powi(2)) - 1.0;
        let r_a: f32 = r_p / denominator;
        let alt_a: f32 = r_a - earth.radius_equator_m();
        assert_ulps_eq!(6_805_140.0, r_a, epsilon = 5000.0);
        assert_ulps_eq!(427_000.0, alt_a, epsilon = 5000.0);
    }

    /// [Problem 4.6](http://www.braeunig.us/space/problem.htm#4.6)
    #[test]
    fn p_4_6() {
        let earth: Body<f32> = Body::new_earth();
        let r_p: f32 = 6_578_140.0;
        let v_p: f32 = 7_850.0;
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
    fn p_4_7() {
        let a: f32 = 6700.0;
        let e: f32 = 0.01;
        let r_p: f32 = a * (1.0 - e);
        let r_a: f32 = a * (1.0 + e);
        assert_ulps_eq!(6633.0, r_p);
        assert_ulps_eq!(6767.0, r_a);
    }
}


/// Problems from the [Launch of a Space Vehicle](http://www.braeunig.us/space/orbmech.htm#launch) section
mod launch
{
    use super::*;

    /// [Problem 4.8](http://www.braeunig.us/space/problem.htm#4.8)
    /// 
    /// Calculate a satellite's perigee and apogee altitude from a given
    /// position and velocity
    #[test]
    fn p_4_8() {
        let earth: Body<f32> = Body::new_earth();
        let r_1: f32 = 6_628_140.0;
        let v_1: f32 = 7_900.0;
        let angle_deg: f32 = 89.0;
        let angle_rad: f32 = angle_deg * CONVERT_DEG_TO_RAD;
        let c: f32 = (2.0 * earth.gm()) / (r_1 * v_1.powi(2));
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
    fn p_4_9() {
        let earth: Body<f32> = Body::new_earth();
        let r_1: f32 = 6_628_140.0;
        let v_1: f32 = 7_900.0;
        let angle_deg: f32 = 89.0;
        let angle_rad: f32 = angle_deg * CONVERT_DEG_TO_RAD;
        let sin: f32 = angle_rad.sin().powi(2);
        let cos: f32 = angle_rad.cos().powi(2);
        let sqrt: f32 = (r_1 * v_1.powi(2) / earth.gm() - 1.0).powi(2) * sin + cos;
        let e: f32 = sqrt.sqrt();
        assert_ulps_eq!(0.0416170, e, epsilon=0.00000005);
    }

    /// [Problem 4.10](http://www.braeunig.us/space/problem.htm#4.10)
    /// 
    /// Calculate the angle *ν* from the perigee point to launch point for
    /// the stellite in problem 4.8
    #[test]
    fn p_4_10() {
        let earth: Body<f32> = Body::new_earth();
        let r_1: f32 = 6_628_140.0;
        let v_1: f32 = 7_900.0;
        let angle_deg: f32 = 89.0;
        let angle_rad: f32 = angle_deg * CONVERT_DEG_TO_RAD;
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
    fn p_4_11() {
        let earth: Body<f32> = Body::new_earth();
        let r_1: f32 = 6_628_140.0;
        let v_1: f32 = 7_900.0;
        let a: f32 = 1.0 / (2.0 / r_1 - v_1.powi(2) / earth.gm());
        assert_ulps_eq!(6_888_430.0, a);
    }

    /// [Problem 4.12](http://www.braeunig.us/space/problem.htm#4.12)
    /// 
    /// For the satellite in problem 4.8, burnout occurs 2000-10-20, 15:00 UT. The geocentric
    /// coordinates at burnout are 32° N latitude, 60° W longitude, and the azimuth heading
    /// is 86°.  Calculate the orbit's inclination, argument of perigee, and longitude of
    /// ascending node.
    #[test]
    fn p_4_12() {
        let beta: f32 = 86.0 * CONVERT_DEG_TO_RAD;
        let delta: f32 = 32.0 * CONVERT_DEG_TO_RAD;
        let lambda_2: f32 = -60.0 * CONVERT_DEG_TO_RAD;
        let nu: f32 = 25.794 * CONVERT_DEG_TO_RAD;
        let epsilon_angle: f32 = 0.0005;
        let i: f32 = (delta.cos() * beta.sin()).acos();
        assert_ulps_eq!(32.223, i * CONVERT_RAD_TO_DEG, epsilon=epsilon_angle);
        let iota: f32 = (delta.tan() / beta.cos()).atan();
        assert_ulps_eq!(83.630, iota * CONVERT_RAD_TO_DEG, epsilon=epsilon_angle);
        let omega: f32 = iota - nu;
        assert_ulps_eq!(57.836, omega * CONVERT_RAD_TO_DEG, epsilon=epsilon_angle);
        let d_lambda: f32 = (delta.sin() * beta.tan()).atan();
        assert_ulps_eq!(82.483, d_lambda * CONVERT_RAD_TO_DEG, epsilon=epsilon_angle);
        let lambda_1: f32 = lambda_2 - d_lambda;
        assert_ulps_eq!(-142.483, lambda_1 * CONVERT_RAD_TO_DEG, epsilon=epsilon_angle);
    }

    /// [Problem 4.13](http://www.braeunig.us/space/problem.htm#4.13)
    /// 
    /// A satellite is in an orbit with a semi-major axis of 7,500 km and an eccentricity of
    /// 0.1. Calculate the time it takes to move from a position 30 degrees past perigee to 90
    /// degrees past perigee.
    #[test]
    fn p_4_13() {
        let earth: Body<f32> = Body::new_earth();
        let a: f32 = 7_500_000.0;
        let e: f32 = 0.1;
        let t_0: f32 = 0.0;
        let nu_0: f32 = 30.0 * CONVERT_DEG_TO_RAD;
        let nu: f32 = 90.0 * CONVERT_DEG_TO_RAD;
        let epsilon = 0.01;
        let eccentric_anomaly_0: f32 = ((e + nu_0.cos()) / (1.0 + e * nu_0.cos())).acos();
        assert_ulps_eq!(0.47557, eccentric_anomaly_0, epsilon=epsilon);
        let eccentric_anomaly: f32 = ((e + nu.cos()) / (1.0 + e * nu_0.cos())).acos();
        assert_ulps_eq!(1.47063, eccentric_anomaly, epsilon=epsilon);
        let mean_anomaly_0: f32 = eccentric_anomaly_0 - e * eccentric_anomaly_0.sin();
        assert_ulps_eq!(0.42978, mean_anomaly_0, epsilon=epsilon);
        let mean_anomaly: f32 = eccentric_anomaly - e * eccentric_anomaly.sin();
        assert_ulps_eq!(1.37113, mean_anomaly, epsilon=epsilon);
        let n = (earth.gm() / a.powi(3)).sqrt();
        assert_ulps_eq!(0.00097202, n, epsilon=0.000001);
        let t = t_0 + (mean_anomaly - mean_anomaly_0) / n;
        assert_ulps_eq!(968.4, t, epsilon=10.0);
    }

    /// [Problem 4.14](http://www.braeunig.us/space/problem.htm#4.14)
    /// 
    /// The satellite in problem 4.13 has a true anomaly of 90 degrees.  What will be the
    /// satellite's position, i.e. it's true anomaly, 20 minutes later?
    #[test]
    fn p_4_14() {
        // let earth: Body = Body::new(EARTH_MASS_KG, EARTH_RADIUS_KM);
        // let a: f32 = 7_500_000.0;
        let e: f32 = 0.1;
        let t_0: f32 = 0.0;
        let t: f32 = 1200.0;
        // let nu_0: f32 = 30.0 * CONVERT_DEG_TO_RAD;
        let mean_anomaly_0: f32 = 1.37113;
        let n: f32 = 0.00097202;
        let mean_anomaly: f32 = mean_anomaly_0 + n * (t - t_0);
        assert_ulps_eq!(2.53755, mean_anomaly, epsilon=0.000005);
        // Low accuracy
        let nu: f32 = mean_anomaly + 2.0 * e * mean_anomaly.sin() + 1.25 * e.powi(2) * (2.0 * mean_anomaly).sin();
        assert_ulps_eq!(2.63946, nu, epsilon=0.000002);
    }

    /// [Problem 4.15](http://www.braeunig.us/space/problem.htm#4.15)
    /// 
    /// The satellite in problem 4.13 has a true anomaly of 90 degrees.  What will be the
    /// satellite's position, i.e. it's true anomaly, 20 minutes later?
    #[test]
    fn p_4_15() {
        let earth: Body<f32> = Body::new_earth();
        let a: f32 = 7_500_000.0;
        let e: f32 = 0.1;
        let nu: f32 = 225.0 * CONVERT_DEG_TO_RAD;
        let r: f32 = a * (1.0 - e.powi(2)) / (1.0 + e * nu.cos());
        assert_ulps_eq!(7_989_977.0, r);
        let theta: f32 = (e * nu.sin() / (1.0 + e * nu.cos())).atan();
        assert_ulps_eq!(-4.351, theta * CONVERT_RAD_TO_DEG, epsilon=0.0005);
        let v: f32 = (earth.gm() * (2.0 / r - 1.0 / a)).sqrt();
        assert_ulps_eq!(6828.0, v, epsilon=0.5);
    }
}

/// Problems from the [Orbit Maneuvers](http://www.braeunig.us/space/orbmech.htm#maneuver) section
mod maneuver
{
    use super::*;

    /// [Problem 4.19](http://www.braeunig.us/space/problem.htm#4.19)
    /// 
    /// A spacecraft is in a circular parking orbit with an altitude of 200 km. Calculate the
    /// velocity change required to perform a Hohmann transfer to a circular orbit at
    /// geosynchronous altitude.
    #[test]
    fn p_4_19() {
        let earth: Body<f32> = Body::new_earth();
        let r_a: f32 = 6_578_140.0;
        let r_b: f32 = 42_164_170.0;
        let a_tx: f32 = (r_a + r_b) / 2.0;
        assert_ulps_eq!(24_371_155.0, a_tx);
        let v_epsilon = 2.0;
        let v_i_a: f32 = (earth.gm() / r_a).sqrt();
        assert_ulps_eq!(7_784.0, v_i_a, epsilon=v_epsilon);
        let v_f_b: f32 = (earth.gm() / r_b).sqrt();
        assert_ulps_eq!(3_075.0, v_f_b, epsilon=v_epsilon);
        let v_tx_a: f32 = (earth.gm() * (2.0 / r_a - 1.0 / a_tx)).sqrt();
        assert_ulps_eq!(10_239.0, v_tx_a, epsilon=v_epsilon);
        let v_tx_b: f32 = (earth.gm() * (2.0 / r_b - 1.0 / a_tx)).sqrt();
        assert_ulps_eq!(1_597.0, v_tx_b, epsilon=v_epsilon);
        let delta_v_a: f32 = v_tx_a - v_i_a;
        assert_ulps_eq!(2_455.0, delta_v_a, epsilon=v_epsilon);
        let delta_v_b: f32 = v_f_b - v_tx_b;
        assert_ulps_eq!(1_478.0, delta_v_b, epsilon=v_epsilon);
        let delta_v_t: f32 = delta_v_a + delta_v_b;
        assert_ulps_eq!(3_933.0, delta_v_t, epsilon=v_epsilon);
    }

    /// [Problem 4.20](http://www.braeunig.us/space/problem.htm#4.20)
    /// 
    /// A satellite is in a circular parking orbit with an altitude of 200 km.
    /// Using a one-tangent burn, it is to be transferred to geosynchronous
    /// altitude using a transfer ellipse with a semi-major axis of 30,000 km.
    /// Calculate the total required velocity change and the time required to
    /// complete the transfer. 
    #[test]
    fn p_4_20() {
        let earth: Body<f32> = Body::new_earth();
        let r_a: f32 = 6_578_140.0;
        let r_b: f32 = 42_164_170.0;
        let a_tx: f32 = (r_a + r_b) / 2.0;
        assert_ulps_eq!(24_371_155.0, a_tx);
        let v_epsilon = 2.0;
        let v_i_a: f32 = (earth.gm() / r_a).sqrt();
        assert_ulps_eq!(7_784.0, v_i_a, epsilon=v_epsilon);
        let v_f_b: f32 = (earth.gm() / r_b).sqrt();
        assert_ulps_eq!(3_075.0, v_f_b, epsilon=v_epsilon);
        let v_tx_a: f32 = (earth.gm() * (2.0 / r_a - 1.0 / a_tx)).sqrt();
        assert_ulps_eq!(10_239.0, v_tx_a, epsilon=v_epsilon);
        let v_tx_b: f32 = (earth.gm() * (2.0 / r_b - 1.0 / a_tx)).sqrt();
        assert_ulps_eq!(1_597.0, v_tx_b, epsilon=v_epsilon);
        let delta_v_a: f32 = v_tx_a - v_i_a;
        assert_ulps_eq!(2_455.0, delta_v_a, epsilon=v_epsilon);
        let delta_v_b: f32 = v_f_b - v_tx_b;
        assert_ulps_eq!(1_478.0, delta_v_b, epsilon=v_epsilon);
        let delta_v_t: f32 = delta_v_a + delta_v_b;
        assert_ulps_eq!(3_933.0, delta_v_t, epsilon=v_epsilon);
    }
}