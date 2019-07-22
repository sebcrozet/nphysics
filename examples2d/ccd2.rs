extern crate nalgebra as na;

use na::{Point2, Vector2, Isometry2};
use ncollide2d::shape::{Cuboid, ShapeHandle};
use nphysics2d::object::{ColliderDesc, RigidBodyDesc, DefaultBodySet, DefaultColliderSet, Ground, BodyPartHandle};
use nphysics2d::force_generator::DefaultForceGeneratorSet;
use nphysics2d::joint::DefaultJointConstraintSet;
use nphysics2d::world::{DefaultDynamicWorld, DefaultColliderWorld};
use nphysics2d::material::{MaterialHandle, BasicMaterial};
use nphysics2d::math::Velocity;
use nphysics_testbed2d::Testbed;


pub fn init_world(testbed: &mut Testbed) {
    /*
     * World
     */
    let dynamic_world = DefaultDynamicWorld::new(Vector2::new(0.0, -9.81));
    let collider_world = DefaultColliderWorld::new();
    let mut bodies = DefaultBodySet::new();
    let mut colliders = DefaultColliderSet::new();
    let joint_constraints = DefaultJointConstraintSet::new();
    let force_generators = DefaultForceGeneratorSet::new();

    /*
     * Ground
     */
    let ground_size = 25.0;
    let ground_shape =
        ShapeHandle::new(Cuboid::new(Vector2::new(ground_size, 0.1)));

    let material = MaterialHandle::new(BasicMaterial::new(
                                           0.0, 0.5));

    let ground_handle = bodies.insert(Ground::new());
    let co = ColliderDesc::new(ground_shape.clone())
        .ccd_enabled(true)
        .material(material.clone())
        .build(BodyPartHandle(ground_handle, 0));
    colliders.insert(co);

    let co = ColliderDesc::new(ground_shape.clone())
        .position(Isometry2::new(Vector2::new(-3.0, 0.0), 3.14 / 2.0))
        .ccd_enabled(true)
        .material(material.clone())
        .build(BodyPartHandle(ground_handle, 0));
    colliders.insert(co);

    let co = ColliderDesc::new(ground_shape.clone())
        .position(Isometry2::new(Vector2::new(3.0, 0.0), 3.14 / 2.0))
        .ccd_enabled(true)
        .material(material.clone())
        .build(BodyPartHandle(ground_handle, 0));
    colliders.insert(co);

    let co = ColliderDesc::new(ground_shape)
        .position(Isometry2::translation(0.0, 10.0))
        .ccd_enabled(true)
        .material(material.clone())
        .build(BodyPartHandle(ground_handle, 0));
    colliders.insert(co);

    /*
     * Create the balls
     */
    let num = 5;
    let rad = 0.1;

    let shape = ShapeHandle::new(Cuboid::new(Vector2::new(rad * 4.0, rad))); // Ball::new(rad));

    let shift = (rad + ColliderDesc::<f32>::default_margin()) * 2.0;
    let centerx = shift * (num as f32) / 2.0 + 1.0;
    let centery = shift / 2.0 + 6.0;

    for i in 0usize..num {
        for j in 0..num {
            let x = i as f32 * shift * 4.0 - centerx;
            let y = j as f32 * shift + centery;

            // Build the rigid body.
            let rb = RigidBodyDesc::new()
                .translation(Vector2::new(x, y))
                .velocity(Velocity::linear(100.0, -100.0))
                .build();
            let rb_handle = bodies.insert(rb);

            // Build the collider.
            let co = ColliderDesc::new(shape.clone())
//                .ccd_enabled(true)
                .density(1.0)
                .build(BodyPartHandle(rb_handle, 0));
            colliders.insert(co);
        }
    }

    /*
     * Set up the testbed.
     */
    testbed.set_ground_handle(Some(ground_handle));
    testbed.set_world(dynamic_world, collider_world, bodies, colliders, joint_constraints, force_generators);
    testbed.look_at(Point2::new(0.0, -2.5), 95.0);
}

fn main() {
    let testbed = Testbed::from_builders(0, vec![
        ("CCD", init_world),
    ]);
    testbed.run()
}