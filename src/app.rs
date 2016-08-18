use graphics;
use specs;
use utils::Direction;
use event_loop;
use config;
use glium;
use specs::Join;
use levels;
use systems::*;
use components::*;
use std::sync::mpsc;
use baal;
use std::rc::Rc;
use std::sync::Arc;
use entities;

static CREDIT: &'static str = "
thiolliere - thiolliere.org

";

static DONATE: &'static str = "
if you want to
please consider donate to:

TODO paypal

";

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
            Effect::Line { origin: o, length: le, angle: a, persistance: mut p, thickness: t, layer: la, color: c, }
            => {
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

// ajouter
pub enum Control {
    GotoLevel(levels::Level),
    ResetLevel,
    ResetGame,
    CreateBall([f32;2],Arc<()>),
}

#[derive(Clone)]
pub struct UpdateContext {
    pub effect_tx: mpsc::Sender<Effect>,
    pub control_tx: mpsc::Sender<Control>,
    pub dt: f64,
    pub master_entity: specs::Entity,
}

#[derive(PartialEq)]
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
    fn new(name: Box<Fn(&App)->String>, left: Rc<Box<Fn(&mut App)>>, right: Rc<Box<Fn(&mut App)>>) -> Self {
        MenuEntry {
            name: name,
            left: left,
            right: right,
        }
    }
}

pub struct App {
    menu: Vec<MenuEntry>,
    castles: Vec<levels::Castle>,
    state: State,
    current_level: levels::Level,
    camera: graphics::Camera,
    graphics: graphics::Graphics,
    planner: specs::Planner<UpdateContext>,
    player_dir: Vec<Direction>,
    control_rx: mpsc::Receiver<Control>,
    control_tx: mpsc::Sender<Control>,
    effect_rx: mpsc::Receiver<Effect>,
    effect_storage: Vec<Effect>,
    effect_tx: mpsc::Sender<Effect>,
    master_entity: specs::Entity,
    pub quit: bool,
}

