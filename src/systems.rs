use collider::{Collider, Hitbox};
use collider::geom::DirVec2;
use graphics;
use specs;
use specs::Join;
use app::{self, Notifications};
use utils::AsColliderId;
use components::*;
use colors;

const NOTIFICATION_DL: f32 = 0.03;
const NOTIFICATION_SCALE: f32 = 0.02;
const NOTIFICATION_SMALL_MARGIN: f32 = 0.005;
const NOTIFICATION_BIG_MARGIN: f32 = 0.01;

pub fn draw_hitbox(world: &mut specs::World, frame: &mut graphics::Frame) {
    use collider::geom::ShapeKind::*;

    let hitbox_id_flags = world.read::<HitboxIdFlag>();
    let hitbox_draws= world.read::<HitboxDraw>();
    let collider = world.read_resource::<Collider>();
    let entities = world.entities();

    for (draw, _, entity) in (&hitbox_draws, &hitbox_id_flags, &entities).iter() {
        let shape = collider.get_hitbox(entity.aci()).shape;
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

pub fn draw_notifications(world: &mut specs::World, frame: &mut graphics::Frame) {
    let mut notifications = world.write_resource::<Notifications>();

    let (mut x, mut y) = frame.get_down_left_billboard_anchor();
    x += NOTIFICATION_DL;
    y += NOTIFICATION_DL;

    for &mut (ref notification, ref mut count) in notifications.0.iter_mut().rev() {
        *count -= 1;
        let (width, height) = frame.get_size(NOTIFICATION_SCALE, &*notification);
        {
            let width = width + NOTIFICATION_BIG_MARGIN*2.0;
            let height = height + NOTIFICATION_BIG_MARGIN*2.0;
            let x = x + width/2.0;
            let y = y + height/2.0;
            frame.draw_rectangle(x, y, width, height, graphics::Layer::UnderBillboard, colors::BASE01);
        }
        {
            let width = width + NOTIFICATION_SMALL_MARGIN*2.0;
            let height = height + NOTIFICATION_SMALL_MARGIN*2.0;
            let x = x + width/2.0 + NOTIFICATION_BIG_MARGIN/2.0;
            let y = y + height/2.0 + NOTIFICATION_BIG_MARGIN/2.0;
            frame.draw_rectangle(x, y, width, height, graphics::Layer::Billboard, colors::BASE2);
        }
        frame.draw_text(x+NOTIFICATION_BIG_MARGIN, y+NOTIFICATION_BIG_MARGIN, NOTIFICATION_SCALE, notification, graphics::Layer::AboveBillboard, colors::BASE03);
        y += height+NOTIFICATION_BIG_MARGIN*2.0;
    }

    notifications.0.retain(|&(_, count)| count > 0)
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

        if behaviors.get(context.id0).is_none() && behaviors.get(context.id1).is_none() {
            return;
        }

        let hitbox_0 = collider.get_hitbox(context.id0.aci());
        let hitbox_1 = collider.get_hitbox(context.id0.aci());
        let normal_from_1_to_0 = hitbox_0.shape.normal_from(&hitbox_1.shape);

        if let Some(&behavior) = behaviors.get(context.id0) {
            let res = resolve_collision(behavior, &hitbox_0, &hitbox_1, &normal_from_1_to_0);
            collider.update_hitbox(context.id0.aci(), res);
        }
        if let Some(&behavior) = behaviors.get(context.id1) {
            let res = resolve_collision(behavior, &hitbox_1, &hitbox_0, &normal_from_1_to_0.flip());
            collider.update_hitbox(context.id1.aci(), res);
        }
    }
}
fn resolve_collision(behavior: CollisionBehavior, hitbox0: &Hitbox, _hitbox1: &Hitbox, normal: &DirVec2) -> Hitbox {
    use components::CollisionBehavior::*;

    let mut res = hitbox0.clone();
    if normal.len() > 0.0 {
        res.shape.pos += normal.dir()*normal.len();
    }

    match behavior {
        Dodge => {
            let scalar_product = hitbox0.vel.pos.x*normal.dir().x + hitbox0.vel.pos.y*normal.dir().y;
            if scalar_product < 0.0 {
                res.vel.pos += normal.dir()*(-scalar_product);
                res.vel.pos *= hitbox0.vel.pos.len()/res.vel.pos.len()
            }
        },
        Back => res.vel.pos = res.vel.pos.rotate_deg(180.0),
    }

    res
}
