use std::f32::consts::{PI, TAU};
use bevy::prelude::*;
use game_orbits::Database;


const CAM_ROTATE_LEFT: KeyCode = KeyCode::ArrowLeft;
const CAM_ROTATE_RIGHT: KeyCode = KeyCode::ArrowRight;
const CAM_ROTATE_SPEED: f32 = 1.0; // rad/s

const ORBIT_SEGMENTS: usize = 100;
const ORBIT_COLOR: Color = Color::srgb(0.5, 1.0, 0.0);
const PERIAPSIS_COLOR: Color = Color::srgb(1.0, 0.5, 0.0);
const APOAPSIS_COLOR: Color = Color::srgb(0.0, 0.5, 1.0);
const PLANET_COLOR: Color = Color::srgb(1.0, 1.0, 1.0);
const SUN_COLOR: Color = Color::srgb(1.0, 1.0, 0.0);
const APSIS_SIZE: f32 = 0.2;

#[derive(Component)]
struct CameraParent {
	pub rotation: f32,
}
impl Default for CameraParent {
	fn default() -> Self {
		Self{ rotation: 0.0 }
	}
}


fn setup_camera(mut commands: Commands) {
	// camera
	let camera = commands.spawn((
		Camera3d::default(),
		Transform::from_xyz(50.0, 35.0, -100.0).looking_at(Vec3::ZERO, Vec3::Y),
	)).id();
	// camera parent
	commands.spawn((
		Transform::default(),
		CameraParent::default(),
	)).add_child(camera);
}

fn process_input(
	keyboard: Res<ButtonInput<KeyCode>>, time: Res<Time>,
	mut camera_parents: Query<&mut CameraParent>,
){
	let delta = time.delta_secs();
	let mut camera_parent = camera_parents.single_mut();
	if keyboard.pressed(CAM_ROTATE_RIGHT) {
		camera_parent.rotation += CAM_ROTATE_SPEED * delta;
	}
	if keyboard.pressed(CAM_ROTATE_LEFT) {
		camera_parent.rotation -= CAM_ROTATE_SPEED * delta;
	}
}

fn update_camera_position(
	mut camera_parents: Query<(&mut Transform, &CameraParent), Without<Camera3d>>,
	mut cameras: Query<&mut Transform, (With<Camera3d>, Without<CameraParent>)>,
){
	let (mut camera_parent_transform, camera_parent) = camera_parents.single_mut();
	let mut _camera = cameras.single_mut();
	camera_parent_transform.rotation = Quat::from_axis_angle(Vec3::Y, camera_parent.rotation);
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
	let sun = db.get_entry(0);
	let scale = sun.scale;
	for (handle, entry) in db.iter() {
		let pos_nalgebra = db.position_at_mean_anomaly(*handle, entry.mean_anomaly_at_epoch);
		let pos = Vec3::new(pos_nalgebra.x, pos_nalgebra.y, pos_nalgebra.z);
		let soi_radius = db.radius_soi(*handle);
		gizmos.sphere(pos, soi_radius * scale, PLANET_COLOR);
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
		.add_systems(Update, (
			draw_orbits, draw_planets, draw_axis,
			process_input, update_camera_position.after(process_input),
		))
		.run();
}