impl App {
    pub fn new<F: glium::backend::Facade>(facade: &F, castles: Vec<levels::Castle>) -> Result<App,String> {
        // level
        let level = levels::Level::Entry;

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
            font_precision: config.graphics.font_precision,
            font_file: config.graphics.font_file.clone(),
            font_ratio: config.graphics.font_ratio,
            billboard_font_length: config.graphics.billboard_font_length,
            billboard_font_interline: config.graphics.billboard_font_interline,
        }).map_err(|e| format!("ERRROR: graphics init failed: {:#?}",e)));

        // init camera
        let camera = try!(graphics::Camera::new(facade, graphics::CameraSetting {
            zoom: config.camera.zoom
        }).map_err(|e| format!("ERROR: camera init failed: {:#?}",e)));

        // init world
        let mut world = specs::World::new();
        world.register::<PlayerControl>();
        world.register::<TowardPlayerControl>();
        world.register::<MonsterControl>();

        world.register::<PhysicState>();
        world.register::<PhysicForce>();
        world.register::<PhysicType>();
        world.register::<PhysicWorld>();
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
                        try!(levels::load(&level, &castles, &mut world)
                                .map_err(|e| format!("ERROR: load level failed: {} {:#?} {:#?}",e,level,castles)));
                    }
                }
            }
        }

        // load level
        let master_entity = try!(levels::load(&level, &castles, &mut world)
                                 .map_err(|e| format!("ERROR: load level failed: {}",e)));

        // init planner
        let mut planner = specs::Planner::new(world,config.general.number_of_thread);
        planner.add_system(PhysicSystem, "physic", 10);
        planner.add_system(PlayerSystem, "player", 5);
        planner.add_system(MonsterSystem, "monster", 5);
        planner.add_system(TowardPlayerSystem, "toward_player", 5);
        planner.add_system(KillerSystem, "killer", 5);
        planner.add_system(BallSystem, "ball", 5);
        planner.add_system(PortalSystem, "portal", 5);
        planner.add_system(ColumnSystem, "column", 5);
        planner.add_system(LifeSystem, "life", 1);

        let (effect_tx, effect_rx) = mpsc::channel();
        let (control_tx, control_rx) = mpsc::channel();

        // create menu
        let menu = vec!(
            MenuEntry::new(
                Box::new(|_| "continue".into()),
                Rc::new(Box::new(|app| app.state = State::Game)),
                Rc::new(Box::new(|app| app.state = State::Game))),
            MenuEntry::new(
                Box::new(|_| format!("global volume: {}",(baal::volume()*10.) as usize)),
                Rc::new(Box::new(|_| baal::set_volume((baal::volume()-0.1).max(0.0)))),
                Rc::new(Box::new(|_| baal::set_volume((baal::volume()+0.1).min(1.0))))),
            MenuEntry::new(
                Box::new(|_| format!("music volume: {}",(baal::music::volume()*10.) as usize)),
                Rc::new(Box::new(|_| baal::music::set_volume((baal::music::volume()-0.1).max(0.0)))),
                Rc::new(Box::new(|_| baal::music::set_volume((baal::music::volume()+0.1).min(1.0))))),
            MenuEntry::new(
                Box::new(|_| format!("effects volume: {}",(baal::effect::volume()*10.) as usize)),
                Rc::new(Box::new(|_| baal::effect::set_volume((baal::effect::volume()-0.1).max(0.0)))),
                Rc::new(Box::new(|_| baal::effect::set_volume((baal::effect::volume()+0.1).min(1.0))))),
            MenuEntry::new(
                Box::new(|app| format!("switch: {}", match app.graphics.mode() {
                    graphics::Mode::Dark => "dark",
                    graphics::Mode::Light => "light",
                })),
                Rc::new(Box::new(|app| app.graphics.toggle_mode())),
                Rc::new(Box::new(|app| app.graphics.toggle_mode()))),
            MenuEntry::new(
                Box::new(|app| format!("luminosity: {}", (app.graphics.luminosity()*10.) as usize)),
                Rc::new(Box::new(|app| {
                    let l = app.graphics.luminosity();
                    app.graphics.set_luminosity((l-0.1).max(0.0));
                })),
                Rc::new(Box::new(|app| {
                    let l = app.graphics.luminosity();
                    app.graphics.set_luminosity((l+0.1).min(1.0));
                }))),
            MenuEntry::new(
                Box::new(|_| "restart room".into()),
                Rc::new(Box::new(|app| {
                    app.control_tx.send(Control::ResetLevel).unwrap();
                })),
                Rc::new(Box::new(|app| {
                    app.control_tx.send(Control::ResetLevel).unwrap();
                }))),
            MenuEntry::new(
                Box::new(|_| "restart game".into()),
                Rc::new(Box::new(|app| {
                    app.control_tx.send(Control::ResetGame).unwrap();
                })),
                Rc::new(Box::new(|app| {
                    app.control_tx.send(Control::ResetGame).unwrap();
                }))),
            MenuEntry::new(
                Box::new(|_| "donate".into()),
                Rc::new(Box::new(|app| {
                    let entry = if let State::Menu(e) = app.state { e } else { 0 };
                    app.state = State::Text(entry,DONATE.into());
                })),
                Rc::new(Box::new(|app| {
                    let entry = if let State::Menu(e) = app.state { e } else { 0 };
                    app.state = State::Text(entry,DONATE.into());
                }))),
            MenuEntry::new(
                Box::new(|_| "credit".into()),
                Rc::new(Box::new(|app| {
                    let entry = if let State::Menu(e) = app.state { e } else { 0 };
                    app.state = State::Text(entry,CREDIT.into());
                })),
                Rc::new(Box::new(|app| {
                    let entry = if let State::Menu(e) = app.state { e } else { 0 };
                    app.state = State::Text(entry,CREDIT.into());
                }))),
            MenuEntry::new(
                Box::new(|_| "quit".into()),
                Rc::new(Box::new(|app| app.quit = true)),
                Rc::new(Box::new(|app| app.quit = true))),
            );

        Ok(App {
            menu: menu,
            state: State::Game,
            castles: castles,
            current_level: level,
            effect_storage: Vec::new(),
            camera: camera,
            graphics: graphics,
            planner: planner,
            player_dir: vec!(),
            master_entity: master_entity,
            effect_rx: effect_rx,
            effect_tx: effect_tx,
            control_rx: control_rx,
            control_tx: control_tx,
            quit: false,
        })
    }
    pub fn update(&mut self, args: event_loop::UpdateArgs) {
        if self.state == State::Game {
            let context = UpdateContext {
                dt: args.dt,
                master_entity: self.master_entity,
                effect_tx: self.effect_tx.clone(),
                control_tx: self.control_tx.clone(),
            };

            self.planner.dispatch(context);
            self.planner.wait();
        }
        while let Ok(control) = self.control_rx.try_recv() {
            match control {
                Control::GotoLevel(level) => self.goto_level(level),
                Control::ResetLevel => {
                    let level = self.current_level.clone();
                    self.goto_level(level);
                    self.state = State::Game;
                }
                Control::ResetGame => {
                    self.goto_level(levels::Level::Entry);
                    self.state = State::Game;
                }
                Control::CreateBall(pos,arc) => entities::add_ball(self.planner.mut_world(),pos,arc),
            }
        }
    }
    pub fn goto_level(&mut self, level: levels::Level) {
        while let Ok(_) = self.control_rx.try_recv() {}
        while let Ok(_) = self.effect_rx.try_recv() {}

        self.master_entity = match levels::load(&level,&self.castles,self.planner.mut_world()) {
            Err(e) => panic!(format!("ERROR: load level failed: {}",e)),
            Ok(m) => m,
        };

        self.current_level = level;
        let mut player_dir = vec!();
        player_dir.append(&mut self.player_dir);
        for k in player_dir {
            match k {
                Direction::Up => self.key_pressed(config.keys.up[0]),
                Direction::Down => self.key_pressed(config.keys.down[0]),
                Direction::Right => self.key_pressed(config.keys.right[0]),
                Direction::Left => self.key_pressed(config.keys.left[0]),
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
                    let states = world.read::<PhysicState>();
                    for (_, state) in (&characters, &states).iter() {
                        self.camera.x = state.position[0];
                        self.camera.y = state.position[1];
                    }
                }

                let mut frame = graphics::Frame::new(&self.graphics, args.frame, &self.camera);

                // draw entities
                {
                    let states = world.read::<PhysicState>();
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
                }

                // draw effects
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
                frame.draw_text("target.draw_text 0,0puis 1.1mmmmmmmmmmmmmmmmmmmmm",&vec!(graphics::Line {x:0,y:0,length:10},graphics::Line {x:1,y:1,length:100}),graphics::Layer::Ceil,graphics::Color::Base5);
                frame.finish().unwrap();
            },
            State::Menu(entry) => {
                let mut frame = graphics::Frame::new(&self.graphics, args.frame, &self.camera);
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
                }
                frame.draw_rectangle(0.,0.,config.menu.background_width,config.menu.background_height,graphics::Layer::BillBoard,config.menu.background_color);
                frame.draw_billboard_centered_text(&*cursor,config.menu.cursor_color);
                frame.draw_billboard_centered_text(&*menu,config.menu.entry_color);
                frame.finish().unwrap();
            }
            State::Text(_,ref text) => {
                let mut frame = graphics::Frame::new(&self.graphics, args.frame, &self.camera);
                frame.draw_rectangle(0.,0.,25.0,18.0,graphics::Layer::BillBoard,config.menu.background_color);
                frame.draw_billboard_centered_text(&*text,config.menu.entry_color);
                frame.finish().unwrap();
            }
        }

    }
    pub fn key_pressed(&mut self, key: u8) {
        use std::ops::Rem;

        let direction = if config.keys.up.contains(&key) {
            Some(Direction::Up)
        } else if config.keys.down.contains(&key) {
            Some(Direction::Down)
        } else if config.keys.left.contains(&key) {
            Some(Direction::Left)
        } else if config.keys.right.contains(&key) {
            Some(Direction::Right)
        } else {
            None
        };

        if let Some(direction) = direction {
            match self.state {
                State::Game => {
                    if !self.player_dir.contains(&direction) {
                        self.player_dir.push(direction);
                        self.update_player_direction();
                    }
                },
                State::Menu(entry) => {
                    match direction {
                        Direction::Up => self.state = State::Menu(if entry == 0 { self.menu.len()-1 } else { entry-1 }),
                        Direction::Down => self.state = State::Menu((entry+1).rem(self.menu.len())),
                        Direction::Right => (*self.menu[entry].right.clone())(self),
                        Direction::Left => (*self.menu[entry].left.clone())(self),
                    }
                }
                State::Text(entry,_) => {
                    self.state = State::Menu(entry)
                }
            }
        }

        if config.keys.escape.contains(&key) {
            match self.state {
                State::Game => self.state = State::Menu(0),
                State::Menu(_) => self.state = State::Game,
                State::Text(entry,_) => self.state = State::Menu(entry),
            }
        }

    }
    pub fn key_released(&mut self, key: u8) {
        let direction = if config.keys.up.contains(&key) {
            Some(Direction::Up)
        } else if config.keys.down.contains(&key) {
            Some(Direction::Down)
        } else if config.keys.left.contains(&key) {
            Some(Direction::Left)
        } else if config.keys.right.contains(&key) {
            Some(Direction::Right)
        } else {
            None
        };

        if let Some(direction) = direction {
            if self.state == State::Game {
                self.player_dir.retain(|dir| &direction != dir);
                self.update_player_direction();
            }
        }
    }
    fn update_player_direction(&mut self) {
        use std::f32::consts::PI;

        let world = self.planner.mut_world();

        if let Some(dir) = self.player_dir.last() {

            let mut last_perpendicular: Option<&Direction> = None;
            for d in &self.player_dir {
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
    }
    pub fn resize(&mut self, width: u32, height: u32) {
        self.camera.ratio = width as f32 / height as f32;
    }
}
