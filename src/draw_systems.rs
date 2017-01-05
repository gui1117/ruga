use graphics::{self, Layer, obj, Transformed, Transformation};
use weapons;
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
use itertools::Itertools;

use physics::draw_systems::*;
use weapons::draw_systems::*;
use notifications::draw_systems::*;

pub fn run(world: &mut specs::World, frame: &mut graphics::Frame) {
    draw_notifications(world, frame);
    draw_physic(world, frame);
    draw_cursor(world, frame);
    draw_weapon(world, frame);
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
