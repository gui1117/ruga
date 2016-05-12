use specs;

#[derive(Debug,Clone,Default)]
pub struct PlayerControl;
impl specs::Component for PlayerControl {
    type Storage = specs::NullStorage<Self>;
}
