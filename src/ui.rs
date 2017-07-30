// TODO : maybe add a title to each menu
use app;
use app::App;
use graphics;
use graphics::Layer::BillBoard;
use graphics::Color;
use glium::glutin;

const WIDTH: f32 = 0.66;
const HEIGHT: f32 = 0.1;
const OUTER: f32 = 0.01;
const INNER: f32 = 0.01;
const FONT_SIZE: f32 = 0.5;
const SPIN_CENTER_WIDTH: f32 = 0.5;

#[derive(Clone, Copy)]
pub struct MenuId(usize);

#[derive(Clone, Copy)]
struct WidgetId(usize);

pub enum Widget {
    Spin {
        left_callback: Box<Fn(&mut App)>,
        right_callback: Box<Fn(&mut App)>,
        left_text: Box<Fn(&app::UITexts) -> &graphics::Text>,
        right_text: Box<Fn(&app::UITexts) -> &graphics::Text>,
        text: Box<Fn(&app::UITexts) -> &graphics::Text>,
    },
    Button {
        text: Box<Fn(&app::UITexts) -> &graphics::Text>,
        callback: Box<Fn(&mut App)>,
    },
}

fn draw_button(x: f32, y: f32, width: f32, text: &graphics::Text, focus: bool, pressed: bool, frame: &mut graphics::Frame) {
    frame.draw_rectangle(x*WIDTH, y*HEIGHT, width*WIDTH-OUTER, HEIGHT-OUTER, BillBoard, Color::Base4);
    frame.draw_rectangle(x*WIDTH, y*HEIGHT, width*WIDTH-OUTER-INNER, HEIGHT-OUTER-INNER, BillBoard, Color::Base2);
    frame.draw_text(text, x*WIDTH, y*HEIGHT, HEIGHT*FONT_SIZE, BillBoard, Color::Base4);
}

impl Widget {
    fn left(&self, app: &mut App) {
        use self::Widget::*;
        match *self {
            Spin { ref left_callback, .. } => left_callback(app),
            Button { ref callback, .. } => callback(app),
        }
    }
    fn right(&self, app: &mut App) {
        use self::Widget::*;
        match *self {
            Spin { ref right_callback, .. } => right_callback(app),
            Button { ref callback, .. } => callback(app),
        }
    }
    fn draw(&self, y: f32, app_ui_texts: &app::UITexts, focus: Option<Position>, frame: &mut graphics::Frame) {
        match self {
            &Widget::Spin { ref left_text, ref right_text, ref text, ..  } => {
                let border_size = (1. - SPIN_CENTER_WIDTH) / 2.;
                let x = SPIN_CENTER_WIDTH/2. + border_size/2.;

                draw_button(0., y, SPIN_CENTER_WIDTH, text(app_ui_texts), false, false, frame);
                draw_button(-x, y, border_size, left_text(app_ui_texts), false, false, frame);
                draw_button(x, y, border_size, right_text(app_ui_texts), false, false, frame);
            },
            &Widget::Button { ref text, .. } => {
                draw_button(0., y, 1., text(app_ui_texts), false, false, frame);
            }
        }
    }
}

pub struct Menu {
    widgets: Vec<Widget>,
}

impl Menu {
    fn new() -> Self {
        Menu {
            widgets: vec!(),
        }
    }
}

#[derive(Clone, Copy)]
struct Focus {
    widget_id: WidgetId,
    position: Position,
}

#[derive(Clone, Copy)]
enum Position {
    Left,
    Right,
    Middle,
    All,
}

enum State {
    CursorGame {
        focus: bool
    },
    CursorMenu {
        menu_stack: Vec<(MenuId, WidgetId)>,
        current_menu: MenuId,
        focus: Option<Focus>,
        pressed: bool,
    },
    ButtonGame,
    ButtonMenu {
        menu_stack: Vec<(MenuId, WidgetId)>,
        current_menu: MenuId,
        focus: WidgetId,
        pressed: bool
    },
}

impl State {
    fn is_in_menu(&self) -> bool {
        use self::State::*;
        match *self {
            ButtonMenu { .. } | CursorMenu { .. } => true,
            ButtonGame | CursorGame { .. } => false,
        }
    }
    fn is_button_mode(&self) -> bool {
        use self::State::*;
        match *self {
            CursorGame { .. } | CursorMenu { .. } => false,
            ButtonGame | ButtonMenu { .. } => true,
        }
    }

    fn is_cursor_mode(&self) -> bool { !self.is_button_mode() }

    fn switch_mode(&mut self) {
        use self::State::*;
        *self = match self {
            &mut CursorGame { .. } => ButtonGame,
            &mut ButtonGame => CursorGame { focus: false },
            &mut ButtonMenu { ref menu_stack, ref current_menu, ref pressed, .. } => CursorMenu {
                menu_stack: menu_stack.clone(),
                current_menu: *current_menu,
                focus: None,
                pressed: *pressed,
            },
            &mut CursorMenu { ref menu_stack, ref current_menu, ref pressed, .. } => ButtonMenu {
                menu_stack: menu_stack.clone(),
                current_menu: *current_menu,
                focus: WidgetId(0),
                pressed: *pressed,
            },
        };
    }

