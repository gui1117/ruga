use graphics;
use specs;
use utils::{self, Direction, HorizontalVerticalAxis};
use event_loop;
use config;
use glium::{self, glutin};
use specs::Join;
use levels;
use systems::*;
use components::*;
use std::sync::mpsc;
use baal;
use std::rc::Rc;
use std::sync::Arc;
use entities;
use std::fmt;
use gilrs;

static HELP: &'static str = "
use up,down,left,right or w,s,a,d to move

use escape to go to or escape from menu

";

static CREDIT: &'static str = "
made by thiolliere [thiolliere.org]


musics from ¿Therence?

sounds effects from Xonotic game [xonotic.org]

colors from solarized [ethanschoonover.com/solarized]

powered by rust language [rust-lang.org]

";

// static DONATE: &'static str = "
// if you want to
// please consider donate
// maybe $2 or $5 to:

// TODO paypal

// ";

pub struct Graphic {
    color: graphics::Color,
    layer: graphics::Layer,
}
impl Graphic {
    pub fn new(color: graphics::Color, layer: graphics::Layer) -> Self {
        Graphic {
            color: color,
            layer: layer,
        }
    }
}
impl specs::Component for Graphic {
    type Storage = specs::VecStorage<Self>;
}

pub enum Effect {
    Line {
        origin: [f32;2],
        length: f32,
        angle: f32,
        persistance: f32,
        thickness: f32,
        layer: graphics::Layer,
        color: graphics::Color,
    },
}
impl Effect {
    fn next(self,dt: f32) -> Option<Effect> {
        match self {
            Effect::Line { origin: o, length: le, angle: a, persistance: mut p, thickness: t, layer: la, color: c, } => {
                p -= dt;
                if p > 0. {
                    Some(Effect::Line { origin: o, length: le, angle: a, persistance: p, thickness: t, layer: la, color: c, })
                } else {
                    None
                }
            },
        }
    }
    fn draw(&self, frame: &mut graphics::Frame) {
        match self {
            &Effect::Line {
                origin: o,
                length: le,
                angle: a,
                persistance: _,
                thickness: t,
                layer: la,
                color: co,
            } => {
                frame.draw_line(o[0],o[1],a,le,t,la,co);
            },
        }
    }
}

pub enum Control {
    GotoLevel(levels::Level),
    ResetLevel,
    ResetGame,
    ResetCastle,
    CreateBall([f32;2],Arc<()>),
}

#[derive(Clone)]
pub struct UpdateContext {
    pub effect_tx: mpsc::Sender<Effect>,
    pub control_tx: mpsc::Sender<Control>,
    pub dt: f32,
}

#[derive(PartialEq,Clone)]
enum State {
    Game,
    Menu(usize),
    Text(usize,String),
}

struct MenuEntry {
    name: Box<Fn(&App)->String>,
    left: Rc<Box<Fn(&mut App)>>,
    right: Rc<Box<Fn(&mut App)>>,
}

impl MenuEntry {
    fn new_left_right(name: Box<Fn(&App)->String>, left: Rc<Box<Fn(&mut App)>>, right: Rc<Box<Fn(&mut App)>>) -> Self {
        MenuEntry {
            name: name,
            left: left,
            right: right,
        }
    }
    fn new_button(name: Box<Fn(&App)->String>, button: Rc<Box<Fn(&mut App)>>) -> Self {
        MenuEntry {
            name: name,
            left: button.clone(),
            right: button,
        }
    }
}

enum PlayerControlState {
    Keyboard(Vec<Direction>),
    Joystick(f32,f32),
}

