mod utils;
mod world;
mod spatial_hashing;

pub mod body;
pub use self::world::World;

pub use frame_manager::FrameManager;
pub use effect_manager::EffectManager;

use self::body::Body;

use std::cell::{Ref, RefMut};
use std::collections::HashSet;

pub trait Entity {
    fn body(&self) -> &Body;
    fn mut_body(&mut self) -> &mut Body;
    fn on_collision(&mut self, _other: &mut Entity) {}
    fn render(&self, _render_args: &mut FrameManager) {}
    fn modify_wall_map(&self, &mut HashSet<(i32,i32)>) {}
    //fn render_ai(&self, _frame_manager: &mut Management::IAFrameManager) {}
}

pub trait EntityCell {
    fn update(&self, _dt: f64, _world: &World, _update_args: &mut EffectManager) {}
    fn borrow(&self) -> Ref<Entity>;
    fn borrow_mut(&self) -> RefMut<Entity>;
}
