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


#[cfg(test)]
mod tests {
    use super::*;
    use approx::assert_ulps_eq;

    /// Using this library to solve the example problems from the article
    /// 
    /// This confirms that the parts of the math based on the article give
    /// correct results, providing a baseline for the rest of the library
    mod problems {
        use super::*;

        /// [Problem 4.1](http://www.braeunig.us/space/problem.htm#4.1)
        #[test]
        fn problem_4_1() {
            let earth: Body = Body::new_earth();
            let gm: f64 = earth.gm();
            let altitude_m: f64 = 200_000.0;
            let r = earth.radius_m() + altitude_m;
            let v: f64 = (gm / r).sqrt();
            assert_ulps_eq!(7784.0, v, epsilon = 5.0)
        }
    }
}
