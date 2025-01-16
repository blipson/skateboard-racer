use bevy::prelude::{App, AssetServer, Camera3dBundle, Commands, DirectionalLight, Quat, Res, Scene, SceneBundle, SceneRoot, Startup, Transform, Update, Vec3};
use bevy::utils::default;
use bevy::DefaultPlugins;

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(DirectionalLight {
        color: Default::default(),
        shadows_enabled: true,
        shadow_depth_bias: 0.0,
        illuminance: 50000.,
        shadow_normal_bias: 0.0,
        ..default()
    }).insert(Transform {
        rotation: Quat::from_rotation_x(-90.0f32.to_radians()),
        ..default()
    },);

    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(0., 2., -7.).looking_at(Vec3::new(0., 5., 50.), Vec3::Y),
        ..default()
    });

    commands.spawn(SceneRoot {
        0: asset_server.load("tryce.glb#Scene0"),
    }).insert(Transform {
        rotation: Quat::from_rotation_y(-90.0f32.to_radians()),
        ..default()
    });

    commands.spawn(SceneRoot {
        0: asset_server.load("skateboard.glb#Scene0"),
    }).insert(Transform {
        scale: Vec3::new(0.0025, 0.0025, 0.0025),
        translation: Vec3::new(0., -0.1, 0.),
        ..default()
    });
}

fn update() {

}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Update, update)
        .run();
}
