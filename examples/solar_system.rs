use std::f32::consts::{PI, TAU};
use bevy::prelude::*;
use game_orbits::Database;


const ORBIT_SEGMENTS: usize = 100;
const ORBIT_COLOR: Color = Color::srgb(0.5, 1.0, 0.0);
const PERIAPSIS_COLOR: Color = Color::srgb(1.0, 0.5, 0.0);
const APOAPSIS_COLOR: Color = Color::srgb(0.0, 0.5, 1.0);
const PLANET_COLOR: Color = Color::srgb(1.0, 1.0, 1.0);
const SUN_COLOR: Color = Color::srgb(1.0, 1.0, 0.0);
const APSIS_SIZE: f32 = 0.5;
const PLANET_SIZE: f32 = 1.0;


fn setup_camera(mut commands: Commands) {
	commands.spawn((
		Camera3d::default(),
		Transform::from_xyz(0.0, 100.0, -1.0).looking_at(Vec3::ZERO, Vec3::Y),
	));
}

fn draw_orbits(mut gizmos: Gizmos, db: Res<Database<usize, f32>>) {
	let step = TAU / (ORBIT_SEGMENTS-1) as f32;
	for (handle, entry) in db.iter() {
		if entry.parent.is_none() {
			continue;
		}
		let orbit = entry.orbit.clone().unwrap();
		// draw orbit path
		for i in 0..ORBIT_SEGMENTS-1 {
			let t_0 = step * i as f32;
			let t_1 = step * (i + 1) as f32;
			let pos_0_nalgebra = db.position_at_mean_anomaly(*handle, t_0);
			let pos_1_nalgebra = db.position_at_mean_anomaly(*handle, t_1);
			let pos_0_bevy = Vec3::new(pos_0_nalgebra.x, pos_0_nalgebra.y, pos_0_nalgebra.z);
			let pos_1_bevy = Vec3::new(pos_1_nalgebra.x, pos_1_nalgebra.y, pos_1_nalgebra.z);
			gizmos.line(pos_0_bevy, pos_1_bevy, ORBIT_COLOR);
		}
		// draw apoapsis/periapsis
		let periapsis_nalgebra = db.position_at_mean_anomaly(*handle, 0.0);
		let apoapsis_nalgebra = db.position_at_mean_anomaly(*handle, PI);
		let periapsis_bevy = Vec3::new(periapsis_nalgebra.x, periapsis_nalgebra.y, periapsis_nalgebra.z);
		let apoapsis_bevy = Vec3::new(apoapsis_nalgebra.x, apoapsis_nalgebra.y, apoapsis_nalgebra.z);
		gizmos.sphere(periapsis_bevy, APSIS_SIZE, PERIAPSIS_COLOR);
		gizmos.sphere(apoapsis_bevy, APSIS_SIZE, APOAPSIS_COLOR);
	}
}

fn draw_planets(mut gizmos: Gizmos, db: Res<Database<usize, f32>>) {
	for (handle, entry) in db.iter() {
		if let Some(orbit) = entry.orbit {
			let pos_nalgebra = db.position_at_mean_anomaly(*handle, entry.mean_anomaly_at_epoch);
			let pos = Vec3::new(pos_nalgebra.x, pos_nalgebra.y, pos_nalgebra.z);
			gizmos.sphere(pos, PLANET_SIZE, PLANET_COLOR);
		}
	}
}

fn draw_axis(mut gizmos: Gizmos) {
	gizmos.axes(Transform::from_translation(Vec3::ZERO), 5.0);
}

fn main() {
	App::new()
		.add_plugins(DefaultPlugins)
		.insert_resource(Database::<usize, f32>::solar_system())
		.add_systems(Startup, setup_camera)
		.add_systems(Update, (draw_orbits, draw_planets, draw_axis))
		.run();
}