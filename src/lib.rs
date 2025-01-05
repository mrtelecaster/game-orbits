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
#[cfg(test)]
mod problems;


/// Defining this so that I can swap `f32` with `f64` and back to compare
pub type Float = f32;
