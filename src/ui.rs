// TODO : maybe add a title to each menu
use app;
use app::App;
use graphics;
use graphics::Layer::BillBoard;
use graphics::Color;


const WIDTH: f32 = 0.3;
const HEIGHT: f32 = 0.3;
const OUTER: f32 = 0.25;
const INNER: f32 = 0.22;

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
    fn draw(&self, y: f32, app_ui_texts: &app::UITexts, frame: &mut graphics::Frame) {
        match self {
            &Widget::Spin { ref left_text, ref right_text, ref text, ..  } => {
                frame.draw_rectangle(0., y*HEIGHT, WIDTH, OUTER, BillBoard, Color::Base4);
                frame.draw_rectangle(0., y*HEIGHT, WIDTH, INNER, BillBoard, Color::Base2);
                frame.draw_text(text(app_ui_texts), 0., y*HEIGHT, HEIGHT/2., BillBoard, Color::Base2);
            },
            &Widget::Button { ref text, .. } => {
                frame.draw_rectangle(0., y*HEIGHT, WIDTH, OUTER, BillBoard, Color::Base4);
                frame.draw_rectangle(0., y*HEIGHT, WIDTH, INNER, BillBoard, Color::Base2);
                frame.draw_text(text(app_ui_texts), 0., y*HEIGHT, HEIGHT/2., BillBoard, Color::Base2);
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

type Focus = Option<(WidgetId, Position)>;

enum Position {
    Left,
    Right,
    Middle,
}

enum State {
    CursorGame {
        focus: bool
    },
    CursorMenu {
        menu_stack: Vec<(MenuId, WidgetId)>,
        current_menu: MenuId,
        focus: Focus,
    },
    ButtonGame,
    ButtonMenu {
        menu_stack: Vec<(MenuId, WidgetId)>,
        current_menu: MenuId,
        focus: WidgetId,
    },
}

impl State {
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
            &mut ButtonMenu { ref menu_stack, ref current_menu, .. } => CursorMenu {
                menu_stack: menu_stack.clone(),
                current_menu: *current_menu,
                focus: None
            },
            &mut CursorMenu { ref menu_stack, ref current_menu, .. } => ButtonMenu {
                menu_stack: menu_stack.clone(),
                current_menu: *current_menu,
                focus: WidgetId(0)
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

enum Event {
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

    pub fn do_back(&mut self, app: &mut App) {
        self.state.switch_to_button_mode();
        self.state = match self.state {
            State::ButtonGame => State::ButtonMenu {
                current_menu: MenuId(0),
                focus: WidgetId(0),
                menu_stack: vec!(),
            },
            State::ButtonMenu { ref mut current_menu, ref mut focus, ref mut menu_stack } => if let Some(new_menu) = menu_stack.pop() {
                State::ButtonMenu {
                    menu_stack: menu_stack.clone(),
                    current_menu: new_menu.0,
                    focus: new_menu.1,
                }
            } else {
                State::ButtonGame
            },
            State::CursorGame { .. } | State::CursorMenu { .. } => unreachable!(),
        }
    }

    pub fn do_cursor_move(&mut self) {
        self.state.switch_to_cursor_mode();
        unimplemented!();
    }

    pub fn do_cursor_down(&mut self) {
        self.state.switch_to_cursor_mode();
        unimplemented!();
    }

    pub fn do_cursor_uo(&mut self, _app: &mut App) {
        self.state.switch_to_cursor_mode();
        unimplemented!();
    }

    pub fn handle_event(&mut self, event: Event) {
    //     // self.active = if let Some((MenuId(menu_id), WidgetId(widget_id))) = self.active {
    //     //     let widget_len = self.menus[menu_id].widgets.len();
    //     match action {
    //         Left => {
    //             match self.menus[menu_id].widgets[widget_id] {
    //                 Widget::Spin { left_callback: ref callback, .. } | Widget::Button { ref callback, .. } => {
    //                     callback(app);
    //                     Some((MenuId(menu_id), WidgetId(widget_id)))
    //                 },
    //             }
    //         },
    //         Right => {
    //             match self.menus[menu_id].widgets[widget_id] {
    //                 Widget::Spin { right_callback: ref callback, .. } | Widget::Button { ref callback, .. } => {
    //                     callback(app);
    //                     Some((MenuId(menu_id), WidgetId(widget_id)))
    //                 },
    //             }
    //         },
    //         Up => if widget_id == 0 {
    //             Some((MenuId(menu_id), WidgetId(widget_len)))
    //         } else {
    //             Some((MenuId(menu_id), WidgetId(widget_id-1)))
    //         },
    //         Down => if widget_id == widget_len-1 {
    //             Some((MenuId(menu_id), WidgetId(0)))
    //         } else {
    //             Some((MenuId(menu_id), WidgetId(widget_id+1)))
    //         },
    //         Back => if let Some(menu_id) = self.menus[menu_id].parent {
    //             Some((menu_id, WidgetId(0)))
    //         } else {
    //             None
    //         },
    //     }
    }

    pub fn draw(&mut self, app_ui_texts: &app::UITexts, frame: &mut graphics::Frame) {
        match &self.state {
            &State::CursorMenu {
                current_menu,
                ref focus,
                ..
            } => {
                let ref widgets = self.menus[current_menu.0].widgets;
                let len = widgets.len();
                let iterator = widgets.iter()
                    .enumerate()
                    .map(|(i, widget)| ((len - 1) as f32 / 2. - i as f32, widget));
                for (y, widget) in iterator {
                    widget.draw(y, app_ui_texts, frame);
                }
            },
            _ => (),
        }
    }
}

fn main() {
}
