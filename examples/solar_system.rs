use std::f32::consts::{PI, TAU};
use bevy::prelude::*;
use game_orbits::Database;


const SCALE: f32 = 1.0 / 100_000_000.0;

const CAM_ROTATE_UP: KeyCode = KeyCode::ArrowUp;
const CAM_ROTATE_DOWN: KeyCode = KeyCode::ArrowDown;
const CAM_ROTATE_LEFT: KeyCode = KeyCode::ArrowLeft;
const CAM_ROTATE_RIGHT: KeyCode = KeyCode::ArrowRight;
const CAM_ZOOM_IN: KeyCode = KeyCode::Equal;
const CAM_ZOOM_OUT: KeyCode = KeyCode::Minus;
const CAM_ROTATE_SPEED: f32 = 1.0; // rad/s
const CAM_MIN_DISTANCE: f32 = 0.4;
const CAM_MAX_DISTANCE: f32 = 10000.0;
const CAM_ZOOM_SPEED: f32 = 0.1;

const ORBIT_SEGMENTS: usize = 200;
const ORBIT_COLOR: Color = Color::srgb(0.5, 1.0, 0.0);
const PERIAPSIS_COLOR: Color = Color::srgb(1.0, 0.5, 0.0);
const APOAPSIS_COLOR: Color = Color::srgb(0.0, 0.5, 1.0);
const PLANET_COLOR: Color = Color::srgb(0.5, 0.0, 1.0);
const SOI_COLOR: Color = Color::srgb(1.0, 1.0, 1.0);
const APSIS_SIZE: f32 = 0.5;

#[derive(Component)]
struct CameraParent {
	pub centered_body: usize,
	pub yaw: f32,
	pub pitch: f32,
	pub zoom: f32,
}
impl CameraParent {
	pub fn centered_on(mut self, handle: usize) -> Self {
		self.centered_body = handle;
		self
	}
}
impl Default for CameraParent {
	fn default() -> Self {
		Self{ yaw: 0.0, pitch: 0.0, zoom: 0.1, centered_body: 0 }
	}
}


fn setup_camera(mut commands: Commands) {
	// camera
	let camera = commands.spawn((
		Camera3d::default(),
		Transform::from_xyz(0.0, 35.0, -100.0).looking_at(Vec3::ZERO, Vec3::Y),
		InheritedVisibility::default(),
	)).id();
	// camera parent
	commands.spawn((
		Transform::default(),
		Visibility::default(),
		CameraParent::default().centered_on(0),
	)).add_child(camera);
}

fn process_input(
	keyboard: Res<ButtonInput<KeyCode>>, time: Res<Time>,
	mut camera_parents: Query<&mut CameraParent>,
){
	let delta = time.delta_secs();
	let mut camera_parent = camera_parents.single_mut();
	if keyboard.pressed(CAM_ROTATE_RIGHT) {
		camera_parent.yaw += CAM_ROTATE_SPEED * delta;
	}
	if keyboard.pressed(CAM_ROTATE_LEFT) {
		camera_parent.yaw -= CAM_ROTATE_SPEED * delta;
	}
	if keyboard.pressed(CAM_ROTATE_UP) {
		camera_parent.pitch += CAM_ROTATE_SPEED * delta;
	}
	if keyboard.pressed(CAM_ROTATE_DOWN) {
		camera_parent.pitch -= CAM_ROTATE_SPEED * delta;
	}
	camera_parent.pitch = camera_parent.pitch.clamp(-PI/2.0, PI/2.0);
	if keyboard.pressed(CAM_ZOOM_IN) {
		camera_parent.zoom -= CAM_ZOOM_SPEED * delta;
	}
	if keyboard.pressed(CAM_ZOOM_OUT) {
		camera_parent.zoom += CAM_ZOOM_SPEED * delta;
	}
	camera_parent.zoom = camera_parent.zoom.clamp(0.0, 1.0);
}

