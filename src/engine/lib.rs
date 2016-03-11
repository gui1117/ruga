mod utils;
mod world;
mod spatial_hashing;

pub use world::World;

pub mod body;

use body::Body;

pub trait Entity<Management: EntityManagement> {
    fn mut_body(&self) -> &mut Body;
    fn body(&self) -> &Body;
    fn on_collision(&self, _other: &mut Body) {}
    fn update(&self, _dt: f64, _world: &World<Management>, _effect_manager: &mut Management::EffectManager) {}
    fn render(&self, _frame_manager: &mut Management::FrameManager) {}
    //fn render_ai(&self, _frame_manager: &mut Management::IAFrameManager) {}
}

pub trait EntityManagement {
    type FrameManager;
    type EffectManager;
    //type IAFrameManager;
}
