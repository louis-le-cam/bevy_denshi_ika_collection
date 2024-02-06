use bevy::{
    core_pipeline::core_3d::Camera3dBundle,
    ecs::{
        bundle::Bundle,
        component::Component,
        query::{Changed, With},
        system::{Local, Query},
    },
    gizmos::gizmos::Gizmos,
    math::Vec3,
    reflect::Reflect,
    render::color::Color,
    transform::components::Transform,
};
use bevy_denshi_ika_gen_plugin::gen_plugin;
use bevy_xpbd_3d::{
    components::{Collider, Position, Rotation},
    plugins::spatial_query::{ShapeCaster, ShapeHits, SpatialQueryFilter},
};

gen_plugin! {
    pub CameraSpringArmPlugin;
    reflect(CameraSpringArm);
    systems(PostUpdate)((
        update_camera_spring_arm_shape_raycaster,
        update_camera_spring_arm_shape_raycaster,
        update_camera_spring_arm_shape_caster_transform,
        update_camera_spring_arm,
        update_camera_spring_arm_shape_caster_transform,
    ).chain());
}

#[derive(Bundle)]
pub struct CameraSpringArmBundle {
    pub camera_spring_arm: CameraSpringArm,
    pub shape_caster: ShapeCaster,
    pub position: Position,
    pub rotation: Rotation,
    pub camera_3d_bundle: Camera3dBundle,
}

#[derive(Component, Reflect)]
pub struct CameraSpringArm {
    pub distance: f32,
    pub yaw: f32,
    pub pitch: f32,
    pub camera_yaw: f32,
    pub camera_pitch: f32,
    pub camera_roll: f32,
    pub camera_radius: f32,
    #[reflect(ignore)]
    pub query_filter: SpatialQueryFilter,
}

fn update_camera_spring_arm_shape_raycaster(
    mut cameras: Query<(&mut ShapeCaster, &CameraSpringArm), Changed<CameraSpringArm>>,
) {
    for (mut shape_caster, camera) in &mut cameras {
        shape_caster.query_filter = camera.query_filter.clone();
        shape_caster.max_time_of_impact = camera.distance;
        shape_caster.shape = Collider::ball(camera.camera_radius);
        shape_caster.max_hits = 1;
    }
}

fn update_camera_spring_arm_shape_caster_transform(
    mut cameras: Query<(&mut ShapeCaster, &Transform), (With<CameraSpringArm>, Changed<Transform>)>,
    mut origin_direction: Local<(Vec3, Vec3)>,
    mut gizmos: Gizmos,
) {
    for (mut shape_caster, transform) in &mut cameras {
        shape_caster.origin = -transform.translation;
        shape_caster.direction = -transform.forward();
        origin_direction.0 = shape_caster.origin;
        origin_direction.1 = shape_caster.direction;
    }

    gizmos.ray(origin_direction.0, origin_direction.1, Color::RED);
}

fn update_camera_spring_arm(
    mut cameras: Query<(&mut Transform, &CameraSpringArm, &ShapeCaster, &ShapeHits)>,
) {
    for (mut transform, camera_spring_arm, shape_caster, hits) in &mut cameras {
        dbg!(&hits);
        let time_of_impact = match hits.iter().next() {
            Some(hit) => hit.time_of_impact,
            None => camera_spring_arm.distance,
        };

        // dbg!(time_of_impact);

        transform.translation = shape_caster.direction * time_of_impact;
    }
}
