#[macro_use]
extern crate glium;
extern crate vecmath;
extern crate glium_text;
extern crate yaml_rust;
extern crate specs;

use glium::{
    Frame,
    SwapBuffersError,
    Surface,
    VertexBuffer,
    index,
    Program,
    DrawParameters,
    Depth,
    DepthTest,
};
use glium::backend::Facade;
use glium::program::ProgramCreationError;
use glium::vertex::BufferCreationError;
use glium::draw_parameters::Smooth;
use yaml_rust::yaml::Yaml;
use glium_text::{
    TextSystem,
    FontTexture,
};

pub type Transformation = vecmath::Matrix2x3<f32>;

pub trait Transformed {
    fn translate(self, x: f32, y: f32) -> Self;
    fn rotate(self, angle: f32) -> Self;
    fn scale(self, sx: f32, sy: f32) -> Self;
    fn identity() -> Self;
}

impl Transformed for Transformation {
    #[inline(always)]
    fn translate(self, x: f32, y: f32) -> Self {
        let trans = {
            [[ 1.,  0., x],
            [ 0.,  1., y]]
        };
        vecmath::row_mat2x3_mul(self, trans)
    }

    #[inline(always)]
    fn rotate(self, angle: f32) -> Self {
        let rot = {
            let c = angle.cos();
            let s = angle.sin();
            [[c, -s, 0.],
            [s,  c, 0.]]
        };
        vecmath::row_mat2x3_mul(self, rot)
    }

    #[inline(always)]
    fn scale(self, sx: f32, sy: f32) -> Self {
        let scale = {
            [[sx, 0., 0.],
            [0., sy, 0.]]
        };
        vecmath::row_mat2x3_mul(self, scale)
    }

    #[inline(always)]
    fn identity() -> Self {
        [[1.,0.,0.],
        [0.,1.,0.]]
    }
}

#[derive(Debug,Clone)]
pub struct Setting {
    pub colors: ColorsValue,
    pub mode: Mode,
    pub luminosity: f32,
    pub circle_precision: usize,
    pub font_precision: u32,
    /// should be mono
    pub font_file: String,
    pub font_ratio: f32,
}

macro_rules! color_from_yaml {
    ( $colors_hash:expr, $name:expr ) => {
        {
            let vec = try!(try!($colors_hash.get(&Yaml::String(String::from($name)))
                .ok_or_else(|| format!("colors map must have a {} key",$name))).as_vec()
                .ok_or_else(|| format!("{} must be a vector",$name)));
            assert_eq!(vec.len(),4);
            [
                try!(vec[0].as_f64().ok_or_else(|| format!("{} first element must a float",$name))) as f32,
                try!(vec[1].as_f64().ok_or_else(|| format!("{} second element must a float",$name))) as f32,
                try!(vec[2].as_f64().ok_or_else(|| format!("{} third element must a float",$name))) as f32,
                try!(vec[3].as_f64().ok_or_else(|| format!("{} fourth element must a float",$name))) as f32,
            ]
        }
    }
}

