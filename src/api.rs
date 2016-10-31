pub enum State {
    Stop,
    Pause,
    Play,
}

pub trait Caller {
    fn set_player_gun_direction(&mut self, _angle: f32);
    fn player_gun_direction(&self) -> f32;

    fn set_player_run_direction(&mut self, _angle: f32);
    fn player_run_direction(&self) -> f32;

    fn set_player_run_force(&mut self, _force: f32);
    fn player_run_force(&self) -> f32;

    fn set_player_gun_shoot(&mut self, shoot: bool);
    fn is_player_gun_shoot(&self) -> bool;

    fn restart(&mut self);
    fn quit(&mut self);
    fn pause(&mut self);
    fn resume(&mut self);
    fn state(&self) -> State;

    fn set_zoom(&mut self, zoom: f32);
    fn zoom(&self);
}

pub trait Callee {
    // fn input(???) mouse + gamepad + keyboard
    // fn mouse_moved(???)
    // fn mouse_wheel(???)
    // fn character(???)
    fn poll_event();
}
