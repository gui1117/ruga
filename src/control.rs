use specs;

pub struct PlayerControl;
impl specs::Component for PlayerControl {
    type Storage = specs::VecStorage<Self>;
}