impl Setting {
    pub fn from_yaml(code: &Yaml) -> Result<Self,String> {
        let hash = try!(code.as_hash().ok_or_else(|| "config must be an associative array"));
        let mode = {
            let s: &str = try!(try!(hash.get(&Yaml::String(String::from("mode")))
                .ok_or_else(|| "config map must have mode key")).as_str()
                .ok_or_else(|| "mode must be a string"));
            match s {
                "dark" => Mode::Dark,
                "light" => Mode::Light,
                _ => panic!("mode must be \"dark\" or \"light\""),
            }
        };
        let luminosity = try!(try!(hash.get(&Yaml::String(String::from("luminosity")))
                .ok_or_else(|| "config map must have a luminosity key")).as_f64()
                .ok_or_else(|| "luminosity must be a float")) as f32;
        let font_precision = try!(try!(hash.get(&Yaml::String(String::from("font_precision")))
                .ok_or_else(|| "config map must have a font_precision key")).as_i64()
                .ok_or_else(|| "font_precision must be an integer")) as u32;
        let font_ratio = try!(try!(hash.get(&Yaml::String(String::from("font_ratio")))
                .ok_or_else(|| "config map must have a font_ratio key")).as_f64()
                .ok_or_else(|| "font_ratio must be a float")) as f32;
        let font_file: String = try!(try!(hash.get(&Yaml::String(String::from("font_file")))
                .ok_or_else(|| "config map must have font_file key")).as_str()
                .ok_or_else(|| "font_file must be a string")).into();
        let circle_precision = try!(try!(hash.get(&Yaml::String(String::from("circle_precision")))
                .ok_or_else(|| "config map must have a circle_precision key")).as_i64()
                .ok_or_else(|| "circle_precision must be an integer")) as usize;
        let colors = {
            let colors_hash = try!(try!(hash.get(&Yaml::String(String::from("colors")))
                .ok_or_else(|| "config map must have a colors key")).as_hash()
                .ok_or_else(|| "colors must be an associative array"));
            ColorsValue {
                base03: color_from_yaml!(colors_hash, "base03"),
                base02: color_from_yaml!(colors_hash, "base02"),
                base01: color_from_yaml!(colors_hash, "base01"),
                base00: color_from_yaml!(colors_hash, "base00"),
                base0: color_from_yaml!(colors_hash, "base0"),
                base1: color_from_yaml!(colors_hash, "base1"),
                base2: color_from_yaml!(colors_hash, "base2"),
                base3: color_from_yaml!(colors_hash, "base3"),
                yellow: color_from_yaml!(colors_hash, "yellow"),
                orange: color_from_yaml!(colors_hash, "orange"),
                red: color_from_yaml!(colors_hash, "red"),
                magenta: color_from_yaml!(colors_hash, "magenta"),
                violet: color_from_yaml!(colors_hash, "violet"),
                blue: color_from_yaml!(colors_hash, "blue"),
                cyan: color_from_yaml!(colors_hash, "cyan"),
                green: color_from_yaml!(colors_hash, "green"),
            }
        };

        Ok(Setting {
            colors: colors,
            mode: mode,
            luminosity: luminosity,
            circle_precision: circle_precision,
            font_precision: font_precision,
            font_file: font_file,
            font_ratio: font_ratio,
        })
    }
}

#[derive(Clone,Copy)]
struct Vertex {
    position: [f32;2],
}
implement_vertex!(Vertex, position);

pub struct TGraphics {
    colors: ColorsValue,
    colors_setting: ColorsValue,
    mode: Mode,
    quad_vertex_buffer: VertexBuffer<Vertex>,
    quad_indices: index::NoIndices,
    circle_vertex_buffer: VertexBuffer<Vertex>,
    circle_indices: index::NoIndices,
    line_vertex_buffer: VertexBuffer<Vertex>,
    line_indices: index::NoIndices,
    program: Program,
    text_system: TextSystem,
    font: FontTexture,
    font_ratio: f32,
    luminosity: f32,
}

#[derive(Debug)]
pub enum TGraphicsCreationError {
    ProgramCreationError(ProgramCreationError),
    BufferCreationError(BufferCreationError),
    FontFileOpenError(std::io::Error),
    FontCreationError(()),
}

