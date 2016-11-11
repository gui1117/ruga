use specs;
use graphics::Layer;
use components::*;
use physics::Shape;

// pub fn add_character(world: &mut specs::World, x: f32, y: f32) {
//     let entity = world.create_now()
//         .with(HitboxDraw::new([0.0, 0.0, 0.0, 1.0], Layer::Middle))
//         .with(PlayerControl)
//         .with(CollisionBehavior::Dodge)
//         .with(HitboxIdFlag)
//         .build();

//     let hitbox = circle_hitbox(x, y, 0.5);
//     let mut collider = world.write_resource::<Collider>();
//     collider.add_hitbox(entity.aci(), hitbox);
// }

// pub fn add_wall(world: &mut specs::World, x: f32, y: f32, width: f32, height: f32) {
//     let entity = world.create_now()
//         .with(HitboxDraw::new([0.0, 0.0, 0.0, 1.0], Layer::Middle))
//         .with(HitboxIdFlag)
//         .build();

//     let hitbox = rect_hitbox(x, y, width, height);
//     let mut collider = world.write_resource::<Collider>();
//     collider.add_hitbox(entity.aci(), hitbox);
// }

pub fn add_debug_rectangle(world: &mut specs::World, x: f32, y: f32, width: f32, height: f32) {
    let shape = Shape::Rectangle(width, height);
    let entity = world.create_now()
        .with(PhysicState::new([x,y]))
        .with(PhysicType::new_static(!0, !0, shape))
        .with(PhysicStatic)
        .with(DebugActive { active: false })
        .build();
}

pub fn add_debug_circle(world: &mut specs::World, x: f32, y: f32, r: f32) {
    let shape = Shape::Circle(r);
    let entity = world.create_now()
        .with(PhysicState::new([x,y]))
        .with(PhysicType::new_static(!0, !0, shape))
        .with(PhysicStatic)
        .with(DebugActive { active: false })
        .build();
}
