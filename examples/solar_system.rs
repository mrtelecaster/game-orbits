use std::f32::consts::{PI, TAU};
use bevy::prelude::*;
use game_orbits::{BevyPlanetDatabase, handles::*};


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
const CAM_CENTERED_ON_DEFAULT: usize = HANDLE_EARTH as usize;

const ORBIT_SEGMENTS: usize = 200;
const ORBIT_COLOR: Color = Color::srgb(0.5, 1.0, 0.0);
const PERIAPSIS_COLOR: Color = Color::srgb(1.0, 0.5, 0.0);
const APOAPSIS_COLOR: Color = Color::srgb(0.0, 0.5, 1.0);
const PLANET_COLOR: Color = Color::srgb(0.5, 0.0, 1.0);
const SOI_COLOR: Color = Color::srgb(1.0, 1.0, 1.0);
const APSIS_SIZE_MIN: f32 = 0.01;
const APSIS_SIZE_MAX: f32 = 50.0;

type Database = BevyPlanetDatabase<usize>;

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
		CameraParent::default().centered_on(CAM_CENTERED_ON_DEFAULT),
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
	database: Res<Database>,
){
	let (mut camera_parent_transform, camera_parent) = camera_parents.single_mut();
	let mut camera_transform = cameras.single_mut();
	let camera_rotation = Quat::from_axis_angle(Vec3::X, camera_parent.pitch);
	let camera_direction = camera_rotation * -Vec3::Z;
	let center_position = database.absolute_position_at_time(&camera_parent.centered_body, 0.0) * SCALE;
	// info!("Setting camera center position to {:?}", center_position);
	let camera_distance = CAM_MIN_DISTANCE.lerp(CAM_MAX_DISTANCE, camera_parent.zoom.powf(3.0));
	camera_parent_transform.translation = center_position;
	camera_parent_transform.rotation = Quat::from_axis_angle(Vec3::Y, camera_parent.yaw);
	camera_transform.translation = camera_direction * camera_distance;
	camera_transform.look_at(Vec3::ZERO, Vec3::Y);
}

fn draw_orbits(
	mut gizmos: Gizmos, db: Res<Database>, camera_parents: Query<&CameraParent>,
) {
	let camera_parent = camera_parents.single();
	let step = TAU / (ORBIT_SEGMENTS-1) as f32;
	for (handle, entry) in db.iter() {
		if let Some(parent_handle) = entry.parent {
			let parent_pos = db.absolute_position_at_time(&parent_handle, entry.mean_anomaly_at_epoch) * SCALE;
			// draw orbit path
			for i in 0..ORBIT_SEGMENTS-1 {
				let t_0 = step * i as f32;
				let t_1 = step * (i + 1) as f32;
				let pos_0 = db.position_at_mean_anomaly(handle, t_0) * SCALE;
				let pos_1 = db.position_at_mean_anomaly(handle, t_1) * SCALE;
				gizmos.line(pos_0 + parent_pos, pos_1 + parent_pos, ORBIT_COLOR);
			}
			// draw apoapsis/periapsis
			let pos_periapsis = db.position_at_mean_anomaly(handle, 0.0) * SCALE;
			let pos_apoapsis = db.position_at_mean_anomaly(handle, PI) * SCALE;
			let apsis_size = APSIS_SIZE_MIN.lerp(APSIS_SIZE_MAX, camera_parent.zoom.powf(3.0));
			gizmos.sphere(pos_periapsis + parent_pos, apsis_size, PERIAPSIS_COLOR);
			gizmos.sphere(pos_apoapsis + parent_pos, apsis_size, APOAPSIS_COLOR);
		}
	}
}

fn draw_planets(mut gizmos: Gizmos, db: Res<Database>) {
	for (handle, entry) in db.iter() {
		let pos = db.absolute_position_at_time(handle, 0.0) * SCALE;
		let soi_radius = db.radius_soi(handle);
		let info = entry.info.clone();
		// info!("Scale radius: {} units", info.radius_avg_km() * scale);
		gizmos.sphere(pos, soi_radius * SCALE, SOI_COLOR); // sphere of influence
		gizmos.sphere(pos, info.radius_avg_km() * SCALE, PLANET_COLOR);
	}
}

fn draw_axis(mut gizmos: Gizmos) {
	gizmos.axes(Transform::from_translation(Vec3::ZERO), 5.0);
}

fn main() {
	App::new()
		.add_plugins(DefaultPlugins)
		.insert_resource(Database::default().with_solar_system())
		.add_systems(Startup, setup_camera)
		.add_systems(Update, (
			draw_orbits, draw_planets, draw_axis,
			process_input, update_camera_position.after(process_input),
		))
		.run();
}