impl TGraphics {
    pub fn new<F: Facade>(facade: &F, setting: Setting) -> Result<TGraphics,TGraphicsCreationError> {

        let quad_vertex = vec![
            Vertex { position: [-1., -1.] },
            Vertex { position: [ 1., -1.] },
            Vertex { position: [-1.,  1.] },
            Vertex { position: [ 1.,  1.] }
        ];
        let quad_vertex_buffer = try!(VertexBuffer::new(facade, &quad_vertex)
            .map_err(|bce| TGraphicsCreationError::BufferCreationError(bce)));

        let quad_indices = index::NoIndices(index::PrimitiveType::TriangleStrip);

        let mut circle_vertex = vec!(Vertex { position: [0., 0.] });
        {
            let delta_angle = std::f32::consts::PI * 2. / setting.circle_precision as f32;
            let mut angle = 0f32;
            circle_vertex.push(Vertex { position: [angle.cos(),angle.sin()]});
            for _ in 0..setting.circle_precision {
                angle += delta_angle;
                circle_vertex.push(Vertex { position: [angle.cos(),angle.sin()]});
            }
        }

        let circle_vertex_buffer = try!(VertexBuffer::new(facade, &circle_vertex)
            .map_err(|bce| TGraphicsCreationError::BufferCreationError(bce)));

        let circle_indices = index::NoIndices(index::PrimitiveType::TriangleFan);

        let line_vertex = vec![
            Vertex { position: [ 0., 0.] },
            Vertex { position: [ 1., 1.] },
        ];

        let line_vertex_buffer = try!(VertexBuffer::new(facade, &line_vertex)
            .map_err(|bce| TGraphicsCreationError::BufferCreationError(bce)));

        let line_indices = index::NoIndices(index::PrimitiveType::LinesList);

        let vertex_shader_src = r#"
            #version 140
            in vec2 position;
            uniform mat4 trans;
            uniform mat4 camera;
            void main() {
                mat4 matrix = camera * trans;
                gl_Position = matrix * vec4(position, 0.0, 1.0);
            }
        "#;
        let fragment_shader_src = r#"
            #version 140
            out vec4 out_color;
            uniform vec4 color;
            void main() {
                out_color = color;
            }
        "#;
        let program = try!(Program::from_source(facade, vertex_shader_src, fragment_shader_src, None)
            .map_err(|pce| TGraphicsCreationError::ProgramCreationError(pce)));

        let mut colors = setting.colors.clone();
        colors.apply(&mut |color: &mut [f32;4]| {
            color[0] *= setting.luminosity;
            color[1] *= setting.luminosity;
            color[2] *= setting.luminosity;
        });

        let text_system = glium_text::TextSystem::new(facade);
        let font_file = try!(std::fs::File::open(&std::path::Path::new(&setting.font_file))
            .map_err(|ioe| TGraphicsCreationError::FontFileOpenError(ioe)));
        let font = try!(glium_text::FontTexture::new(facade, font_file, setting.font_precision)
            .map_err(|fce| TGraphicsCreationError::FontCreationError(fce)));

        Ok(TGraphics {
            colors: colors,
            colors_setting: setting.colors,
            mode: setting.mode,
            quad_vertex_buffer: quad_vertex_buffer,
            quad_indices: quad_indices,
            circle_vertex_buffer: circle_vertex_buffer,
            circle_indices: circle_indices,
            line_vertex_buffer: line_vertex_buffer,
            line_indices: line_indices,
            program: program,
            text_system: text_system,
            font: font,
            font_ratio: setting.font_ratio,
            luminosity: setting.luminosity,
        })
    }

    pub fn set_luminosity(&mut self, luminosity: f32) {
        self.luminosity = luminosity;
        self.colors = self.colors_setting.clone();
        self.colors.apply(&mut |color: &mut [f32;4]| {
            color[0] *= luminosity;
            color[1] *= luminosity;
            color[2] *= luminosity;
        });
    }

    pub fn luminosity(&self) -> f32 {
        self.luminosity
    }

    pub fn mode(&self) -> Mode {
        self.mode
    }

    pub fn toggle_mode(&mut self) {
        self.mode = match self.mode {
            Mode::Light => Mode::Dark,
            Mode::Dark => Mode::Light,
        }
    }
}

pub struct TFrame<'a> {
    frame: Frame,
    t_graphics: &'a TGraphics,
    camera: [[f32;4];4],
    draw_parameters: DrawParameters<'a>,
}

pub struct Camera {
    pub x: f32,
    pub y: f32,
    pub zoom: f32,
    pub ratio: f32,
}

#[derive(Debug,Clone)]
pub struct Line {
    x: i32,
    y: i32,
    length: usize,
}

