use app::UpdateContext;
use physics::update_systems::*;
use weapons::update_systems::*;

pub fn add_systems(planner: &mut ::specs::Planner<UpdateContext>) {
    planner.add_system(PhysicSystem, "physic", 10);
    planner.add_system(WeaponSystem, "weapon", 9);
}
