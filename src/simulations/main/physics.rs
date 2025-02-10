use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::simulations::main::components::*;

pub fn setup_physics(mut commands: Commands) {
    // Load collision grid
    if let Ok(grid) = CollisionGrid::load_from_file("collision_grid.bin") {
        println!("Loaded collision grid: {}x{}", grid.grid[0].len(), grid.grid.len());
        commands.insert_resource(grid);
    } else {
        println!("Failed to load collision grid!");
    }

    let elevator_body = commands
        .spawn((
            RigidBody::Fixed,
            Collider::cuboid(2.52, 48.25),
            Transform::from_xyz(0.0, 0.0, 0.0),
            CollisionGroups::new(ELEVATOR, INTAKE),
        ))
        .id();

    let carriage = commands
        .spawn((
            RigidBody::Dynamic,
            Sleeping::disabled(),
            Collider::ball(5.08),
            Transform::from_xyz(5.08, -13.65, 0.0),
            CollisionGroups::new(Group::NONE, Group::NONE),
            Damping {
                linear_damping: 0.5,
                angular_damping: 0.5,
            },
            GravityScale(0.0),
        ))
        .id();

    let arm = commands
        .spawn((
            RigidBody::Dynamic,
            Sleeping::disabled(),
            Collider::cuboid(19.685, 2.52),
            Transform::from_xyz(24.765, -13.4, 0.0),
            CollisionGroups::new(Group::NONE, Group::NONE),
            Damping {
                linear_damping: 0.5,
                angular_damping: 0.5,
            },
            GravityScale(0.0),
        ))
        .id();

    let intake_pivot = commands
        .spawn((
            RigidBody::Dynamic,
            Sleeping::disabled(),
            LockedAxes::ROTATION_LOCKED,
            Collider::ball(5.08),
            Transform::from_xyz(44.45, -13.4, 0.0)
                .with_rotation(Quat::from_rotation_z(std::f32::consts::PI / 4.0)),
            CollisionGroups::new(Group::NONE, Group::NONE),
            Damping {
                linear_damping: 0.5,
                angular_damping: 0.5,
            },
            GravityScale(0.0),
        ))
        .id();

    let intake = commands
        .spawn((
            RigidBody::Dynamic,
            Sleeping::disabled(),
            Collider::cuboid(19.05, 6.675),
            Transform::from_xyz(35.258, -3.808, 0.0)
                .with_rotation(Quat::from_rotation_z(std::f32::consts::PI / 4.0)),
            CollisionGroups::new(INTAKE, ELEVATOR),
            Damping {
                linear_damping: 0.5,
                angular_damping: 0.5,
            },
            GravityScale(0.0),
            IntakeMarker,
            ExternalForce::default(),
        ))
        .id();

    let joint_elevator_carriage = PrismaticJointBuilder::new(Vec2::Y)
        .local_anchor1(Vec2::new(0.0, 0.0))
        .local_anchor2(Vec2::new(-5.08, 0.0))
        .limits([-13.65, 31.75]);

    let joint_carriage_arm = RevoluteJointBuilder::new()
        .local_anchor1(Vec2::new(0.0, 0.0))
        .local_anchor2(Vec2::new(-19.685, 0.0));

    let joint_arm_pivot = RevoluteJointBuilder::new()
        .local_anchor1(Vec2::new(19.685, 0.0))
        .local_anchor2(Vec2::new(0.0, 0.0));

    let joint_pivot_intake = FixedJointBuilder::new()
        .local_anchor1(Vec2::new(0.0, 0.0))
        .local_anchor2(Vec2::new(0.0, -13.0));

    let elevator_joint = commands
        .spawn(ImpulseJoint::new(elevator_body, joint_elevator_carriage))
        .set_parent(carriage)
        .id();

    let arm_joint = commands
        .spawn(ImpulseJoint::new(carriage, joint_carriage_arm))
        .set_parent(arm)
        .id();

    commands.insert_resource(MotorJoints {
        elevator: elevator_joint,
        arm: arm_joint,
        elevator_body: carriage,
        arm_body: arm,
    });

    commands
        .spawn(ImpulseJoint::new(arm, joint_arm_pivot))
        .set_parent(intake_pivot);

    commands
        .spawn(ImpulseJoint::new(intake_pivot, joint_pivot_intake))
        .set_parent(intake);
}
