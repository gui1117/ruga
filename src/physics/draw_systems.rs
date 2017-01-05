use specs::{self, Join};
use graphics::{self, Layer, obj, Transformed, Transformation};

use super::*;
use super::components::*;
use super::resources::*;

pub fn draw_physic(world: &mut specs::World, frame: &mut graphics::Frame) {
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
