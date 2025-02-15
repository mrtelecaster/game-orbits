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
const CAM_CENTERED_ON_DEFAULT: usize = HANDLE_EARTH as usize;
const CHANGE_VIEW_ORBITS: KeyCode = KeyCode::Digit1;
const TOGGLE_VIEW_SOI: KeyCode = KeyCode::Digit2;
const TOGGLE_VIEW_APSIS: KeyCode = KeyCode::Digit3;
const TOGGLE_VIEW_AXES: KeyCode = KeyCode::Digit4;

const ORBIT_SEGMENTS: usize = 100;
const ORBIT_COLOR: Color = Color::srgb(0.5, 1.0, 0.0);
const PERIAPSIS_COLOR: Color = Color::srgb(1.0, 0.5, 0.0);
const APOAPSIS_COLOR: Color = Color::srgb(0.0, 0.5, 1.0);
const PLANET_COLOR: Color = Color::srgb(1.0, 0.1, 0.5);
const SOI_COLOR: Color = Color::srgb(1.0, 1.0, 1.0);
const APSIS_SIZE_MIN: f32 = 0.01;
const APSIS_SIZE_MAX: f32 = 2000.0;
const AXIS_SIZE_MIN: f32 = 0.4;
const AXIS_SIZE_MAX: f32 = 20000.0;

type Database = BevyPlanetDatabase<usize>;

#[derive(Clone, Copy, PartialEq)]
enum OrbitViewMode {
	All,
	Children,
	Selected,
}
impl OrbitViewMode {
	pub fn next(&self) -> Self {
		match self {
			Self::All => Self::Children,
			Self::Children => Self::Selected,
			Self::Selected => Self::All,
		}
	}
	pub fn to_str(&self) -> &'static str {
		match self {
			Self::All => "All",
			Self::Children => "Children",
			Self::Selected => "Selected",
		}
	}
}

#[derive(Resource)]
struct UiElements {
	parent_planet_name: Entity,
	focused_planet_name: Entity,
	satellite_name: Entity,
	prev_planet_name: Entity,
	next_planet_name: Entity,
	control_view_orbits: Entity,
	control_view_soi: Entity,
	control_view_apsis: Entity,
	control_view_axes: Entity,
}