impl<'a> TFrame<'a> {
    pub fn new(t_graphics: &'a TGraphics, mut frame: Frame, camera: &Camera) -> TFrame<'a> {
        let camera_matrix = {
            let kx = camera.zoom;
            let ky = camera.zoom*camera.ratio;
            let dx = -camera.x;
            let dy = -camera.y;
            [
                [   kx,    0., 0., 0.],
                [   0.,    ky, 0., 0.],
                [   0.,    0., 1., 0.],
                [kx*dx, ky*dy, 0., 1.]
            ]
        };

        let background = Color::Base1.into_vec4(t_graphics.mode,&t_graphics.colors);
        frame.clear_color_and_depth((background[0],background[1],background[2],background[3]),0f32);;

        let draw_parameters = DrawParameters {
            smooth: Some(Smooth::DontCare),
            depth: Depth {
                test: DepthTest::IfMoreOrEqual,
                write: true,
                .. Default::default()
            },
            .. Default::default()
        };

        TFrame {
            camera: camera_matrix,
            frame: frame,
            t_graphics: t_graphics,
            draw_parameters: draw_parameters,
        }
    }

    pub fn draw_square(&mut self, x: f32, y: f32, radius: f32, layer: Layer, color: Color) {
        let trans = {
            [
                [ radius,     0.,           0., 0.],
                [     0., radius,           0., 0.],
                [     0.,     0.,           1., 0.],
                [      x,      y, layer.into(), 1.]
            ]
        };
        let uniform = uniform!{
            trans: trans,
            camera: self.camera,
            color: color.into_vec4(self.t_graphics.mode,&self.t_graphics.colors),
        };

        self.frame.draw(
            &self.t_graphics.quad_vertex_buffer,
            &self.t_graphics.quad_indices,
            &self.t_graphics.program,
            &uniform,
            &self.draw_parameters).unwrap();
    }

    pub fn draw_circle(&mut self, x: f32, y: f32, radius: f32, layer: Layer, color: Color) {
        let trans = {
            [
                [ radius,     0.,           0., 0.],
                [     0., radius,           0., 0.],
                [     0.,     0.,           1., 0.],
                [      x,      y, layer.into(), 1.]
            ]
        };

        let uniform = uniform!{
            trans: trans,
            camera: self.camera,
            color: color.into_vec4(self.t_graphics.mode,&self.t_graphics.colors),
        };

        self.frame.draw(
            &self.t_graphics.circle_vertex_buffer,
            &self.t_graphics.circle_indices,
            &self.t_graphics.program,
            &uniform,
            &self.draw_parameters).unwrap();
    }

    pub fn draw_text(&mut self, text: &str, lines: &Vec<Line>, layer: Layer, color: Color) {
        use std::io::Write;

        let color = {
            let c = color.into_vec4(self.t_graphics.mode,&self.t_graphics.colors);
            (c[0],c[2],c[2],c[3])
        };

        let mut index = 0;
        let mut remain = text;
        while remain.len() > 0 {
            if index == lines.len() {
                writeln!(&mut std::io::stderr(), "draw_text: text doesn't fit in lines: \t\"{}\"",text).unwrap();
                break;
            }

            let split = if lines[index].length*2 > remain.len() {
                remain.len()
            } else {
                lines[index].length*2
            };
            let (burn,not_burn) = remain.split_at(split);
            remain = not_burn;

            let text_display = glium_text::TextDisplay::new(&self.t_graphics.text_system, &self.t_graphics.font, burn);

            let dx = lines[index].x as f32;
            let dy = lines[index].y as f32;
            let dz = layer.into();
            let ratio = self.t_graphics.font_ratio;
            let trans = [
                [ 1.,    0., 0., 0.],
                [ 0., ratio, 0., 0.],
                [ 0.,    0., 1., 0.],
                [ dx,    dy, dz, 1.]
            ];

            let matrix = vecmath::row_mat4_mul(self.camera,trans);
            glium_text::draw(&text_display, &self.t_graphics.text_system, &mut self.frame, matrix, color);
            index += 1;
        }
    }

    pub fn draw_quad(&mut self, trans: Transformation, layer: Layer, color: Color) {
        let trans = {
               [[ trans[0][0], trans[1][0],           0., 0.],
                [ trans[0][1], trans[1][1],           0., 0.],
                [          0.,          0.,           1., 0.],
                [ trans[0][2], trans[1][2], layer.into(), 1.]]
        };
        let uniform = uniform!{
            trans: trans,
            camera: self.camera,
            color: color.into_vec4(self.t_graphics.mode,&self.t_graphics.colors),
        };

        self.frame.draw(
            &self.t_graphics.quad_vertex_buffer,
            &self.t_graphics.quad_indices,
            &self.t_graphics.program,
            &uniform,
            &self.draw_parameters).unwrap();
    }

    pub fn draw_line(&mut self, x: f32, y: f32, angle: f32, length: f32, width: f32, layer: Layer, color: Color) {
        let trans = {
            let kx = (length*angle.cos()) as f32;
            let ky = (length*angle.sin()) as f32;
            let dx = x as f32;
            let dy = y as f32;
            let dz = layer.into();
            [
                [ kx, 0., 0., 0.],
                [ 0., ky, 0., 0.],
                [ 0., 0., 1., 0.],
                    [ dx, dy, dz, 1.]
            ]
        };
        let uniform = uniform!{
            trans: trans,
            camera: self.camera,
            color: color.into_vec4(self.t_graphics.mode,&self.t_graphics.colors),
        };
        self.draw_parameters.line_width = Some(width);

        self.frame.draw(
            &self.t_graphics.line_vertex_buffer,
            &self.t_graphics.line_indices,
            &self.t_graphics.program,
            &uniform,
            &self.draw_parameters).unwrap()
    }

    #[inline]
    pub fn finish(self) -> Result<(), SwapBuffersError> {
        self.frame.finish()
    }
}

#[derive(Debug,Clone,Copy)]
pub enum Color {
    Base1,
    Base2,
    Base3,
    Base4,
    Base5,
    Yellow,
    Orange,
    Red,
    Magenta,
    Violet,
    Blue,
    Cyan,
    Green,
}

impl Color {
    fn into_vec4(self, mode: Mode, colors_value: &ColorsValue) -> [f32;4] {
        match self {
            Color::Base1 => match mode {
                Mode::Light => colors_value.base3,
                Mode::Dark => colors_value.base03,
            },
            Color::Base2 => match mode {
                Mode::Light => colors_value.base2,
                Mode::Dark => colors_value.base02,
            },
            Color::Base3 => match mode {
                Mode::Light => colors_value.base1,
                Mode::Dark => colors_value.base01,
            },
            Color::Base4 => match mode {
                Mode::Light => colors_value.base01,
                Mode::Dark => colors_value.base1,
            },
            Color::Base5 => match mode {
                Mode::Light => colors_value.base02,
                Mode::Dark => colors_value.base2,
            },
            Color::Yellow => colors_value.yellow,
            Color::Orange => colors_value.orange,
            Color::Red => colors_value.red,
            Color::Magenta => colors_value.magenta,
            Color::Violet => colors_value.violet,
            Color::Blue => colors_value.blue,
            Color::Cyan => colors_value.cyan,
            Color::Green => colors_value.green,
        }
    }
}
impl specs::Component for Color {
    type Storage = specs::VecStorage<Self>;
}

#[derive(Debug,Clone,Copy)]
pub enum Layer {
    Floor,
    Middle,
    Ceil,
}

impl Into<f32> for Layer {
    fn into(self) -> f32 {
        match self {
            Layer::Floor => 0.,
            Layer::Middle => 0.5,
            Layer::Ceil => 1.,
        }
    }
}

#[derive(Debug,Clone,Copy)]
pub enum Mode {
    Light,
    Dark,
}

#[derive(Debug,Clone)]
pub struct ColorsValue {
    pub base03: [f32;4],
    pub base02: [f32;4],
    pub base01: [f32;4],
    pub base00: [f32;4],
    pub base0: [f32;4],
    pub base1: [f32;4],
    pub base2: [f32;4],
    pub base3: [f32;4],
    pub yellow: [f32;4],
    pub orange: [f32;4],
    pub red: [f32;4],
    pub magenta: [f32;4],
    pub violet: [f32;4],
    pub blue: [f32;4],
    pub cyan: [f32;4],
    pub green: [f32;4],
}

impl ColorsValue {
    fn apply<F: FnMut(&mut [f32;4])>(&mut self, callback: &mut F) {
        callback(&mut self.base03);
        callback(&mut self.base02);
        callback(&mut self.base01);
        callback(&mut self.base00);
        callback(&mut self.base0);
        callback(&mut self.base1);
        callback(&mut self.base2);
        callback(&mut self.base3);
        callback(&mut self.yellow);
        callback(&mut self.orange);
        callback(&mut self.red);
        callback(&mut self.magenta);
        callback(&mut self.violet);
        callback(&mut self.blue);
        callback(&mut self.cyan);
        callback(&mut self.green);
    }
}

#[test]
fn main_test() {
    use glium::DisplayBuild;
    use yaml_rust::YamlLoader;

    let display = glium::glutin::WindowBuilder::new()
        .with_dimensions(640,480)
        .with_multisampling(2)
        .build_glium()
        .unwrap();

    let yaml_config = YamlLoader::load_from_str(
"---
colors:
    base03: [ 0., 0.16862746, 0.21176471, 1. ]
    base02: [ 0.02745098, 0.21176471, 0.25882354, 1. ]
    base01: [ 0.34509805, 0.43137255, 0.45882353, 1. ]
    base00: [ 0.39607844, 0.48235294, 0.5137255, 1. ]
    base0: [ 0.5137255, 0.5803922, 0.5882353, 1. ]
    base1: [ 0.5764706, 0.6313726, 0.6313726, 1. ]
    base2: [ 0.93333334, 0.9098039, 0.8352941, 1. ]
    base3: [ 0.99215686, 0.9647059, 0.8901961, 1. ]
    yellow: [ 0.70980394, 0.5372549, 0., 1. ]
    orange: [ 0.79607844, 0.29411766, 0.08627451, 1. ]
    red: [ 0.8627451, 0.19607843, 0.18431373, 1. ]
    magenta: [ 0.827451, 0.21176471, 0.50980395, 1. ]
    violet: [ 0.42352942, 0.44313726, 0.76862746, 1. ]
    blue: [ 0.14901961, 0.54509807, 0.8235294, 1. ]
    cyan: [ 0.16470589, 0.6313726, 0.59607846, 1. ]
    green: [ 0.52156866, 0.6, 0., 1. ]
mode: dark
luminosity: 0.5
circle_precision: 32
font_file: assets/DejaVuSansMono-Bold.ttf
font_precision: 24
font_ratio: 1.5
...
").unwrap();

    let setting = Setting::from_yaml(&yaml_config[0]).unwrap();

    let graphics = TGraphics::new(&display,setting).unwrap();

    let camera = Camera {
        x: 0.,
        y: 0.,
        zoom: 0.05,
        ratio: 1.33,
    };

    let trans = Transformation::identity().scale(0.1,0.5).translate(0.2,0.9);

    for _ in 0..10 {
        let mut target = TFrame::new(&graphics,display.draw(),&camera);
        target.draw_square(0.,0.,1.,Layer::Floor,Color::Base2);
        target.draw_circle(0.,0.,10.,Layer::Middle,Color::Base3);
        target.draw_quad(trans,Layer::Ceil,Color::Base4);
        target.draw_line(0.,0.,1.,10.,1.,Layer::Ceil,Color::Base5);
        target.draw_text("target.draw_text",&vec!(Line {x:0,y:0,length:10}),Layer::Ceil,Color::Base5);
        target.finish().unwrap();
        for ev in display.poll_events() {
            match ev {
                glium::glutin::Event::Closed => return,
                _ => ()
            }
        }
        std::thread::sleep(std::time::Duration::from_millis(100));
    }
}
