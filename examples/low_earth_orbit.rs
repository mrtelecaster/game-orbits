//! Puts a camera in low earth orbit at about the same altitude as the
//! international space station

use bevy::prelude::*;
use game_orbits::{constants::*, Body, CONVERT_KM_TO_M};

const SCALE: f32 = 1.0 / 1_000_000.0;
const ALTITUDE_KM: f32 = 400.0;


fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, (setup_earth, setup_camera, setup_sun))
        .run();
}

fn setup_earth(
    mut commands: Commands, mut asset_server: ResMut<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>, mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let earth = Body::new_earth();
    let equatorial_radius_engine = earth.radius_equator_km() * CONVERT_KM_TO_M * SCALE;
    let polar_radius_engine = earth.radius_polar_km() * CONVERT_KM_TO_M * SCALE;
    let mesh = meshes.add(Sphere::new(1.0).mesh().uv(8, 8));
    let material = materials.add(StandardMaterial::default());
    info!("Engine planet radius: equatorial {} polar {}", equatorial_radius_engine, polar_radius_engine);
    commands.spawn((
        Mesh3d(mesh),
        MeshMaterial3d(material),
        Transform::default().with_scale(Vec3::new(equatorial_radius_engine, polar_radius_engine, equatorial_radius_engine)),
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