impl PlayerControlState {
    fn push_keyboard_dir(&mut self, dir: Direction) {
        if let &mut PlayerControlState::Keyboard(ref mut directions) = self {
            if !directions.contains(&dir) {
                directions.push(dir);
            }
        } else {
            *self = PlayerControlState::Keyboard(vec!(dir));
        }
    }
    fn retain_keyboard_dir(&mut self, dir: Direction) {
        if let &mut PlayerControlState::Keyboard(ref mut directions) = self {
            directions.retain(|&d| d != dir);
        }
    }
    fn set_axis_x_state(&mut self, x: f32) {
        if let PlayerControlState::Keyboard(_) = *self {
            *self = PlayerControlState::Joystick(0.,0.);
        }
        if let &mut PlayerControlState::Joystick(ref mut x_ref,_) = self {
            *x_ref = x;
        }
    }
    fn set_axis_y_state(&mut self, y: f32) {
        if let PlayerControlState::Keyboard(_) = *self {
            *self = PlayerControlState::Joystick(0.,0.);
        }
        if let &mut PlayerControlState::Joystick(_,ref mut y_ref) = self {
            *y_ref = y;
        }
    }
}

enum JoystickMenuState {
    Pressed(Direction,f32),
    Released,
}

pub struct App {
    difficulty: f32,
    menu: Vec<MenuEntry>,
    menu_interline: Vec<usize>,
    castles: Vec<levels::Castle>,
    state: State,
    current_level: levels::Level,
    camera: graphics::Camera,
    graphics: graphics::Graphics,
    planner: specs::Planner<UpdateContext>,
    player_control_state: PlayerControlState,
    joystick_menu_state: JoystickMenuState,
    control_rx: mpsc::Receiver<Control>,
    control_tx: mpsc::Sender<Control>,
    effect_rx: mpsc::Receiver<Effect>,
    effect_storage: Vec<Effect>,
    effect_tx: mpsc::Sender<Effect>,
    focus: bool,
    pub quit: bool,
}

pub enum AppError {
    InitGraphics(graphics::GraphicsCreationError),
    LevelCreation(String),
}

impl fmt::Display for AppError {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        use self::AppError::*;
        match *self {
            InitGraphics(ref e) => write!(fmt,"graphics init failed: {}",e),
            LevelCreation(ref s) =>write!(fmt,"level creation error: {}",s),
        }
    }
}

