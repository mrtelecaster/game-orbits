use godot::{prelude::*, builtin::{Array, Vector3}, classes::{INode, Node}};
use crate::Database;

pub struct OrbitExtension;
#[gdextension]
unsafe impl ExtensionLibrary for OrbitExtension {}


fn vec_nalgebra_to_godot(input: nalgebra::Vector3<f32>) -> godot::builtin::Vector3 {
	godot::builtin::Vector3::new(input.x, input.y, input.z)
}


#[derive(GodotClass)]
#[class(base=Node)]
struct GodotPlanetDatabase {
	database: Database<i64, f32>,
}
#[godot_api]
impl INode for GodotPlanetDatabase {
	fn init(_base: Base<Node>) -> Self {
		Self{ database: Database::default() }
	}
}
#[godot_api]
impl GodotPlanetDatabase {
	#[func]
	pub fn add_solar_system(&mut self) {
		self.database.add_solar_system();
	}
	#[func]
	pub fn relative_position(&self, origin: i64, relative: i64, time: f32) -> Vector3 {
		vec_nalgebra_to_godot(self.database.relative_position(&origin, &relative, time).unwrap())
	}
	#[func]
	pub fn axial_tilt_rad(&self, handle: i64) -> f32 {
		self.database.get_entry(&handle).info.axial_tilt_rad()
	}
	#[func]
	pub fn radius_soi(&self, handle: i64) -> f32 {
		self.database.radius_soi(&handle)
	}
	#[func]
	pub fn get_satellites(&self, handle: i64) -> Array<i64> {
		let satellites = self.database.get_satellites(&handle);
		let mut output = Array::new();
		for handle in satellites {
			output.push(handle);
		}
		return output;
	}
}
