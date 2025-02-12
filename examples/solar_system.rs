use std::f32::consts::{PI, TAU};
use bevy::prelude::*;
use game_orbits::{BevyPlanetDatabase, handles::*};


const SCALE: f32 = 1.0 / 20_000_000.0;

const CAM_ROTATE_UP: KeyCode = KeyCode::KeyW;
const CAM_ROTATE_DOWN: KeyCode = KeyCode::KeyS;
const CAM_ROTATE_LEFT: KeyCode = KeyCode::KeyA;
const CAM_ROTATE_RIGHT: KeyCode = KeyCode::KeyD;
const CAM_ZOOM_IN: KeyCode = KeyCode::Equal;
const CAM_ZOOM_OUT: KeyCode = KeyCode::Minus;
const CAM_FOCUS_NEXT_PLANET: KeyCode = KeyCode::ArrowRight;
const CAM_FOCUS_PREV_PLANET: KeyCode = KeyCode::ArrowLeft;
const CAM_FOCUS_PARENT: KeyCode = KeyCode::ArrowUp;
const CAM_FOCUS_SATELLITES: KeyCode = KeyCode::ArrowDown;
const CAM_MAX_PITCH: f32 = 1.55; // rad
const CAM_ROTATE_SPEED: f32 = 0.8; // rad/s
const CAM_MIN_DISTANCE: f32 = 0.3;
const CAM_MAX_DISTANCE: f32 = 1000000.0;
const CAM_ZOOM_SPEED: f32 = 0.08;
const CAM_CENTERED_ON_DEFAULT: usize = HANDLE_JUPITER as usize;

const ORBIT_SEGMENTS: usize = 100;
const ORBIT_COLOR: Color = Color::srgb(0.5, 1.0, 0.0);
const PERIAPSIS_COLOR: Color = Color::srgb(1.0, 0.5, 0.0);
const APOAPSIS_COLOR: Color = Color::srgb(0.0, 0.5, 1.0);
const PLANET_COLOR: Color = Color::srgb(1.0, 0.1, 0.5);
const SOI_COLOR: Color = Color::srgb(1.0, 1.0, 1.0);
const APSIS_SIZE_MIN: f32 = 0.01;
const APSIS_SIZE_MAX: f32 = 2000.0;

type Database = BevyPlanetDatabase<usize>;

#[derive(Resource)]
struct UiElements {
	focused_planet_name: Entity,
}

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

fn setup_ui(mut commands: Commands) {
	let focused_planet = commands.spawn((
		Text::new("Planet Name"),
		Node{
			position_type: PositionType::Absolute,
			bottom: Val::ZERO,
			left: Val::Auto,
			right: Val::Auto,
			justify_self: JustifySelf::Center,
			border: UiRect::all(Val::Px(2.0)),
			..default()
		},
	)).id();
	commands.insert_resource(UiElements{
		focused_planet_name: focused_planet,
	});
}

fn update_planet_focus_ui(
	mut elements: Query<&mut Text>,
	camera_parents: Query<&CameraParent>,
	database: Res<Database>,
	handles: Res<UiElements>,
) {
	let camera_parent = camera_parents.single();
	let entry = database.get_entry(&camera_parent.centered_body);
	let mut text = elements.get_mut(handles.focused_planet_name).unwrap();
	text.0 = entry.name.clone();
}

fn process_camera_input(
	keyboard: Res<ButtonInput<KeyCode>>, time: Res<Time>,
	mut camera_parents: Query<&mut CameraParent>,
){
	let delta = time.delta_secs();
	let mut camera_parent = camera_parents.single_mut();
	// handle rotation inputs
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
	camera_parent.pitch = camera_parent.pitch.clamp(-CAM_MAX_PITCH, CAM_MAX_PITCH);
	// handle zoom inputs
	if keyboard.pressed(CAM_ZOOM_IN) {
		camera_parent.zoom -= CAM_ZOOM_SPEED * delta;
	}
	if keyboard.pressed(CAM_ZOOM_OUT) {
		camera_parent.zoom += CAM_ZOOM_SPEED * delta;
	}
	camera_parent.zoom = camera_parent.zoom.clamp(0.0, 1.0);
}

