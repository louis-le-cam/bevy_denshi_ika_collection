use bevy::prelude::{
    shape, Assets, Camera3dBundle, Color, Commands, Mesh, PbrBundle, ResMut, StandardMaterial,
};
use bevy_denshi_ika_camera_3d_controller::{
    flycam::{FlyCameraController, FlyCameraInputs},
    Camera3dControllerPlugin,
};
use bevy_denshi_ika_gen_plugin::gen_plugin;
use bevy_editor_pls::EditorPlugin;

gen_plugin! {
    main;
    plugins(DefaultPlugins, Camera3dControllerPlugin, EditorPlugin::default());
    systems(Startup)(spawn_camera);
}

fn spawn_camera(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn((
        Camera3dBundle::default(),
        FlyCameraController {
            speed: 1.0,
            inputs: FlyCameraInputs::default(),
        },
    ));

    commands.spawn(PbrBundle {
        mesh: meshes.add(shape::Cube::new(1.0).into()),
        material: materials.add(StandardMaterial {
            base_color: Color::RED,
            ..Default::default()
        }),
        ..Default::default()
    });
}
