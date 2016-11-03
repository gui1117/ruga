use collider::{Collider, Hitbox};
use collider::geom::DirVec2;
use graphics;
use specs;
use specs::Join;
use app;
use components::*;

fn draw_hitbox(world: &mut specs::World, frame: &mut graphics::Frame) {
    use collider::geom::ShapeKind::*;

    let hitbox_ids = world.read::<HitboxIdT>();
    let hitbox_draws= world.read::<HitboxDraw>();
    let collider = world.read_resource::<Collider>();

    for (draw, id) in (&hitbox_draws, &hitbox_ids).iter() {
        let shape = collider.get_hitbox(id.0).shape;
        match shape.shape.kind() {
            Circle => {
                let (x, y) = (shape.pos.x as f32, shape.pos.y as f32);
                let r = shape.shape.dims().x as f32 / 2.0;
                frame.draw_circle(x, y, r, draw.layer, draw.color);
            },
            Rect => {
                let (x, y) = (shape.pos.x as f32, shape.pos.y as f32);
                let dims = shape.shape.dims();
                let (w, h) = (dims.x as f32, dims.y as f32);
                frame.draw_rectangle(x, y, w, h, draw.layer, draw.color);
            },
        }
    }
}

pub struct ResolveCollision;
impl specs::System<app::CollideContext> for ResolveCollision {
    fn run(&mut self, arg: specs::RunArg, context: app::CollideContext) {
        let (behaviors, mut collider) = arg.fetch(|world| {
            (
                world.read::<CollisionBehavior>(),
                world.write_resource::<Collider>(),
            )
        });

        if behaviors.get(context.id0.0).is_none() && behaviors.get(context.id1.0).is_none() {
            return;
        }

        let mut hitbox_0 = collider.get_hitbox(context.id0.1);
        let mut hitbox_1 = collider.get_hitbox(context.id0.1);
        let mut normal_from_1_to_0 = hitbox_0.shape.normal_from(&hitbox_1.shape);

        if let Some(&behavior) = behaviors.get(context.id0.0) {
            resolve_collision(behavior, &mut hitbox_0, normal_from_1_to_0);
            collider.update_hitbox(context.id0.1, hitbox_0);
        }
        if let Some(&behavior) = behaviors.get(context.id1.0) {
            resolve_collision(behavior, &mut hitbox_1, normal_from_1_to_0.flip());
            collider.update_hitbox(context.id1.1, hitbox_1);
        }
    }
}
fn resolve_collision(behavior: CollisionBehavior, hitbox: &mut Hitbox, normal: DirVec2) {
    use components::CollisionBehavior::*;
    match behavior {
        Dodge => {
            // TODO
            unimplemented!();
        },
        Bounce => {
            // TODO
            unimplemented!();
        },
        Back => hitbox.vel.pos = hitbox.vel.pos.rotate_deg(180.0),
    }
}