fn process_navigation_controls(
	mut camera_parents: Query<&mut CameraParent>,
	keyboard: Res<ButtonInput<KeyCode>>,
	database: Res<Database>,
) {
	let mut camera_parent = camera_parents.single_mut();
	if keyboard.just_pressed(CAM_FOCUS_SATELLITES) {
		let children = database.get_satellites(&camera_parent.centered_body);
		if !children.is_empty() {
			camera_parent.centered_body = children[0];
		}
	}
	if keyboard.just_pressed(CAM_FOCUS_PARENT) {
		let entry = database.get_entry(&camera_parent.centered_body);
		if let Some(parent_handle) = entry.parent {
			camera_parent.centered_body = parent_handle;
		}
	}
	if keyboard.just_pressed(CAM_FOCUS_NEXT_PLANET) {
		let entry = database.get_entry(&camera_parent.centered_body);
		if let Some(parent_handle) = entry.parent {
			let siblings = database.get_satellites(&parent_handle);
			if siblings.len() > 0 {
				let err_msg = format!("Siblings list did not contain handle {} (list: {:?})", camera_parent.centered_body, siblings);
				let mut index = siblings.binary_search(&camera_parent.centered_body).expect(&err_msg);
				index += 1;
				if index >= siblings.len() {
					index = 0;
				}
				camera_parent.centered_body = siblings[index];
			}
		}
	}
	if keyboard.just_pressed(CAM_FOCUS_PREV_PLANET) {
		let entry = database.get_entry(&camera_parent.centered_body);
		if let Some(parent_handle) = entry.parent {
			let siblings = database.get_satellites(&parent_handle);
			if siblings.len() > 0 {
				let err_msg = format!("Siblings list did not contain handle {} (list: {:?})", camera_parent.centered_body, siblings);
				let mut index = siblings.binary_search(&camera_parent.centered_body).expect(&err_msg);
				if index <= 0 {
					index = siblings.len();
				}
				index -= 1;
				camera_parent.centered_body = siblings[index];
			}
		}
	}
}

fn update_camera_position(
	mut camera_parents: Query<(&mut Transform, &CameraParent), Without<Camera3d>>,
	mut cameras: Query<&mut Transform, (With<Camera3d>, Without<CameraParent>)>,
){
	let (mut camera_parent_transform, camera_parent) = camera_parents.single_mut();
	let mut camera_transform = cameras.single_mut();
	let camera_rotation = Quat::from_axis_angle(Vec3::X, camera_parent.pitch);
	let camera_direction = camera_rotation * -Vec3::Z;
	let center_position = Vec3::ZERO;
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
	let origin_body = camera_parent.centered_body;
	let step = TAU / (ORBIT_SEGMENTS-1) as f32;
	for (handle, entry) in db.iter() {
		if let Some(parent_handle) = entry.parent {
			let failure_msg = format!("Failed to find relative position between origin body {} and relative body {}", origin_body, parent_handle);
			let parent_pos = db.relative_position(&origin_body, &parent_handle).expect(&failure_msg) * SCALE;
			let mut points: Vec<(f32, Vec3)> = Vec::new();
			// get orbit path
			for i in 0..ORBIT_SEGMENTS {
				let m = step * i as f32;
				let m_1 = step * (i+1) as f32;
				let pos = db.position_at_mean_anomaly(handle, m) * SCALE;
				points.push((m, parent_pos + pos));
				if m <= entry.mean_anomaly_at_epoch && m_1 >= entry.mean_anomaly_at_epoch {
					points.push((
						entry.mean_anomaly_at_epoch,
						parent_pos + db.position_at_mean_anomaly(handle, entry.mean_anomaly_at_epoch) * SCALE
					));
				}
			}
			for i in 0..points.len()-1 {
				let (m_0, p_0) = points[i];
				let (m_1, p_1) = points[i+1];
				let mut t_0 = (m_0 - entry.mean_anomaly_at_epoch) / TAU;
				let mut t_1 = (m_1 - entry.mean_anomaly_at_epoch) / TAU;
				while t_0 < 0.0 {
					t_0 += 1.0;
				}
				while t_1 <= 0.0 {
					t_1 += 1.0;
				}
				let c_0 = ORBIT_COLOR.with_alpha(t_0.powi(2));
				let c_1 = ORBIT_COLOR.with_alpha(t_1.powi(2));
				gizmos.line_gradient(p_0, p_1, c_0, c_1);
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

fn draw_planets(mut gizmos: Gizmos, db: Res<Database>, camera_parents: Query<&CameraParent>) {
	let camera_parent = camera_parents.single();
	let centered_body = camera_parent.centered_body;
	for (handle, entry) in db.iter() {
		let pos = db.relative_position(&centered_body, handle).unwrap() * SCALE;
		let soi_radius = db.radius_soi(handle);
		let info = entry.info.clone();
		// info!("Scale radius: {} units", info.radius_avg_km() * scale);
		gizmos.sphere(pos, soi_radius * SCALE, SOI_COLOR); // sphere of influence
		gizmos.sphere(pos, info.radius_avg_m() * SCALE, PLANET_COLOR);
	}
}

fn main() {
	App::new()
		.add_plugins(DefaultPlugins)
		.insert_resource(Database::default().with_solar_system())
		.add_systems(Startup, (setup_camera, setup_ui))
		.add_systems(Update, (
			draw_orbits, draw_planets,
			process_navigation_controls.before(update_camera_position),
			process_camera_input.before(update_camera_position),
			update_camera_position,
			update_planet_focus_ui.after(process_navigation_controls),
		))
		.run();
}