fn update_camera_position(
	mut camera_parents: Query<(&mut Transform, &CameraParent), Without<Camera3d>>,
	mut cameras: Query<&mut Transform, (With<Camera3d>, Without<CameraParent>)>,
	database: Res<Database<usize, f32>>,
){
	let (mut camera_parent_transform, camera_parent) = camera_parents.single_mut();
	let mut camera_transform = cameras.single_mut();
	let camera_rotation = Quat::from_axis_angle(Vec3::X, camera_parent.pitch);
	let camera_direction = camera_rotation * -Vec3::Z;
	let centered_on_entry = database.get_entry(camera_parent.centered_body);
	let nalgebra_position = database.position_at_mean_anomaly(camera_parent.centered_body, centered_on_entry.mean_anomaly_at_epoch);
	let center_position = Vec3::new(nalgebra_position.x, nalgebra_position.y, nalgebra_position.z) * SCALE;
	// info!("Setting camera center position to {:?}", center_position);
	let camera_distance = CAM_MIN_DISTANCE.lerp(CAM_MAX_DISTANCE, camera_parent.zoom.powf(3.0));
	camera_parent_transform.translation = center_position;
	camera_parent_transform.rotation = Quat::from_axis_angle(Vec3::Y, camera_parent.yaw);
	camera_transform.translation = camera_direction * camera_distance;
	camera_transform.look_at(Vec3::ZERO, Vec3::Y);
}

fn draw_orbits(mut gizmos: Gizmos, db: Res<Database<usize, f32>>) {
	let step = TAU / (ORBIT_SEGMENTS-1) as f32;
	for (handle, entry) in db.iter() {
		if let Some(parent_handle) = entry.parent {
			let parent_position_nalgebra = db.position_at_mean_anomaly(parent_handle, entry.mean_anomaly_at_epoch);
			let parent_pos = Vec3::new(parent_position_nalgebra.x, parent_position_nalgebra.y, parent_position_nalgebra.z);
			// draw orbit path
			for i in 0..ORBIT_SEGMENTS-1 {
				let t_0 = step * i as f32;
				let t_1 = step * (i + 1) as f32;
				let pos_0_nalgebra = db.position_at_mean_anomaly(*handle, t_0);
				let pos_1_nalgebra = db.position_at_mean_anomaly(*handle, t_1);
				let pos_0_bevy = Vec3::new(pos_0_nalgebra.x, pos_0_nalgebra.y, pos_0_nalgebra.z) * SCALE;
				let pos_1_bevy = Vec3::new(pos_1_nalgebra.x, pos_1_nalgebra.y, pos_1_nalgebra.z) * SCALE;
				gizmos.line(pos_0_bevy + parent_pos, pos_1_bevy + parent_pos, ORBIT_COLOR);
			}
			// draw apoapsis/periapsis
			let periapsis_nalgebra = db.position_at_mean_anomaly(*handle, 0.0) * SCALE;
			let apoapsis_nalgebra = db.position_at_mean_anomaly(*handle, PI) * SCALE;
			let periapsis_bevy = Vec3::new(periapsis_nalgebra.x, periapsis_nalgebra.y, periapsis_nalgebra.z);
			let apoapsis_bevy = Vec3::new(apoapsis_nalgebra.x, apoapsis_nalgebra.y, apoapsis_nalgebra.z);
			gizmos.sphere(periapsis_bevy + parent_pos, APSIS_SIZE, PERIAPSIS_COLOR);
			gizmos.sphere(apoapsis_bevy + parent_pos, APSIS_SIZE, APOAPSIS_COLOR);
		}
	}
}

fn draw_planets(mut gizmos: Gizmos, db: Res<Database<usize, f32>>) {
	let sun = db.get_entry(0);
	let scale = sun.scale;
	for (handle, entry) in db.iter() {
		let pos_nalgebra = db.position_at_mean_anomaly(*handle, entry.mean_anomaly_at_epoch);
		let pos = Vec3::new(pos_nalgebra.x, pos_nalgebra.y, pos_nalgebra.z) * SCALE;
		let soi_radius = db.radius_soi(*handle);
		let info = entry.info.clone();
		// info!("Scale radius: {} units", info.radius_avg_km() * scale);
		gizmos.sphere(pos, soi_radius * scale, SOI_COLOR); // sphere of influence
		gizmos.sphere(pos, info.radius_avg_km() * scale, PLANET_COLOR);
	}
}

fn draw_axis(mut gizmos: Gizmos) {
	gizmos.axes(Transform::from_translation(Vec3::ZERO), 5.0);
}

fn main() {
	App::new()
		.add_plugins(DefaultPlugins)
		.insert_resource(Database::<usize, f32>::default().with_solar_system())
		.add_systems(Startup, setup_camera)
		.add_systems(Update, (
			draw_orbits, draw_planets, draw_axis,
			process_input, update_camera_position.after(process_input),
		))
		.run();
}