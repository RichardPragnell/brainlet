use bevy::prelude::*;
use bevy_rapier3d::prelude::*;
use smooth_bevy_cameras::{
    controllers::unreal::{UnrealCameraBundle, UnrealCameraController, UnrealCameraPlugin},
    LookTransformPlugin,
};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::default())
        .add_plugin(RapierDebugRenderPlugin::default())
        .add_plugin(LookTransformPlugin)
        .add_plugin(UnrealCameraPlugin::default())
        .add_startup_system(setup_camera)
        .add_startup_system(setup_physics)
        .add_system(print_ball_altitude)
        .run();
}

fn setup_camera(mut commands: Commands) {
    commands
        .spawn_bundle(Camera3dBundle::default())
        .insert_bundle(UnrealCameraBundle::new(
            UnrealCameraController::default(),
            Vec3::new(-5.0, 5.0, 10.0),
            Vec3::new(0., 0., 0.),
        ));
}

fn setup_physics(mut commands: Commands) {
    /* Create the ground. */
    commands
        .spawn()
        .insert(Collider::cuboid(100.0, 0.1, 100.0))
        .insert_bundle(TransformBundle::from(Transform::from_xyz(0.0, 0.0, 0.0)));

    let drone_width = 0.75;
    let drone_height = 0.1;

    /* Create the bouncing ball. */
    commands
        .spawn()
        .insert(RigidBody::Dynamic)
        .insert(GravityScale(0.5))
        .with_children(|children| {
            children.spawn()
                .insert(Collider::cuboid(drone_width, drone_height, drone_height));
            children.spawn()
                .insert(Collider::cuboid(drone_height, drone_height, drone_width));
            children.spawn()
                .insert(Collider::cylinder(drone_height, drone_height))
                .insert(ColliderDebugColor(Color::Rgba { red: 0.0, green: 1.0, blue: 1.0, alpha: 1.0 }))
                .insert_bundle(TransformBundle::from(Transform::from_xyz(drone_width, 0.0, 0.0)));
            children.spawn()
                .insert(Collider::cylinder(drone_height, drone_height))
                .insert(ColliderDebugColor(Color::Rgba { red: 0.0, green: 0.0, blue: 1.0, alpha: 1.0 }))
                .insert_bundle(TransformBundle::from(Transform::from_xyz(-drone_width, 0.0, 0.0)));
            children.spawn()
                .insert(Collider::cylinder(drone_height, drone_height))
                .insert(ColliderDebugColor(Color::Rgba { red: 1.0, green: 1.0, blue: 0.0, alpha: 1.0 }))
                .insert_bundle(TransformBundle::from(Transform::from_xyz(0.0, 0.0, drone_width)));
            children.spawn()
                .insert(Collider::cylinder(drone_height, drone_height))
                .insert(ColliderDebugColor(Color::Rgba { red: 0.0, green: 1.0, blue: 0.0, alpha: 1.0 }))
                // .insert(ColliderMassProperties::Mass(100.0))
                .insert_bundle(TransformBundle::from(Transform::from_xyz(0.0, 0.0, -drone_width)));
        })
        .insert(Damping { linear_damping: 1.0, angular_damping: 1.0 })
        // .insert(Restitution::coefficient(0.7))
        // .insert(ExternalImpulse  {
        //     impulse: Vec3::new(0.0, 0.0, 5.0),
        //     torque_impulse: Vec3::new(0.0, 0.0, 0.0),
        // })
        
        .insert(ExternalForce  {
            force: Vec3::new(0.0, 0.0, 0.0),
            torque: Vec3::new(0.0, 0.0, 1.5),
        })
        .insert_bundle(TransformBundle::from(Transform::from_xyz(0.0, 5.0, 0.0)));
}

fn print_ball_altitude(positions: Query<&Transform, With<RigidBody>>) {
    for transform in positions.iter() {
        println!("Ball altitude: {}", transform.translation.y);
    }
}
