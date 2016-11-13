macro_rules! api_callee {
    ($( $(#[doc = $doc:expr])* fn $func:ident ($($arg:ident: $typ:ty),*);)*) => {
        pub trait Callee {
            $( $(#[doc = $doc])* fn $func($( $arg: $typ),*);)*
        }

        pub fn set_lua_callee(lua: &mut ::hlua::Lua) {
            $(
                let args = stringify!($($arg),*);
                let func = stringify!($func);
                let function = format!("function {}({}) end", func, args);
                // let help = format!("{}({}): {}", func, args, $doc);
                // let function_help = format!("function help_{}() print(\"{}\") end", func, help);
                lua.execute::<()>(&*function).unwrap();
            )*
        }

        pub fn callee_function_names() -> Vec<String> {
            vec!($(
                String::from(stringify!($func))
            ),*)
        }
    }
}

macro_rules! api_caller {
    ($( $(#[doc = $doc:expr])* fn $func:ident ($($arg:ident: $typ:ty),*);)* + entities) => {
        #[allow(non_camel_case_types)]
        #[doc(hidden)]
        pub enum CallerMsg {
            EntityBuilder(::entities::EntityBuilderMsg),
            $($func((), $($typ),*)),*
        }

        pub trait Caller: ::entities::EntityBuilder{
            $( $(#[doc = $doc])* fn $func(&mut self, $( $arg: $typ),*);)*
            /// Internally used function
            fn call(&mut self, msg: CallerMsg) {
                match msg {
                    CallerMsg::EntityBuilder(msg) => self.build_entity(msg),
                    $( CallerMsg::$func(_, $($arg),*) => self.$func($($arg),*),)*
                }
            }
        }

        pub fn set_lua_caller(lua: &mut ::hlua::Lua, sender: ::std::sync::mpsc::Sender<CallerMsg>) {
            $(
                let sender_clone = sender.clone();
                let func = stringify!($func);
                lua.set(func, infer_type!($($arg)*)(move |$($arg),*| {
                    sender_clone.send(CallerMsg::$func((), $($arg),*)).unwrap();
                }));
            )*
            ::entities::set_lua_builder(lua, sender);
        }

        pub fn caller_function_names() -> Vec<String> {
            let mut vec = vec!($(
                String::from(stringify!($func))
            ),*);
            vec.append(&mut ::entities::builder_function_names());
            vec
        }
    }
}

// fn set_player_gun_direction(&mut self, angle: f32);
// fn set_player_gun_shoot(&mut self, shoot: bool);
// fn restart(&mut self);
// fn pause(&mut self);
// fn resume(&mut self);
// fn set_zoom(&mut self, zoom: f32);
// fn set_player_run_vector(x: f32, y: f32);
api_caller! {
    /// Quit the game
    fn quit();
    /// Show notification on the screen
    fn notify(notification: String);
    /// Print to terminal, use notify instead to notify to screen
    fn print(msg: String);
    /// Fill physic world with static and dynamic physic elements
    fn fill_physic_world();
    /// Set mouse sensibility
    fn set_sensibility(s: f32);
    + entities
}

api_callee! {
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
///   numlock, numpad0, numpad1, numpad2, numpad3, numpad4,
///   numpad5, numpad6, numpad7, numpad8, numpad9,
///
///   left, up, right, down,
///
///   escape, back, return, space,
///
///   snapshot, scroll, pause, insert, home, delete, end, pagedown, pageup, compose,
///   abntc1, abntc2, add, apostrophe, apps, at, ax, backslash, calculator, capital,
///   colon, comma, convert, decimal, divide, equals, grave, kana, kanji, lalt, lbracket,
///   lcontrol, lmenu, lshift, lwin, mail, mediaselect, mediastop, minus, multiply, mute,
///   mycomputer, navigateforward, navigatebackward, nexttrack, noconvert, numpadcomma,
///   numpadenter, numpadequals, oem102, period, playpause, power, prevtrack, ralt,
///   rbracket, rcontrol, rmenu, rshift, rwin, semicolon, slash, sleep, stop, subtract,
///   sysrq, tab, underline, unlabeled, volumedown, volumeup, wake, webback, webfavorites,
///   webforward, webhome, webrefresh, websearch, webstop, yen,
///
/// Scancode[8-9] is:
/// * 0 -> keyboard
/// * 1 -> mouse
/// * 2 -> mouse
/// * 3 -> gamepad ?
    fn input(state: String, scancode: u32, virtualcode: String);

/// Amount in lines or rows or pixels to scroll in the horizontal and vertical directions.
///
/// Positive values indicate movement forward (away from the user) or rightwards.
    fn mouse_wheel(horizontal: f32, vertical: f32);

/// Function called at each update
    fn update(dt: f32);
}
