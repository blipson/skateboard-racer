use std::f32::consts::FRAC_PI_2;
use std::ops::Range;
use avian3d::prelude::*;
use bevy::input::gestures::*;
use bevy::input::mouse::*;
use bevy::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;

#[derive(Debug, Resource)]
struct CameraSettings {
    pub orbit_distance: f32,
    pub pitch_speed: f32,
    pub pitch_range: Range<f32>,
    pub roll_speed: f32,
    pub yaw_speed: f32,
}

impl Default for CameraSettings {
    fn default() -> Self {
        let pitch_limit = FRAC_PI_2 - 0.01;
        Self {
            orbit_distance: 20.0,
            pitch_speed: 0.003,
            pitch_range: -pitch_limit..pitch_limit,
            roll_speed: 1.0,
            yaw_speed: 0.004,
        }
    }
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    asset_server: Res<AssetServer>
) {
    commands.spawn(DirectionalLight {
        shadows_enabled: true,
        illuminance: 50000.,
        ..default()
    }).insert(Transform {
        rotation: Quat::from_rotation_x(-90.0f32.to_radians()),
        ..default()
    });

    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(10., 0., 0.).looking_at(Vec3::new(0., 0., 0.), Vec3::Y),
    ));

    commands.spawn((
        RigidBody::Static,
        Collider::cylinder(4.0, 0.1),
        Mesh3d(meshes.add(Cylinder::new(4.0, 0.1))),
        MeshMaterial3d(materials.add(Color::BLACK)),
        Transform::from_xyz(0., -0.5, 0.)
    ));

    commands.spawn(SceneRoot {
        0: asset_server.load("tryce.glb#Scene0"),
    }).insert(Transform {
        rotation: Quat::from_rotation_y(-90.0f32.to_radians()),
        ..default()
    }).insert(RigidBody::Dynamic)
        .insert(Collider::cuboid(2., 2., 2.));

    commands.spawn(SceneRoot {
        0: asset_server.load("skateboard.glb#Scene0"),
    }).insert(Transform {
        scale: Vec3::new(0.0025, 0.0025, 0.0025),
        translation: Vec3::new(0., -0.1, 0.),
        ..default()
    }).insert(RigidBody::Dynamic)
        .insert(Collider::cuboid(0.1, 0.01, 0.1));
}

fn mouse_events(
    mut mouse_motion_events: EventReader<MouseMotion>,
    mut mouse_wheel_events: EventReader<MouseWheel>,
    mut camera_query: Query<&mut Transform, With<Camera3d>>,
                   mouse_motion: Res<AccumulatedMouseMotion>,
                   mouse_buttons: Res<ButtonInput<MouseButton>>,
                   camera_settings: Res<CameraSettings>,
) {
    for event in mouse_motion_events.read() {
        for mut camera_transform in camera_query.iter_mut() {
            let delta = mouse_motion.delta;

            let mut delta_pitch = 0.;
            let mut delta_yaw = 0.;

            if mouse_buttons.pressed(MouseButton::Left) {
                delta_pitch = delta.y * -camera_settings.pitch_speed;
                delta_yaw = delta.x * -camera_settings.yaw_speed;
            }
            let (yaw, pitch, roll) = camera_transform.rotation.to_euler(EulerRot::YXZ);
            let pitch = (pitch + delta_pitch).clamp(
                camera_settings.pitch_range.start,
                camera_settings.pitch_range.end,
            );
            let yaw = yaw + delta_yaw;
            camera_transform.rotation = Quat::from_euler(EulerRot::YXZ, yaw, pitch, roll);
        }
    }

    for event in mouse_wheel_events.read() {
        for mut camera_transform in camera_query.iter_mut() {
            let zoom_speed = 0.5;
            let zoom_amount = event.y * zoom_speed;
            let forward = camera_transform.rotation * Vec3::NEG_Z;
            camera_transform.translation += forward * zoom_amount;
        }
    }
}

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, PhysicsPlugins::default(), WorldInspectorPlugin::new(), PhysicsDebugPlugin::default()))
        .init_resource::<CameraSettings>()
        .add_systems(Startup, setup)
        .add_systems(Update, mouse_events)
        .run();
}
