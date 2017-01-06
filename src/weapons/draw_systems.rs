use graphics::{self, Layer, obj};
use specs;
use specs::Join;
use components::*;
use utils::math::*;
use utils::math;
use colors;
use itertools::Itertools;
use super::*;

pub fn draw_weapon(world: &mut specs::World, frame: &mut graphics::Frame) {
    let states = world.read::<PhysicState>();
    let weapons = world.read::<Weapon>();
    let aims = world.read::<Aim>();

    for (weapons, state, aim) in (&weapons, &states, &aims).iter() {
        match weapons.kind {
            Kind::Sniper => draw_sniper(state.pos, aim.0, weapons.state.clone(), frame),
            _ => unimplemented!(),
        }
    }
}

fn draw_arm(shoulder: [f32; 2], hand: [f32; 2], left: bool, pos: [f32; 2], angle: f32, dl: f32, da1: f32, da2: f32, frame: &mut graphics::Frame) {
    let len = 2.0;
    let width = 0.15;

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

struct Point {
    time: f32,
    data: (f32, f32, f32),
}

fn interpolation(time: f32, points: &[Point]) -> (f32, f32, f32) {
    if points.is_empty() {
        panic!("interpolation between 0 points not allowed");
    }

    let first = points.first().unwrap();
    if time <= first.time {
        return first.data
    }

    let last = points.last().unwrap();
    if time >= last.time {
        return last.data
    }

    for (a, b) in points.iter().tuple_windows() {
        debug_assert!(a.time < b.time);
        if a.time <= time && time <= b.time {
            let a_coef = 1. - (time - a.time) / (b.time - a.time);
            let b_coef = 1. - a_coef;

            return (
                a.data.0*a_coef + b.data.0*b_coef,
                a.data.1*a_coef + b.data.1*b_coef,
                a.data.2*a_coef + b.data.2*b_coef,
            )
        }
    }
    unreachable!();
}

fn draw_sniper(pos: [f32; 2], aim: f32, state: State, frame: &mut graphics::Frame) {
    let left_hand = [-0.5, 0.1];
    let right_hand = [-1., -0.2];
    let left_shoulder = [0., 0.5];
    let right_shoulder = [0., -0.5];

    let len = 2.;
    let recoil = 0.4;
    let time_recoil = 0.4;
    let time_reload = 0.6;

    let (delta_len, delta_aim, delta_angle) = match state {
        State::Setup(t) => (len*t, 0., 0.),
        State::Ready => (len, 0., 0.),
        State::Reload(t) => {
            interpolation(t, &[
                          Point { time:             0., data: (       len, 0., 0.) },
                          Point { time: time_recoil/2., data: (len-recoil, 0., 0.) },
                          Point { time:    time_recoil, data: (       len, 0., 0.) },
            ])
        },
        State::Setdown(t) => (len*(1. - t), 0., 0.),
    };

    let x = pos[0] + delta_len*(aim + delta_aim).cos();
    let y = pos[1] + delta_len*(aim + delta_aim).sin();

    draw_arm(left_shoulder, left_hand, true, pos, aim, delta_len, delta_aim, delta_angle, frame);
    draw_arm(right_shoulder, right_hand, false, pos, aim, delta_len, delta_aim, delta_angle, frame);

    frame.draw_obj(x, y, aim + delta_aim + delta_angle, obj::sniper, Layer::Middle, colors::BLACK);
}
