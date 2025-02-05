use bevy::prelude::*;

use kesko_core::interaction::groups::GroupDynamic;
use kesko_object_interaction::InteractiveBundle;
use kesko_physics::{
    collider::{ColliderPhysicalProperties, ColliderShape},
    force::Force,
    gravity::GravityScale,
    rigid_body::RigidBody,
};

pub struct Sphere;
impl Sphere {
    pub fn spawn(
        commands: &mut Commands,
        material: Handle<StandardMaterial>,
        transform: Transform,
        meshes: &mut Assets<Mesh>,
    ) {
        commands
            .spawn_bundle(PbrBundle {
                material,
                mesh: meshes.add(Mesh::from(shape::Icosphere {
                    radius: 0.2,
                    subdivisions: 5,
                })),
                transform,
                ..default()
            })
            .insert(RigidBody::Dynamic)
            .insert(ColliderShape::Sphere { radius: 0.2 })
            .insert_bundle(InteractiveBundle::<GroupDynamic>::default())
            .insert(Force::default())
            .insert(ColliderPhysicalProperties {
                restitution: 0.7,
                ..default()
            })
            .insert(GravityScale::default());
    }
}
