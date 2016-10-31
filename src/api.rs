use hlua;

use std::sync::mpsc::Sender;

pub enum API {
    Quit,
    Notify(String),
}

pub trait Caller {
    // fn set_player_gun_direction(&mut self, angle: f32);
    // fn set_player_run_direction(&mut self, angle: f32);
    // fn set_player_run_force(&mut self, force: f32);
    // fn set_player_gun_shoot(&mut self, shoot: bool);
    // fn restart(&mut self);
    fn quit(&mut self);
    // fn pause(&mut self);
    // fn resume(&mut self);
    // fn set_zoom(&mut self, zoom: f32);

    fn notify(&mut self, notification: String);
    // /// Print to terminal, use notify instead to notify to screen
    // fn print(&mut self, msg: String);

    /// internally used function
    fn call(&mut self, msg: API) {
        use self::API::*;
        match msg {
            Quit => self.quit(),
            Notify(string) => self.notify(string),
        }
    }
}

pub fn set_lua_caller(lua: &mut hlua::Lua, sender: Sender<API>) {
    let sender_clone = sender.clone();
    lua.set("notify", hlua::function1(move |string| {
        sender_clone.send(API::Notify(string)).unwrap();
    }));
    lua.set("quit", hlua::function0(move || {
        sender.send(API::Quit).unwrap();
    }));
}

pub trait Callee {
    /// The cursor has moved on the window.
    /// The parameter are the (x,y) coords relative to the center of the window.
    ///
    /// * x is always between -1.0 and 1.0
    /// * y may exceed 1.0 if screen_height is higher than screen_width and never reach 1.0 otherwise.
    ///
    /// the coordinate system is orthonormal
    fn mouse_moved(x: f32, y: f32);

    /// * **Pressed**: released otherwise
    ///
    /// * **Virtualcode**:
    ///
    ///   TODO gamepad
    ///
    ///   MouseLeft, MouseRight, MouseMiddle, MouseXX (XX corresponding to byte code in hexadecimal)
    ///
    ///   Key1, Key2, Key3, Key4, Key5, Key6, Key7, Key8, Key9, Key0,
    ///
    ///   A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W, X, Y, Z,
    ///
    ///   F1, F2, F3, F4, F5, F6, F7, F8, F9, F10, F11, F12, F13, F14, F15,
    ///
    ///   Numlock, Numpad0, Numpad1, Numpad2, Numpad3, Numpad4, Numpad5, Numpad6, Numpad7, Numpad8, Numpad9,
    ///
    ///   Left, Up, Right, Down,
    ///
    ///   Escape, Back, Return, Space,
    ///
    ///   Snapshot, Scroll, Pause, Insert, Home, Delete, End, PageDown, PageUp, Compose, AbntC1, AbntC2, Add, Apostrophe, Apps, At, Ax, Backslash, Calculator, Capital, Colon, Comma, Convert, Decimal, Divide, Equals, Grave, Kana, Kanji, LAlt, LBracket, LControl, LMenu, LShift, LWin, Mail, MediaSelect, MediaStop, Minus, Multiply, Mute, MyComputer, NavigateForward, NavigateBackward, NextTrack, NoConvert, NumpadComma, NumpadEnter, NumpadEquals, OEM102, Period, PlayPause, Power, PrevTrack, RAlt, RBracket, RControl, RMenu, RShift, RWin, Semicolon, Slash, Sleep, Stop, Subtract, Sysrq, Tab, Underline, Unlabeled, VolumeDown, VolumeUp, Wake, WebBack, WebFavorites, WebForward, WebHome, WebRefresh, WebSearch, WebStop, Yen,
    fn input(pressed: bool, scancode: u32, virtualcode: String);

    /// Amount in lines or rows or pixels to scroll in the horizontal and vertical directions.
    ///
    /// Positive values indicate movement forward (away from the user) or rightwards.
    fn mouse_wheel(horizontal: f32, vertical: f32);

    /// Function called at each update
    fn update(dt: f32);
}

pub fn set_lua_callee(lua: &mut hlua::Lua) {
    lua.execute::<()>("
    function mouse_moved(x, y)
    end
    function input(pressed, scancode, virtualcode)
    end
    function mouse_wheel(horizontal, vertical)
    end
    function update(dt)
    end
    ").unwrap();
}
