use graphics::{self, Layer, Transformed, Transformation};
use weapon;
use physics::Shape;
use specs;
use specs::Join;
use app;
use resources::*;
use components::*;
use colors;
use utils::math::*;
use std::f32::consts::PI;

pub fn run(world: &mut specs::World, frame: &mut graphics::Frame) {
    draw_notifications(world, frame);
    draw_physic(world, frame);
    draw_cursor(world, frame);
    draw_weapon(world, frame);
}

const NOTIFICATION_DL: f32 = 0.03;
const NOTIFICATION_SCALE: f32 = 0.02;
const NOTIFICATION_SMALL_MARGIN: f32 = 0.005;
const NOTIFICATION_BIG_MARGIN: f32 = 0.01;

fn draw_notifications(world: &mut specs::World, frame: &mut graphics::Frame) {
    let mut notifications = world.write_resource::<Notifications>();

    let (mut x, mut y) = frame.get_down_left_billboard_anchor();
    x += NOTIFICATION_DL;
    y += NOTIFICATION_DL;

    for &mut (ref notification, ref mut count) in notifications.0.iter_mut().rev() {
        *count -= 1;
        let (width, height) = frame.get_size(NOTIFICATION_SCALE, &*notification);
        {
            let w = width + NOTIFICATION_BIG_MARGIN * 2.0;
            let h = height + NOTIFICATION_BIG_MARGIN * 2.0;
            let x = x + w / 2.0;
            let y = y + h / 2.0;
            frame.draw_rectangle(x, y, w, h, Layer::UnderBillboard, colors::BLACK);
        }
        {
            let w = width + NOTIFICATION_SMALL_MARGIN * 2.0;
            let h = height + NOTIFICATION_SMALL_MARGIN * 2.0;
            let x = x + w / 2.0 + NOTIFICATION_BIG_MARGIN / 2.0;
            let y = y + h / 2.0 + NOTIFICATION_BIG_MARGIN / 2.0;
            frame.draw_rectangle(x, y, w, h, Layer::UnderBillboard, colors::WHITE);
        }
        frame.draw_text(x + NOTIFICATION_BIG_MARGIN,
                        y + NOTIFICATION_BIG_MARGIN / 2.,
                        NOTIFICATION_SCALE,
                        notification,
                        Layer::UnderBillboard,
                        colors::BLACK);
        y += height + NOTIFICATION_BIG_MARGIN * 2.0;
    }

    notifications.0.retain(|&(_, count)| count > 0)
}

const CURSOR_LENGTH: f32 = 0.044;
const CURSOR_GAP: f32 = 0.016;
const CURSOR_THICKNESS: f32 = 0.004;

fn draw_cursor(world: &mut specs::World, frame: &mut graphics::Frame) {
    let cursor = world.read_resource::<Cursor>();

    let width = (CURSOR_LENGTH - CURSOR_GAP) / 2.;
    let height = CURSOR_THICKNESS;
    let dx = -CURSOR_GAP / 2. - width / 2.;

    frame.draw_rectangle(cursor.x - dx, cursor.y, width, height, Layer::Billboard, colors::BLACK);
    frame.draw_rectangle(cursor.x + dx, cursor.y, width, height, Layer::Billboard, colors::BLACK);
    frame.draw_rectangle(cursor.x, cursor.y + dx, height, width, Layer::Billboard, colors::BLACK);
    frame.draw_rectangle(cursor.x, cursor.y - dx, height, width, Layer::Billboard, colors::BLACK);
}

fn draw_physic(world: &mut specs::World, frame: &mut graphics::Frame) {
    let draws = world.read::<DrawPhysic>();
    let states = world.read::<PhysicState>();
    let types = world.read::<PhysicType>();

    for (draw, state, typ) in (&draws, &states, &types).iter() {
        if let Some((thickness, border_color)) = draw.border {
            match typ.shape {
                Shape::Circle(radius) => {
                    frame.draw_circle(state.pos[0], state.pos[1], radius, Layer::Middle, border_color);
                    frame.draw_circle(state.pos[0], state.pos[1], radius - thickness, Layer::Middle, draw.color);
                },
                Shape::Rectangle(width, height) => {
                    frame.draw_rectangle(state.pos[0], state.pos[1], width, height, Layer::Middle, border_color);
                    frame.draw_rectangle(state.pos[0], state.pos[1], width - 2.*thickness, height - 2.*thickness, Layer::Middle, draw.color);
                },
            }
        } else {
            match typ.shape {
                Shape::Circle(radius) => frame.draw_circle(state.pos[0], state.pos[1], radius, Layer::Middle, draw.color),
                Shape::Rectangle(width, height) => frame.draw_rectangle(state.pos[0], state.pos[1], width, height, Layer::Middle, draw.color),
            }
        }
    }
}

fn draw_weapon(world: &mut specs::World, frame: &mut graphics::Frame) {
    let states = world.read::<PhysicState>();
    let weapons = world.read::<Weapon>();
    let aims = world.read::<Aim>();

    for (weapon, state, aim) in (&weapons, &states, &aims).iter() {
        match weapon.kind {
            weapon::Kind::Sniper => draw_sniper(state.pos, aim.0, weapon.state.clone(), frame),
            _ => unimplemented!(),
        }
    }
}

fn draw_sniper(pos: [f32; 2], aim: f32, state: weapon::State, frame: &mut graphics::Frame) {
    use weapon::State::*;
    let (delta_x, delta_angle) = match state {
        Setup(t) => (0., 0.),
        Ready => (0., 0.),
        Reload(t) => (0., 0.),
        Setdown(t) => (0., 0.),
    };

    let parent_trans = Transformation::identity()
        .translate(pos[0]+4., pos[1])
        .rotate(aim + delta_angle);

    // // maybe use freecad
    // let black = [
    //     parent_trans.translate(-0.6, 0.).scale(1.6, 0.2),
    //     parent_trans.translate(1.0, 0.).scale(2.0, 0.1),
    //     parent_trans.translate(0., 0.2).scale(0.2, 0.2),
    //     parent_trans.translate(0., 0.5).scale(0.6, 0.2),
    //     parent_trans.translate(-1.8, -0.5).rotate(PI/4.).scale(0.6, 0.2),
    //     // parent_trans.translate(-1.3, -0.2).scale(0.02, 0.2),
    //     // parent_trans.translate(-1.0, -0.3).scale(0.2, 0.02),
    // ];
    // for &trans in black.iter() {
    //     frame.draw_quad(trans, Layer::Middle, [0., 0., 0., 1.]);
    // }
}