    fn switch_to_button_mode(&mut self) {
        if self.is_cursor_mode() { self.switch_mode() }
    }

    fn switch_to_cursor_mode(&mut self) {
        if self.is_button_mode() { self.switch_mode() }
    }
}

pub enum Event {
    CursorMove(f32, f32),
    CursorDown,
    CursorUp,
    Up,
    Down,
    Right,
    Left,
    Escape,
}

pub struct UI {
    menus: Vec<Menu>,
    state: State,
}

impl UI {
    pub fn new() -> UI {
        UI {
            menus: vec!(Menu::new()),
            state: State::CursorMenu {
                menu_stack: vec!(),
                current_menu: MenuId(0),
                focus: None,
                pressed: false,
            }
        }
    }

    pub fn main_menu(&self) -> MenuId {
        MenuId(0)
    }

    pub fn add_menu(&mut self) -> MenuId {
        self.menus.push(Menu::new());
        return MenuId(self.menus.len() - 1);
    }

    pub fn add_widget(&mut self, menu: MenuId, widget: Widget) {
        self.menus[menu.0].widgets.push(widget);
    }

    pub fn do_up(&mut self) {
        self.state.switch_to_button_mode();
        if let State::ButtonMenu { ref mut current_menu, ref mut focus, .. } = self.state {
            if focus.0 == 0 {
                focus.0 = self.menus[current_menu.0].widgets.len() - 1;
            } else {
                focus.0 -= 1;
            }
        }
    }

    pub fn do_down(&mut self) {
        self.state.switch_to_button_mode();
        if let State::ButtonMenu { ref mut current_menu, ref mut focus, .. } = self.state {
            if focus.0 == self.menus[current_menu.0].widgets.len() - 1 {
                focus.0 = 0;
            } else {
                focus.0 += 1;
            }
        }
    }

    pub fn do_left(&mut self, app: &mut App) {
        self.state.switch_to_button_mode();
        if let State::ButtonMenu { ref mut current_menu, ref mut focus, .. } = self.state {
            self.menus[current_menu.0].widgets[focus.0].left(app);
        }
    }

    pub fn do_right(&mut self, app: &mut App) {
        self.state.switch_to_button_mode();
        if let State::ButtonMenu { ref mut current_menu, ref mut focus, .. } = self.state {
            self.menus[current_menu.0].widgets[focus.0].right(app);
        }
    }

    pub fn do_escape(&mut self) {
        self.state.switch_to_button_mode();
        self.state = match self.state {
            State::ButtonGame => State::ButtonMenu {
                current_menu: MenuId(0),
                focus: WidgetId(0),
                menu_stack: vec!(),
                pressed: false,
            },
            State::ButtonMenu { ref mut current_menu, ref mut focus, ref mut menu_stack, .. } => if let Some(new_menu) = menu_stack.pop() {
                State::ButtonMenu {
                    menu_stack: menu_stack.clone(),
                    current_menu: new_menu.0,
                    focus: new_menu.1,
                    pressed: false,
                }
            } else {
                State::ButtonGame
            },
            State::CursorGame { .. } | State::CursorMenu { .. } => unreachable!(),
        }
    }

    pub fn do_cursor_move(&mut self, x: f32, y: f32) {
        self.state.switch_to_cursor_mode();
        unimplemented!();
    }

    pub fn do_cursor_down(&mut self) {
        self.state.switch_to_cursor_mode();
        unimplemented!();
    }

    pub fn do_cursor_up(&mut self, _app: &mut App) {
        self.state.switch_to_cursor_mode();
        unimplemented!();
    }

    pub fn handle_event(&mut self, event: Event, app: &mut App) {
        use ui::Event::*;
        match event {
            CursorMove(x, y) => self.do_cursor_move(x, y),
            CursorDown => self.do_cursor_down(),
            CursorUp => self.do_cursor_up(app),
            Up => self.do_up(),
            Down => self.do_down(),
            Right => self.do_right(app),
            Left => self.do_left(app),
            Escape => self.do_escape(),
        }
    }

    pub fn draw(&mut self, app_ui_texts: &app::UITexts, frame: &mut graphics::Frame) {
        if self.state.is_in_menu() {
            let focus = match &self.state {
                &State::CursorMenu { focus, ..  } => focus,
                &State::ButtonMenu { focus, ..  } => Some(Focus {
                    widget_id: focus,
                    position: Position::All
                }),
                _ => unreachable!(),
            };
            match &self.state {
                &State::CursorMenu { current_menu, .. }  | &State::ButtonMenu { current_menu, ..  } => {
                    let ref widgets = self.menus[current_menu.0].widgets;
                    let len = widgets.len();
                    let iterator = widgets.iter()
                        .enumerate()
                        .map(|(i, widget)| (i, (len - 1) as f32 / 2. - i as f32, widget));
                    for (id, y, widget) in iterator {
                        let focus = focus.and_then(|focus| {
                            if focus.widget_id.0 == id {
                                Some(focus.position)
                            } else {
                                None
                            }
                        });
                        widget.draw(y, app_ui_texts, focus, frame);
                    }
                }
                _ => unreachable!(),
            }
            // TODO in Cursor Menu draw topright return button
        } else {
            // TODO in Cursor Game draw topright escape button
        }
    }
}
