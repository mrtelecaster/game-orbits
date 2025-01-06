//! Puts a camera in low earth orbit at about the same altitude as the
//! international space station

use std::f32::consts::PI;

use bevy::{prelude::*, pbr::wireframe::{Wireframe, WireframePlugin}};
use game_orbits::{constants::*, Body};

const SCALE: f32 = 1.0 / 1_000_000.0;
const ALTITUDE_KM: f32 = 400.0;
const PLANET_ROTATE_SPEED: f32 = 0.1;


#[derive(Component)]
struct Planet;

#[derive(Component)]
struct FramerateCounter {
	pub frequency: f32,
	time: f32,
	measurements: Vec<f32>,
}
impl FramerateCounter {
	fn new(freq: f32) -> Self {
		Self{ frequency: freq, time: 0.0, measurements: Vec::new() }
	}
	/// Adds `delta` seconds to the counter's timer and returns the average framerate if enough
	/// seconds have elapsed
	fn add_time(&mut self, delta: f32) -> Option<f32> {
		self.time += delta;
		self.measurements.push(delta);
		if self.time >= self.frequency {
			let mut framerate_total = 0.0;
			for measurement in &self.measurements {
				let framerate = 1.0 / measurement;
				framerate_total += framerate;
			}
			let framerate_average = framerate_total / (self.measurements.len() as f32);
			self.time -= self.frequency;
			self.measurements.clear();
			return Some(framerate_average);
		} else {
			return None;
		}
	}
}
impl Default for FramerateCounter {
	fn default() -> Self {
		Self::new(0.3)
	}
}


fn main() {
    App::new()
        .add_plugins((DefaultPlugins, WireframePlugin))
        .add_systems(Startup, (setup_earth, setup_camera, setup_sun, setup_ui))
        .add_systems(Update, (process_rotate_planet, process_framerate_counter, render_gizmos))
        .run();
}

fn setup_earth(
    mut commands: Commands, asset_server: ResMut<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>, mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let earth = Body::new_earth();
    let equatorial_radius_engine = earth.radius_equator_km() * CONVERT_KM_TO_M * SCALE;
    let polar_radius_engine = earth.radius_polar_km() * CONVERT_KM_TO_M * SCALE;
    let mesh = Sphere::new(1.0).mesh().uv(100, 50);
    let material = StandardMaterial{
        base_color_texture: Some(asset_server.load("earth_albedo.jpeg")),
        ..default()
    };
    info!("Engine planet radius: equatorial {} polar {}", equatorial_radius_engine, polar_radius_engine);
    commands.spawn((
        Mesh3d(meshes.add(mesh)),
        MeshMaterial3d(materials.add(material)),
        Transform::default()
            .with_scale(Vec3::new(equatorial_radius_engine, polar_radius_engine, equatorial_radius_engine))
            .with_rotation(Quat::from_axis_angle(Vec3::X, - PI / 2.0)),
        Planet,
		Wireframe,
    ));
}

fn setup_sun(mut commands: Commands) {
    let sun_direction: Dir3 = Dir3::new(Vec3::new(-1.0, -0.1, 0.2).normalize()).unwrap();
    commands.spawn((
        DirectionalLight::default(),
        Transform::default().looking_to(sun_direction, Vec3::Y),
    ));
}

fn setup_camera(mut commands: Commands) {
    let earth = Body::new_earth();
    let planet_radius_km = earth.radius_equator_km();
    let orbit_radius_km = planet_radius_km + ALTITUDE_KM;
    let orbit_radius_engine = orbit_radius_km * CONVERT_KM_TO_M * SCALE;
    info!("Engine orbit radius: {}", orbit_radius_engine);
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(orbit_radius_engine, 0.0, 0.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));
}

fn setup_ui(mut commands: Commands){
	// framerate counter
	commands.spawn((
		Text::new("Framerate: 999.9 fr/s"),
		Node{
			position_type: PositionType::Absolute,
			right: Val::ZERO,
			top: Val::ZERO,
			..default()
		},
		FramerateCounter::default(),
	));
}

fn process_rotate_planet(
    input: Res<ButtonInput<KeyCode>>, time: Res<Time>,
    mut planets: Query<&mut Transform, With<Planet>>
){
    let mut rotation_input = 0.0;
    if input.pressed(KeyCode::ArrowRight) {
        rotation_input += 1.0;
    } else if input.pressed(KeyCode::ArrowLeft) {
        rotation_input -= 1.0;
    }
    let rotation = rotation_input * PLANET_ROTATE_SPEED * time.delta_secs();
    for mut transform in &mut planets {
        transform.rotate(Quat::from_axis_angle(Vec3::Y, rotation));
    }
}

fn process_framerate_counter(
	mut text_nodes: Query<(&mut Text, &mut FramerateCounter), With<Node>>,
	time: Res<Time>,
){
	let delta = time.delta_secs();
	for (mut text, mut counter) in &mut text_nodes {
		if let Some(framerate) = counter.add_time(delta) {
			let message = format!("Framerate: {:.1} fr/s", framerate);
			text.0 = message;
		}
	}
}

fn render_gizmos(mut gizmos: Gizmos) {
	gizmos.axes(Transform::default(), 1.0);
}
