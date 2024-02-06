use std::marker::PhantomData;

use bevy::{
    core::Name,
    hierarchy::BuildChildren,
    prelude::{
        shape, Assets, Camera3dBundle, Color, Commands, Mesh, PbrBundle, ResMut, SpatialBundle,
        StandardMaterial,
    },
};
use bevy_denshi_ika_camera_3d_controller::Camera3dControllerPlugin;
use bevy_denshi_ika_camera_spring_arm::{
    CameraSpringArm, CameraSpringArmBundle, CameraSpringArmPlugin,
};
use bevy_denshi_ika_gen_plugin::gen_plugin;
use bevy_editor_pls::EditorPlugin;
use bevy_xpbd_3d::{
    components::{Collider, Position, RigidBody, Rotation},
    plugins::{
        spatial_query::{ShapeCaster, SpatialQueryFilter},
        PhysicsDebugPlugin, PhysicsPlugins,
    },
};

gen_plugin! {
    main;
    plugins(
        DefaultPlugins,
        EditorPlugin::default(),
        PhysicsPlugins::default(),
        PhysicsDebugPlugin::default(),
        Camera3dControllerPlugin,
        CameraSpringArmPlugin,
    );
    systems(Startup)(spawn_camera);
}

fn spawn_camera(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // commands.spawn((
    //     Camera3dBundle::default(),
    //     FlyCameraController {
    //         speed: 1.0,
    //         inputs: FlyCameraInputs::default(),
    //     },
    // ));

    commands
        .spawn((
            Name::new("Spring arm base"),
            SpatialBundle::default(),
            Position::default(),
            Rotation::default(),
        ))
        .with_children(|p| {
            p.spawn((
                Name::new("Spring arm"),
                RigidBody::Static,
                CameraSpringArmBundle {
                    camera_spring_arm: CameraSpringArm {
                        distance: 4.0,
                        yaw: 0.0,
                        pitch: 0.0,
                        camera_yaw: 0.0,
                        camera_pitch: 0.0,
                        camera_roll: 0.0,
                        camera_radius: 1.0,
                        query_filter: SpatialQueryFilter::default(),
                    },
                    shape_caster: ShapeCaster::default(),
                    camera_3d_bundle: Camera3dBundle::default(),
                    position: Position::default(),
                    rotation: Rotation::default(),
                },
            ));
        });

    commands.spawn((
        PbrBundle {
            mesh: meshes.add(shape::Cube::new(1.0).into()),
            material: materials.add(StandardMaterial {
                base_color: Color::RED,
                ..Default::default()
            }),
            ..Default::default()
        },
        Collider::cuboid(1.0, 1.0, 1.0),
        RigidBody::Static,
    ));
}