impl App {
    pub fn new<F: glium::backend::Facade>(facade: &F, castles: Vec<levels::Castle>) -> Result<App,AppError> {
        // init graphics
        let graphics = try!(graphics::Graphics::new(facade, graphics::GraphicsSetting {
            colors: graphics::ColorsValue {
                base03: config.graphics.base03,
                base02: config.graphics.base02,
                base01: config.graphics.base01,
                base00: config.graphics.base00,
                base0: config.graphics.base0,
                base1: config.graphics.base1,
                base2: config.graphics.base2,
                base3: config.graphics.base3,
                yellow: config.graphics.yellow,
                orange: config.graphics.orange,
                red: config.graphics.red,
                magenta: config.graphics.magenta,
                violet: config.graphics.violet,
                blue: config.graphics.blue,
                cyan: config.graphics.cyan,
                green: config.graphics.green,
            },
            mode: match &*config.graphics.mode {
                "light" => graphics::Mode::Light,
                "dark" => graphics::Mode::Dark,
                _ => unreachable!(),
            },
            luminosity: config.graphics.luminosity,
            circle_precision: config.graphics.circle_precision,
            font: config.graphics.font_file.val.clone(),
            billboard_font_scale: config.graphics.billboard_font_scale,
        }).map_err(|e| AppError::InitGraphics(e)));

        // init camera
        let camera = graphics::Camera::new(0.0, 0.0, config.camera.zoom);

        // init world
        let mut world = specs::World::new();
        world.register::<PlayerControl>();
        world.register::<TowardPlayerControl>();
        world.register::<MonsterControl>();

        world.register::<PhysicState>();
        world.register::<PhysicForce>();
        world.register::<PhysicType>();
        world.register::<PhysicDynamic>();
        world.register::<PhysicStatic>();
        world.register::<PhysicTrigger>();
        world.register::<GridSquare>();

        world.register::<Graphic>();

        world.register::<Life>();
        world.register::<Killer>();
        world.register::<Ball>();
        world.register::<Column>();

        world.register::<Portal>();

        world.register::<FixedCameraText>();
        world.register::<Text>();
        world.register::<FixedCamera>();

        world.register::<DynPersistentSnd>();

        // check levels
        let check_level = match &*config.levels.check_level {
            "always" => true,
            "debug" => {
                let mut check_level = false;
                debug_assert!({
                    check_level = true;
                    true
                });
                check_level
            },
            "never" => false,
            _ => unreachable!(),
        };

        if check_level {
            for (c,castle) in castles.iter().enumerate() {
                for (d,dungeon) in castle.dungeons.iter().enumerate() {
                    for (r,_) in dungeon.rooms.iter().enumerate() {
                        let level = levels::Level::Room {
                            castle: c,
                            dungeon: d,
                            room: r,
                        };
                        try!(levels::load_level(&level, &castles, &mut world)
                                .map_err(|e| AppError::LevelCreation(format!("load level {}.{}.{} failed: {}",castle.name,dungeon.name,r,e))));
                    }
                }
            }
        }

        // load level
        let level = levels::Level::Entry;
        try!(levels::load_level(&level, &castles, &mut world)
             .map_err(|e| AppError::LevelCreation(format!("load entry level failed: {}",e))));

        // init planner
        let mut planner = specs::Planner::new(world,config.general.number_of_thread);
        planner.add_system(PhysicSystem, "physic", 10);
        planner.add_system(PlayerSystem::default(), "player", 5);
        planner.add_system(MonsterSystem, "monster", 5);
        planner.add_system(TowardPlayerSystem, "toward_player", 5);
        planner.add_system(KillerSystem, "killer", 5);
        planner.add_system(BallSystem, "ball", 5);
        planner.add_system(PortalSystem, "portal", 5);
        planner.add_system(ColumnSystem, "column", 5);
        planner.add_system(LifeSystem, "life", 1);
        planner.add_system(PersistentSndSystem::default(), "life", 2);

        let (effect_tx, effect_rx) = mpsc::channel();
        let (control_tx, control_rx) = mpsc::channel();

        // create menu
        let menu_interline = vec!(0,1,4,7,9,11);
        let menu = vec!(
            MenuEntry::new_button(
                Box::new(|_| "continue".into()),
                Rc::new(Box::new(|app| app.goto_state_game()))),
            MenuEntry::new_left_right(
                Box::new(|app| format!("difficulty: {}",((app.difficulty*10.).round() as usize))),
                Rc::new(Box::new(|app| {
                    app.difficulty = (app.difficulty - 0.1).max(0.1);
                    app.save();
                })),
                Rc::new(Box::new(|app| {
                    app.difficulty = (app.difficulty + 0.1).min(1.0);
                    app.save();
                }))),
            MenuEntry::new_button(
                Box::new(|_| "restart room".into()),
                Rc::new(Box::new(|app| {
                    app.control_tx.send(Control::ResetLevel).unwrap();
                }))),
            MenuEntry::new_button(
                Box::new(|_| "restart castle".into()),
                Rc::new(Box::new(|app| {
                    app.control_tx.send(Control::ResetCastle).unwrap();
                }))),
            MenuEntry::new_button(
                Box::new(|_| "restart game".into()),
                Rc::new(Box::new(|app| {
                    app.control_tx.send(Control::ResetGame).unwrap();
                }))),
            MenuEntry::new_left_right(
                Box::new(|_| format!("global volume: {}",(baal::global_volume()*10.).round() as usize)),
                Rc::new(Box::new(|app| {
                    baal::set_global_volume((baal::global_volume()-0.1).max(0.0));
                    app.save();
                })),
                Rc::new(Box::new(|app| {
                    baal::set_global_volume((baal::global_volume()+0.1).min(1.0));
                    app.save();
                }))),
            MenuEntry::new_left_right(
                Box::new(|_| format!("music volume: {}",(baal::music::volume()*10.).round() as usize)),
                Rc::new(Box::new(|app| {
                    baal::music::set_volume((baal::music::volume()-0.1).max(0.0));
                    app.save()
                })),
                Rc::new(Box::new(|app| {
                    baal::music::set_volume((baal::music::volume()+0.1).min(1.0));
                    app.save();
                }))),
            MenuEntry::new_left_right(
                Box::new(|_| format!("effects volume: {}",(baal::effect::volume()*10.).round() as usize)),
                Rc::new(Box::new(|app| {
                    baal::effect::set_volume((baal::effect::volume()-0.1).max(0.0));
                    app.save()
                })),
                Rc::new(Box::new(|app| {
                    baal::effect::set_volume((baal::effect::volume()+0.1).min(1.0));
                    app.save();
                }))),
            MenuEntry::new_button(
                Box::new(|app| format!("theme: {}", match app.graphics.mode() {
                    graphics::Mode::Dark => "dark",
                    graphics::Mode::Light => "light",
                })),
                Rc::new(Box::new(|app| {
                    app.graphics.toggle_mode();
                    app.save();
                }))),
            MenuEntry::new_left_right(
                Box::new(|app| format!("luminosity: {}", (app.graphics.luminosity()*10.).round() as usize)),
                Rc::new(Box::new(|app| {
                    let l = app.graphics.luminosity();
                    app.graphics.set_luminosity((l-0.1).max(0.1));
                    app.save();
                })),
                Rc::new(Box::new(|app| {
                    let l = app.graphics.luminosity();
                    app.graphics.set_luminosity((l+0.1).min(1.0));
                    app.save();
                }))),
            MenuEntry::new_button(
                Box::new(|_| "help".into()),
                Rc::new(Box::new(|app| app.goto_state_text(HELP.into())))),
            // MenuEntry::new_button(
            //     Box::new(|_| "donate".into()),
            //     Rc::new(Box::new(|app| app.goto_state_text(DONATE.into())))),
            MenuEntry::new_button(
                Box::new(|_| "credit".into()),
                Rc::new(Box::new(|app| app.goto_state_text(CREDIT.into())))),
            MenuEntry::new_button(
                Box::new(|_| "quit".into()),
                Rc::new(Box::new(|app| app.quit = true))),
            );

        Ok(App {
            difficulty: config.general.difficulty,
            menu_interline: menu_interline,
            menu: menu,
            state: State::Game,
            castles: castles,
            current_level: level,
            joystick_menu_state: JoystickMenuState::Released,
            effect_storage: Vec::new(),
            camera: camera,
            graphics: graphics,
            planner: planner,
            player_control_state: PlayerControlState::Keyboard(vec!()),
            effect_rx: effect_rx,
            effect_tx: effect_tx,
            control_rx: control_rx,
            control_tx: control_tx,
            focus: true,
            quit: false,
        })
    }
    pub fn save(&self) {
        use conf;
        use std;
        use std::io::Write;

        let result =  conf::save(conf::Save {
            difficulty: self.difficulty,
            global_volume: baal::music::volume(),
            effect_volume: baal::effect::volume(),
            music_volume: baal::effect::volume(),
            luminosity: self.graphics.luminosity(),
            mode: match self.graphics.mode() {
                graphics::Mode::Light => "light".into(),
                graphics::Mode::Dark => "dark".into(),
            },
        });
        if let Some(err) = result.err() {
            writeln!(&mut std::io::stderr(), "ERROR failed to save save_file: {}", err).unwrap();
        }
    }
    fn update_player_control(&mut self) {
        use std::f32::consts::PI;

        let world = self.planner.mut_world();

        match self.player_control_state {
            PlayerControlState::Joystick(x,y) => {
                let angle = y.atan2(x);
                let intensity = (x.powi(2)+y.powi(2)).sqrt();
                let characters = world.read::<PlayerControl>();
                let mut forces = world.write::<PhysicForce>();
                for (_, force) in (&characters, &mut forces).iter() {
                    force.direction = angle;
                    force.intensity = intensity;
                }
            },
            PlayerControlState::Keyboard(ref directions) => {
                if let Some(dir) = directions.last() {

                    let mut last_perpendicular: Option<&Direction> = None;
                    for d in directions {
                        if d.perpendicular(dir) {
                            last_perpendicular = Some(d);
                        }
                    }

                    let angle = match dir {
                        &Direction::Up => {
                            match last_perpendicular {
                                Some(&Direction::Left) => 3.*PI/4.,
                                Some(&Direction::Right) => PI/4.,
                                _ => PI/2.,
                            }
                        },
                        &Direction::Down => {
                            match last_perpendicular {
                                Some(&Direction::Left) => -3.*PI/4.,
                                Some(&Direction::Right) => -PI/4.,
                                _ => -PI/2.,
                            }
                        },
                        &Direction::Right => {
                            match last_perpendicular {
                                Some(&Direction::Down) => -PI/4.,
                                Some(&Direction::Up) => PI/4.,
                                _ => 0.,
                            }
                        },
                        &Direction::Left => {
                            match last_perpendicular {
                                Some(&Direction::Down) => -3.*PI/4.,
                                Some(&Direction::Up) => 3.*PI/4.,
                                _ => PI,
                            }
                        },
                    };

                    let characters = world.read::<PlayerControl>();
                    let mut forces = world.write::<PhysicForce>();
                    for (_, force) in (&characters, &mut forces).iter() {
                        force.direction = angle;
                        force.intensity = 1.;
                    }
                } else {
                    let characters = world.read::<PlayerControl>();
                    let mut forces = world.write::<PhysicForce>();
                    for (_, force) in (&characters, &mut forces).iter() {
                        force.intensity = 0.;
                    }
                }
            },
        }
    }
    pub fn goto_state_menu(&mut self) {
        baal::effect::pause();

        match self.state {
            State::Game => self.state = State::Menu(0),
            State::Menu(_) => (),
            State::Text(entry,_) => self.state = State::Menu(entry),
        }
    }
    pub fn goto_state_game(&mut self) {
        self.joystick_menu_state = JoystickMenuState::Released;
        baal::effect::resume();

        self.state = State::Game;
    }
    pub fn goto_state_text(&mut self, text: String) {
        baal::effect::pause();

        match self.state {
            State::Game => self.state = State::Text(0,text),
            State::Text(entry,_) | State::Menu(entry) => self.state = State::Text(entry,text),
        }
    }
    pub fn goto_level(&mut self, level: levels::Level) {
        while let Ok(_) = self.control_rx.try_recv() {}
        while let Ok(_) = self.effect_rx.try_recv() {}

        if let Some(e) = levels::load_level(&level,&self.castles,self.planner.mut_world()).err() {
            let level_name = match level {
                levels::Level::Room { castle: c, dungeon: d, room: r } => format!("room (castle: {:?}, dungeon: {:?}, room: {:?})",
                self.castles.get(c),
                self.castles.get(c).and_then(|c| c.dungeons.get(d)),
                self.castles.get(c).and_then(|c| c.dungeons.get(d)).and_then(|d| d.rooms.get(r)),
                ),
                levels::Level::Corridor { castle: c } => format!("corridor (castle: {:?})",self.castles.get(c)),
                levels::Level::Entry => "entry".into(),
            };
            panic!(format!("ERROR: failed to load level {}: {}",level_name,e));
        }

        self.current_level = level;
        self.update_player_control();
    }
    pub fn focused(&mut self, focus: bool) {
        self.focus = focus;

        if focus {
            baal::music::resume()
        } else {
            baal::music::pause()
        }
    }
    pub fn update(&mut self, args: event_loop::UpdateArgs) {
        if !self.focus {
            return
        }

        match self.state {
            State::Game => {
                let context = UpdateContext {
                    dt: args.dt as f32 * self.difficulty,
                    effect_tx: self.effect_tx.clone(),
                    control_tx: self.control_tx.clone(),
                };

                self.planner.dispatch(context);
                self.planner.wait();
            },
            State::Menu(_) | State::Text(_,_) => {
                let dir = if let JoystickMenuState::Pressed(dir, ref mut time) = self.joystick_menu_state {
                    if *time <= 0. {
                        *time = config.joystick.time_to_repeat;
                        Some(dir)
                    } else {
                        *time -= args.dt as f32;
                        None
                    }
                } else { None };

                if let Some(dir) = dir {
                    self.dir_pressed(dir);
                }
            },
        }
        while let Ok(control) = self.control_rx.try_recv() {
            match control {
                Control::GotoLevel(level) => self.goto_level(level),
                Control::ResetLevel => {
                    let level = self.current_level.clone();
                    self.goto_level(level);
                    self.goto_state_game();
                }
                Control::ResetCastle => {
                    let level = match self.current_level {
                        levels::Level::Room { castle, dungeon: _, room: _ } => levels::Level::Corridor { castle: castle },
                        levels::Level::Corridor { castle } => levels::Level::Corridor { castle: castle },
                        levels::Level::Entry => levels::Level::Entry,
                    };
                    self.goto_level(level);
                    self.goto_state_game();
                }
                Control::ResetGame => {
                    self.goto_level(levels::Level::Entry);
                    self.goto_state_game();
                }
                Control::CreateBall(pos,arc) => entities::add_ball(self.planner.mut_world(),pos,arc),
            }
        }
    }
    pub fn render(&mut self, args: event_loop::RenderArgs) {
        let dt = 1. / config.event_loop.max_fps as f32;

        match self.state {
            State::Game => {
                let world = self.planner.mut_world();

                // update camera
                {
                    let characters = world.read::<PlayerControl>();
                    let fixed_cameras = world.read::<FixedCamera>();
                    let states = world.read::<PhysicState>();

                    for (_, state) in (&characters, &states).iter() {
                        self.camera.x = state.position[0];
                        self.camera.y = state.position[1];
                    }
                    if fixed_cameras.iter().next().is_some() {
                        self.camera.x = 0.;
                        self.camera.y = 0.;
                    }
                }

                let mut frame = graphics::Frame::new(&mut self.graphics, args.frame, &self.camera);

                // draw entities
                {
                    let states = world.read::<PhysicState>();
                    let fixed_camera_texts = world.read::<FixedCameraText>();
                    let texts = world.read::<Text>();
                    let types = world.read::<PhysicType>();
                    let graphics = world.read::<Graphic>();
                    let squares = world.read::<GridSquare>();

                    for (square, graphic) in (&squares, &graphics).iter() {
                        let p = square.position;
                        frame.draw_square(p[0],p[1],0.5,graphic.layer,graphic.color);
                    }

                    for (state, typ, graphic) in (&states, &types, &graphics).iter() {
                        let x = state.position[0];
                        let y = state.position[1];
                        match typ.shape {
                            Shape::Circle(radius) => frame.draw_circle(x,y,radius,graphic.layer,graphic.color),
                            Shape::Square(radius) => frame.draw_square(x,y,radius,graphic.layer,graphic.color),
                        }
                    }

                    if config.text.right > config.text.left {
                        for text in fixed_camera_texts.iter() {
                            for (y,text_line) in (config.text.bottom+3..config.text.top+1).rev().zip(text.string.lines()) {
                                frame.draw_text(config.text.left as f32, y as f32, config.graphics.font_scale, text_line,graphics::Layer::Floor, config.entities.text_color);
                            }
                        }
                    }

                    for text in texts.iter() {
                        frame.draw_text(text.x, text.y, text.scale, &*text.string, graphics::Layer::Floor, config.entities.text_color);
                    }
                }

                // draw effects
                //TODO draw effects: do not next if pause
                for effect in &self.effect_storage {
                    effect.draw(&mut frame);
                }
                let old_effect_storage = self.effect_storage.drain(..).collect::<Vec<Effect>>();;
                for effect in old_effect_storage {
                    if let Some(effect) = effect.next(dt) {
                        self.effect_storage.push(effect)
                    }
                }

                while let Ok(effect) = self.effect_rx.try_recv() {
                    effect.draw(&mut frame);
                    if let Some(effect) = effect.next(dt) {
                        self.effect_storage.push(effect);
                    }
                }

                frame.finish().unwrap();
            },
            State::Menu(entry) => {
                let mut menu = String::new();
                let mut cursor = String::new();
                for (index,menu_entry) in self.menu.iter().enumerate() {
                    if index == entry {
                        cursor.push_str("<<                     >>\n");
                    } else {
                        cursor.push('\n');
                    }
                    menu.push_str(&*(*menu_entry.name)(&self));
                    menu.push('\n');

                    if self.menu_interline.contains(&index) {
                        cursor.push('\n');
                        menu.push('\n');
                    }
                }
                let mut frame = graphics::Frame::new(&mut self.graphics, args.frame, &self.camera);
                frame.draw_billboard_centered_text(&*cursor,config.menu.cursor_color);
                frame.draw_billboard_centered_text(&*menu,config.menu.entry_color);
                frame.finish().unwrap();
            }
            State::Text(_,ref text) => {
                let mut frame = graphics::Frame::new(&mut self.graphics, args.frame, &self.camera);
                frame.draw_rectangle(0.,0.,25.0,18.0,graphics::Layer::BillBoard,config.menu.background_color);
                frame.draw_billboard_centered_text(&*text,config.menu.entry_color);
                frame.finish().unwrap();
            }
        }

    }
    pub fn dir_pressed(&mut self, direction: Direction) {
        use std::ops::Rem;

        match self.state {
            State::Game => {
                self.player_control_state.push_keyboard_dir(direction);
                self.update_player_control();
            },
            State::Menu(entry) => {
                baal::effect::short::play_on_listener(config.menu.clic_snd);
                match direction {
                    Direction::Up => self.state = State::Menu(if entry == 0 { self.menu.len()-1 } else { entry-1 }),
                    Direction::Down => self.state = State::Menu((entry+1).rem(self.menu.len())),
                    Direction::Right => (*self.menu[entry].right.clone())(self),
                    Direction::Left => (*self.menu[entry].left.clone())(self),
                }
            }
            State::Text(entry,_) => {
                baal::effect::short::play_on_listener(config.menu.clic_snd);
                self.state = State::Menu(entry)
            }
        }
    }
    pub fn dir_released(&mut self, direction: Direction) {
        match self.state {
            State::Game => {
                self.player_control_state.retain_keyboard_dir(direction);
                self.update_player_control();
            },
            _ => (),
        }
    }
    pub fn escape_pressed(&mut self) {
        baal::effect::short::play_on_listener(config.menu.clic_snd);
        match self.state {
            State::Game | State::Text(_,_) => self.goto_state_menu(),
            State::Menu(_) => self.goto_state_game(),
        }
    }
    pub fn key_pressed(&mut self, key: u8) {
        if config.keys.up.contains(&key) {
            self.dir_pressed(Direction::Up);
        } else if config.keys.down.contains(&key) {
            self.dir_pressed(Direction::Down);
        } else if config.keys.left.contains(&key) {
            self.dir_pressed(Direction::Left);
        } else if config.keys.right.contains(&key) {
            self.dir_pressed(Direction::Right);
        } else if config.keys.escape.contains(&key) {
            self.escape_pressed()
        }
    }
    pub fn key_released(&mut self, key: u8) {
        if config.keys.up.contains(&key) {
            self.dir_released(Direction::Up);
        } else if config.keys.down.contains(&key) {
            self.dir_released(Direction::Down);
        } else if config.keys.left.contains(&key) {
            self.dir_released(Direction::Left);
        } else if config.keys.right.contains(&key) {
            self.dir_released(Direction::Right);
        }
    }
    pub fn button_pressed(&mut self, button: gilrs::Button) {
        use gilrs::Button::*;
        match button {
            South | DPadDown => self.dir_pressed(Direction::Down),
            East | DPadRight => self.dir_pressed(Direction::Right),
            North | DPadUp => self.dir_pressed(Direction::Up),
            West | DPadLeft => self.dir_pressed(Direction::Left),
            Select => self.escape_pressed(),
            _ => (),
        }
    }
    pub fn button_released(&mut self, button: gilrs::Button) {
        use gilrs::Button::*;
        match button {
            South | DPadDown => self.dir_released(Direction::Down),
            East | DPadRight => self.dir_released(Direction::Right),
            North | DPadUp => self.dir_released(Direction::Up),
            West | DPadLeft => self.dir_released(Direction::Left),
            _ => (),
        }
    }
    pub fn touch(&mut self, touch: glutin::Touch) {
        use glium::glutin::TouchPhase::*;
        let loc = [touch.location.0,touch.location.1];
        if utils::inside_rectangle(loc,config.touch.escape_rec) {
            if let Started = touch.phase {
                self.escape_pressed();
            }
        } else if utils::inside_rectangle(loc,config.touch.joystick_rec) {
            let rec = config.touch.joystick_rec;

            match touch.phase {
                Started | Moved => {
                    let pos_x = ((loc[0]-rec[0])/rec[2]/2.)
                        .min(config.touch.joystick_radius)
                        .max(-config.touch.joystick_radius)
                        as f32;

                    let pos_y = ((loc[1]-rec[1])/rec[3]/2.)
                        .min(config.touch.joystick_radius)
                        .max(-config.touch.joystick_radius)
                        as f32;

                    self.axis_changed(gilrs::Axis::LeftStickX,pos_x);
                    self.axis_changed(gilrs::Axis::LeftStickY,pos_y);
                },
                Ended | Cancelled => {
                    self.axis_changed(gilrs::Axis::LeftStickX,0.);
                    self.axis_changed(gilrs::Axis::LeftStickY,0.);
                }
            }
        }
    }
    pub fn axis_changed(&mut self, axis: gilrs::Axis, pos: f32) {
        if !axis.is_horizontal() && !axis.is_vertical() {
            return;
        }

        match self.state {
            State::Game => {
                if axis.is_horizontal() {
                    self.player_control_state.set_axis_x_state(pos);
                    self.update_player_control();
                } else {
                    self.player_control_state.set_axis_y_state(pos);
                    self.update_player_control();
                }
            },
            State::Text(_,_) | State::Menu(_) => {
                match self.joystick_menu_state {
                    JoystickMenuState::Released => {
                        if pos.abs() >= config.joystick.press_epsilon {
                            let direction = match (axis.is_horizontal(), pos > 0.) {
                                (true,true)  => Direction::Right,
                                (true,false) => Direction::Left,
                                (false,true) => Direction::Up,
                                (false,false) => Direction::Down,
                            };
                            self.joystick_menu_state = JoystickMenuState::Pressed(direction,config.joystick.time_to_start_repeating);
                            self.dir_pressed(direction);
                        }
                    },
                    JoystickMenuState::Pressed(direction,_) => {
                        if pos.abs() <= config.joystick.release_epsilon && !(direction.perpendicular(&Direction::Up) ^ axis.is_horizontal()) {
                            self.joystick_menu_state = JoystickMenuState::Released;
                        }
                    },
                }
            }
        }
    }
}
