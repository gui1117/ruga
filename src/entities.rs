use specs;
use collider::Collider;
use utils::{AsColliderId, circle_hitbox, rect_hitbox};
use graphics::Layer;
use components::*;

pub fn add_character(world: &mut specs::World, x: f32, y: f32) {
    let entity = world.create_now()
        .with(HitboxDraw::new([0.0, 0.0, 0.0, 1.0], Layer::Middle))
        .with(PlayerControl)
        .with(CollisionBehavior::Dodge)
        .with(HitboxIdFlag)
        .build();

    let hitbox = circle_hitbox(x, y, 0.5);
    let mut collider = world.write_resource::<Collider>();
    collider.add_hitbox(entity.aci(), hitbox);
}

pub fn add_wall(world: &mut specs::World, x: f32, y: f32, width: f32, height: f32) {
    let entity = world.create_now()
        .with(HitboxDraw::new([0.0, 0.0, 0.0, 1.0], Layer::Middle))
        .with(HitboxIdFlag)
        .build();

    let hitbox = rect_hitbox(x, y, width, height);
    let mut collider = world.write_resource::<Collider>();
    collider.add_hitbox(entity.aci(), hitbox);
}
