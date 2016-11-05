use hlua;

use std::sync::mpsc::Sender;

pub enum API {
    Quit,
    Notify(String),
    AddCharacter(f32,f32),
    AddWall(f32,f32,f32,f32),
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
    fn add_character(&mut self, x: f32, y: f32);
    fn add_wall(&mut self, x: f32, y: f32, w: f32, h: f32);

    /// internally used function
    fn call(&mut self, msg: API) {
        use self::API::*;
        match msg {
            Quit => self.quit(),
            Notify(string) => self.notify(string),
            AddCharacter(a, b) => self.add_character(a, b),
            AddWall(a, b, c, d) => self.add_wall(a, b, c, d),
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

    /// * **State**: pressed or released
    ///
    /// * **Virtualcode**:
    ///
    ///   TODO gamepad
    ///
    ///   none
    ///
    ///   mouseleft, mouseright, mousemiddle, mousexx (xx corresponding to byte code in hexadecimal)
    ///
    ///   key1, key2, key3, key4, key5, key6, key7, key8, key9, key0,
    ///
    ///   a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p, q, r, s, t, u, v, w, x, y, z,
    ///
    ///   f1, f2, f3, f4, f5, f6, f7, f8, f9, f10, f11, f12, f13, f14, f15,
    ///
    ///   numlock, numpad0, numpad1, numpad2, numpad3, numpad4, numpad5, numpad6, numpad7, numpad8, numpad9,
    ///
    ///   left, up, right, down,
    ///
    ///   escape, back, return, space,
    ///
    ///   snapshot, scroll, pause, insert, home, delete, end, pagedown, pageup, compose, abntc1, abntc2, add, apostrophe, apps, at, ax, backslash, calculator, capital, colon, comma, convert, decimal, divide, equals, grave, kana, kanji, lalt, lbracket, lcontrol, lmenu, lshift, lwin, mail, mediaselect, mediastop, minus, multiply, mute, mycomputer, navigateforward, navigatebackward, nexttrack, noconvert, numpadcomma, numpadenter, numpadequals, oem102, period, playpause, power, prevtrack, ralt, rbracket, rcontrol, rmenu, rshift, rwin, semicolon, slash, sleep, stop, subtract, sysrq, tab, underline, unlabeled, volumedown, volumeup, wake, webback, webfavorites, webforward, webhome, webrefresh, websearch, webstop, yen,
    // Scancode[8-9] is:
    // * 0 -> keyboard
    // * 1 -> mouse
    // * 2 -> mouse
    // * 3 -> gamepad ?
    fn input(state: String, scancode: u32, virtualcode: String);

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
