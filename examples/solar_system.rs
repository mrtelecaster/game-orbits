use std::f32::consts::TAU;
use bevy::prelude::*;
use game_orbits::Database;


const ORBIT_SEGMENTS: usize = 100;
const ORBIT_COLOR: Color = Color::srgb(0.5, 1.0, 0.0);


fn setup_camera(mut commands: Commands) {
	commands.spawn((
		Camera3d::default(),
		Transform::from_xyz(1.0, 100.0, 0.0).looking_at(Vec3::ZERO, Vec3::Y),
	));
}

fn draw_orbits(mut gizmos: Gizmos, db: Res<Database<usize, f32>>) {
	let step = TAU / (ORBIT_SEGMENTS-1) as f32;
	for (handle, entry) in db.iter() {
		if entry.parent.is_none() {
			continue;
		}
		for i in 0..ORBIT_SEGMENTS-1 {
			let t_0 = step * i as f32;
			let t_1 = step * (i + 1) as f32;
			let (r_0, n_0) = db.position_at_mean_anomaly(*handle, t_0);
			let (r_1, n_1) = db.position_at_mean_anomaly(*handle, t_1);
			info!("r_0: {} \tn_0: {} \tr_1: {} \tn_1: {}", r_0, n_0, r_1, n_1);
			let rot_0 = Quat::from_axis_angle(Vec3::Y, n_0);
			let rot_1 = Quat::from_axis_angle(Vec3::Y, n_1);
			let dir_0 = rot_0 * Vec3::X;
			let dir_1 = rot_1 * Vec3::X;
			let pos_0 = dir_0 * r_0;
			let pos_1 = dir_1 * r_1;
			gizmos.line(pos_0, pos_1, ORBIT_COLOR);
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
		.add_systems(Update, (draw_orbits, draw_axis))
		.run();
}