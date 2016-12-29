use graphics::{self, Layer, obj, Transformed, Transformation};
use weapon;
use physics::Shape;
use specs;
use specs::Join;
use app;
use resources::*;
use components::*;
use colors;
use utils::math::*;
use utils::math;
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

fn draw_arm(shoulder: [f32; 2], hand: [f32; 2], left: bool, pos: [f32; 2], angle: f32, dl: f32, da1: f32, da2: f32, frame: &mut graphics::Frame) {
    let len = 4.0;
    let width = 0.2;

    let shoulder = into_polar(shoulder);
    let p0x = pos[0] + shoulder[0]*(shoulder[1]+angle).cos();
    let p0y = pos[1] + shoulder[0]*(shoulder[1]+angle).sin();

    let hand = into_polar(hand);
    let p3x = pos[0] + dl*(angle+da1).cos() + hand[0]*(angle+da1+da2+hand[1]).cos();
    let p3y = pos[1] + dl*(angle+da1).sin() + hand[0]*(angle+da1+da2+hand[1]).sin();

    let norm_shoulder_hand = norm(sub([p3x, p3y], [p0x, p0y]));
    let angle_arm = if norm_shoulder_hand <= len {
        (norm_shoulder_hand/len).acos() * if left { 1. } else { -1. }
    } else {
        0.
    };

    let angle_shoulder_hand = math::angle(sub([p3x, p3y], [p0x, p0y]));
    let p1x = p0x + len/2.*(angle_shoulder_hand+angle_arm).cos();
    let p1y = p0y + len/2.*(angle_shoulder_hand+angle_arm).sin();

    let p2x = p1x;
    let p2y = p1y;

    frame.draw_bezier_curve((p0x, p0y), (p1x,p1y), (p2x, p2y), (p3x, p3y), width, Layer::Middle, colors::BLACK);
}

fn draw_sniper(pos: [f32; 2], aim: f32, state: weapon::State, frame: &mut graphics::Frame) {
    use weapon::State;

    let left_hand = [0.3, 0.3];
    let right_hand = [-0.3, -0.3];
    let left_shoulder = [0., 1.0];
    let right_shoulder = [0., -1.0];

    let len = 3.0;
    let recoil = 0.4;

    let (delta_len, delta_aim, delta_angle) = match state {
        State::Setup(t) => (len*t, 0., 0.),
        State::Ready => (len, 0., 0.),
        State::Reload(t) => (len - recoil*(0.5 - (t-0.5).abs()), 0., 0.),
        State::Setdown(t) => (len*(1. - t), 0., 0.),
    };

    let x = pos[0] + delta_len*(aim + delta_aim).cos();
    let y = pos[1] + delta_len*(aim + delta_aim).sin();

    draw_arm(left_shoulder, left_hand, true, pos, aim, delta_len, delta_aim, delta_angle, frame);
    draw_arm(right_shoulder, right_hand, false, pos, aim, delta_len, delta_aim, delta_angle, frame);

    frame.draw_obj(x, y, aim + delta_aim + delta_angle, obj::sniper, Layer::Middle, colors::BLACK);
}