#[derive(Component)]
struct CameraParent {
	pub centered_body: usize,
	pub yaw: f32,
	pub pitch: f32,
	pub zoom: f32,
	pub view_apsis: bool,
	pub view_soi: bool,
	pub view_axes: bool,
	pub view_orbit: OrbitViewMode
}
impl CameraParent {
	pub fn centered_on(mut self, handle: usize) -> Self {
		self.centered_body = handle;
		self
	}
}
impl Default for CameraParent {
	fn default() -> Self {
		Self{ yaw: 0.0, pitch: 0.0, zoom: 0.1, centered_body: 0, view_apsis: false, view_soi: true, view_axes: false, view_orbit: OrbitViewMode::All }
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
	// controls text
	let font = TextFont{
		font_size: 14.0,
		..default()
	};
	let control_camera_up = commands.spawn((Text::new("[W] Rotate camera up"), font.clone())).id();
	let control_camera_down = commands.spawn((Text::new("[S] Rotate camera down"), font.clone())).id();
	let control_camera_left = commands.spawn((Text::new("[A] Rotate camera left"), font.clone())).id();
	let control_camera_right = commands.spawn((Text::new("[D] Rotate camera right"), font.clone())).id();
	let control_zoom_in = commands.spawn((Text::new("[+] Zoom in"), font.clone())).id();
	let control_zoom_out = commands.spawn((Text::new("[-] Zoom out"), font.clone())).id();
	let control_view_orbits = commands.spawn((Text::new("[1] Change orbit visibility: All orbits"), font.clone())).id();
	let control_view_soi = commands.spawn((Text::new("[2] Toggle SOI visibility: Visible"), font.clone())).id();
	let control_view_apsis = commands.spawn((Text::new("[3] Toggle -apsis visibility: Visible"), font.clone())).id();
	let control_view_axes = commands.spawn((Text::new("[4] Toggle axis visibility: Visible"), font.clone())).id();
	let _controls_container = commands.spawn(Node{
		position_type: PositionType::Absolute,
		left: Val::Px(0.0),
		top: Val::Px(0.0),
		flex_direction: FlexDirection::Column,
		padding: UiRect::axes(Val::Px(6.0), Val::Px(3.0)),
		..default()
	}).add_child(control_camera_up).add_child(control_camera_down).add_child(control_camera_left).add_child(control_camera_right)
		.add_child(control_zoom_in).add_child(control_zoom_out)
		.add_child(control_view_orbits).add_child(control_view_soi).add_child(control_view_apsis).add_child(control_view_axes)
		.id();
	// navigation text
	let text_alpha = 0.4;
	let transparent_text_color = Color::linear_rgba(1.0, 1.0, 1.0, text_alpha);
	let parent_planet_name = commands.spawn((
		Text::new("Parent"),
		TextColor::from(transparent_text_color),
	)).id();
	let prev_planet_name = commands.spawn((
		Text::new("Prev Planet"),
		TextColor::from(transparent_text_color),
	)).id();
	let focused_planet_name = commands.spawn((
		Text::new("Focused"),
	)).id();
	let next_planet_name = commands.spawn((
		Text::new("Next Planet"),
		TextColor::from(transparent_text_color),
	)).id();
	let focused_planet_row = commands.spawn(Node{
		flex_direction: FlexDirection::Row,
		column_gap: Val::Px(16.0),
		..default()
	}).add_child(prev_planet_name).add_child(focused_planet_name).add_child(next_planet_name).id();
	let satellite_name = commands.spawn((
		Text::new("Satellite"),
		TextColor::from(transparent_text_color),
	)).id();
	let _planet_nav_container = commands.spawn(
		Node{
			position_type: PositionType::Absolute,
			bottom: Val::ZERO,
			left: Val::Auto,
			right: Val::Auto,
			justify_self: JustifySelf::Center,
			align_items: AlignItems::Center,
			flex_direction: FlexDirection::Column,
			..default()
		},
	).add_child(parent_planet_name)
		.add_child(focused_planet_row)
		.add_child(satellite_name)
		.id();
	commands.insert_resource(UiElements{
		parent_planet_name,
		focused_planet_name,
		satellite_name,
		prev_planet_name,
		next_planet_name,
		control_view_orbits,
		control_view_soi,
		control_view_apsis,
		control_view_axes,
	});
}

fn update_controls_ui(
	mut elements: Query<&mut Text>,
	camera_parents: Query<&CameraParent>,
	handles: Res<UiElements>,
){
	let camera_parent = camera_parents.single();
	let mut text = elements.get_mut(handles.control_view_orbits).unwrap();
	text.0 = format!("[1] Change orbit view mode: {}", camera_parent.view_orbit.to_str());
	text = elements.get_mut(handles.control_view_soi).unwrap();
	let visibility_str = match camera_parent.view_soi {
		true => "Visible",
		false => "Hidden",
	};
	text.0 = format!("[2] Toggle SOI visibility: {}", visibility_str);
	text = elements.get_mut(handles.control_view_apsis).unwrap();
	let visibility_str = match camera_parent.view_apsis {
		true => "Visible",
		false => "Hidden",
	};
	text.0 = format!("[3] Toggle -apsis visibility: {}", visibility_str);
	let visibility_str = match camera_parent.view_axes {
		true => "Visible",
		false => "Hidden",
	};
	text = elements.get_mut(handles.control_view_axes).unwrap();
	text.0 = format!("[4] Toggle axis visibility: {}", visibility_str);
}

fn update_planet_focus_ui(
	mut elements: Query<&mut Text>,
	camera_parents: Query<&CameraParent>,
	database: Res<Database>,
	handles: Res<UiElements>,
) {
	let empty_string = String::from(" ");
	let camera_parent = camera_parents.single();
	let entry = database.get_entry(&camera_parent.centered_body);
	// focused planet text
	let mut text = elements.get_mut(handles.focused_planet_name).unwrap();
	text.0 = entry.name.clone();
	// parent planet text
	text = elements.get_mut(handles.parent_planet_name).unwrap();
	if let Some(parent_handle) = entry.parent {
		let parent = database.get_entry(&parent_handle);
		text.0 = parent.name.clone();
	} else {
		text.0 = empty_string.clone();
	}
	// satellite planet text
	text = elements.get_mut(handles.satellite_name).unwrap();
	let satellites = database.get_satellites(&camera_parent.centered_body);
	if satellites.is_empty() {
		text.0 = empty_string.clone();
	} else {
		let first_satellite = database.get_entry(&satellites[0]);
		text.0 = first_satellite.name.clone();
	}
	// prev/next planet
	if let Some(parent_handle) = entry.parent {
		let siblings = database.get_satellites(&parent_handle);
		let index = siblings.binary_search(&camera_parent.centered_body).unwrap();
		let prev_index;
		if index > 0 {
			prev_index = index - 1;
		} else {
			prev_index = siblings.len() - 1;
		}
		let next_index;
		if index < siblings.len() - 1 {
			next_index = index + 1;
		} else {
			next_index = 0;
		}
		let prev_handle = siblings[prev_index];
		let prev_entry = database.get_entry(&prev_handle);
		text = elements.get_mut(handles.prev_planet_name).unwrap();
		text.0 = prev_entry.name.clone();
		let next_handle = siblings[next_index];
		let next_entry = database.get_entry(&next_handle);
		text = elements.get_mut(handles.next_planet_name).unwrap();
		text.0 = next_entry.name.clone();
	} else {
		text = elements.get_mut(handles.prev_planet_name).unwrap();
		text.0 = empty_string.clone();
		text = elements.get_mut(handles.next_planet_name).unwrap();
		text.0 = empty_string.clone();
	}
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

fn process_visibility_input(
	mut camera_parents: Query<&mut CameraParent>,
	keyboard: Res<ButtonInput<KeyCode>>,
){
	let mut camera_parent = camera_parents.single_mut();
	if keyboard.just_pressed(TOGGLE_VIEW_APSIS) {
		camera_parent.view_apsis = !camera_parent.view_apsis;
	}
	if keyboard.just_pressed(TOGGLE_VIEW_SOI) {
		camera_parent.view_soi = !camera_parent.view_soi;
	}
	if keyboard.just_pressed(CHANGE_VIEW_ORBITS) {
		camera_parent.view_orbit = camera_parent.view_orbit.next();
	}
	if keyboard.just_pressed(TOGGLE_VIEW_AXES) {
		camera_parent.view_axes = !camera_parent.view_axes;
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
		let heirarchy = db.get_parents(&handle);
		if let Some(parent_handle) = entry.parent {
			let view_all = camera_parent.view_orbit == OrbitViewMode::All;
			let view_heirarchy = camera_parent.view_orbit == OrbitViewMode::Children && heirarchy.contains(&camera_parent.centered_body);
			let view_selected = camera_parent.view_orbit == OrbitViewMode::Selected && *handle == camera_parent.centered_body;
			if view_all || view_heirarchy || view_selected {
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
				if camera_parent.view_apsis {
					// draw apoapsis/periapsis
					let pos_periapsis = db.position_at_mean_anomaly(handle, 0.0) * SCALE;
					let pos_apoapsis = db.position_at_mean_anomaly(handle, PI) * SCALE;
					let apsis_size = APSIS_SIZE_MIN.lerp(APSIS_SIZE_MAX, camera_parent.zoom.powf(3.0));
					gizmos.sphere(pos_periapsis + parent_pos, apsis_size, PERIAPSIS_COLOR);
					gizmos.sphere(pos_apoapsis + parent_pos, apsis_size, APOAPSIS_COLOR);
				}
			}
		}
	}
}

fn draw_planets(mut gizmos: Gizmos, db: Res<Database>, camera_parents: Query<&CameraParent>) {
	let camera_parent = camera_parents.single();
	let centered_body = camera_parent.centered_body;
	for (handle, entry) in db.iter() {
		let info = entry.info.clone();
		let pos = db.relative_position(&centered_body, handle).unwrap() * SCALE;
		let rot = Quat::from_axis_angle(Vec3::X, info.axial_tilt_rad());
		let iso = Isometry3d::new(pos, rot);
		// info!("Scale radius: {} units", info.radius_avg_km() * scale);
		gizmos.sphere(iso, info.radius_avg_m() * SCALE, PLANET_COLOR);
		if camera_parent.view_soi {
			let soi_radius = db.radius_soi(handle);
			gizmos.sphere(pos, soi_radius * SCALE, SOI_COLOR); // sphere of influence
		}
		if camera_parent.view_axes {
			let axis_size = AXIS_SIZE_MIN.lerp(AXIS_SIZE_MAX, camera_parent.zoom.powi(3));
			gizmos.axes(iso, axis_size);
		}
	}
}

fn main() {
	App::new()
		.add_plugins(DefaultPlugins)
		.insert_resource(Database::default().with_solar_system())
		.add_systems(Startup, (setup_camera, setup_ui))
		.add_systems(Update, (
			process_visibility_input,
			draw_orbits.after(process_visibility_input), draw_planets.after(process_visibility_input),
			process_navigation_controls.before(update_camera_position),
			process_camera_input.before(update_camera_position),
			update_camera_position,
			update_controls_ui.after(process_visibility_input),
			update_planet_focus_ui.after(process_navigation_controls),
		))
		.run();
}