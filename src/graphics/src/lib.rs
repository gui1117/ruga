extern crate vecmath;
extern crate glium_text_rusttype as glium_text;
#[macro_use] extern crate glium;
#[macro_use] extern crate serde_derive;

use glium::{
    Blend,
    SwapBuffersError,
    Surface,
    VertexBuffer,
    index,
    vertex,
    Program,
    DrawParameters,
    Depth,
    DepthTest,
};
use glium::backend::{Facade, Context};
use glium::program::ProgramCreationError;
use glium::draw_parameters::Smooth;

use glium_text::{
    TextSystem,
    FontTexture,
    TextDisplay,
};

use std::fs::File;
use std::path::Path;
use std::rc::Rc;
use std::error::Error;
use std::fmt;

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
        [[1., 0., 0.],
        [0., 1., 0.]]
    }
}

#[derive(Debug)]
pub enum GraphicsCreationError {
    ProgramCreation(ProgramCreationError),
    VertexBufferCreation(vertex::BufferCreationError),
    FontTexture(glium_text::Error),
    FontFileOpen(std::io::Error),
}

impl Error for GraphicsCreationError {
    fn description(&self) -> &str {
        use self::GraphicsCreationError::*;
        match *self {
            ProgramCreation(ref e) => e.description(),
            VertexBufferCreation(ref e) => e.description(),
            FontTexture(ref _e) => "glium_text_rusttype: font texture error: {}",
            FontFileOpen(ref e) => e.description(),
        }
    }
    fn cause(&self) -> Option<&Error> {
        use self::GraphicsCreationError::*;
        match *self {
            ProgramCreation(ref e) => Some(e),
            VertexBufferCreation(ref e) => Some(e),
            FontTexture(ref _e) => None,
            FontFileOpen(ref e) => Some(e),
        }
    }
}

impl fmt::Display for GraphicsCreationError {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        use self::GraphicsCreationError::*;
        match *self {
            ProgramCreation(ref e) => write!(fmt, "Program creation error: {}", e),
            VertexBufferCreation(ref e) => write!(fmt, "Buffer creation error: {}", e),
            FontTexture(ref e) => write!(fmt, "Font Texture creation error: {:?}", e),
            FontFileOpen(ref e) => write!(fmt, "Font file opening error: {}", e),
        }
    }
}
impl From<ProgramCreationError> for GraphicsCreationError {
    fn from(err: ProgramCreationError) -> GraphicsCreationError {
        GraphicsCreationError::ProgramCreation(err)
    }
}
impl From<vertex::BufferCreationError> for GraphicsCreationError {
    fn from(err: vertex::BufferCreationError) -> GraphicsCreationError {
        GraphicsCreationError::VertexBufferCreation(err)
    }
}
impl From<glium_text::Error> for GraphicsCreationError {
    fn from(err: glium_text::Error) -> GraphicsCreationError {
        GraphicsCreationError::FontTexture(err)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GraphicsSetting {
    pub colors: ColorsValue,
    pub mode: Mode,
    pub luminosity: f32,
    pub circle_precision: usize,
    pub font: String,
    pub font_size: u32,
}

#[derive(Clone, Copy)]
struct Vertex {
    position: [f32;2],
}
implement_vertex!(Vertex, position);

#[derive(Copy, Clone)]
struct FontVertex {
    position: [f32; 2],
    tex_coords: [f32; 2],
}
implement_vertex!(FontVertex, position, tex_coords);

pub struct TextElement(usize);

pub struct Graphics {
    context: Rc<Context>,

    colors: ColorsValue,
    colors_setting: ColorsValue,
    mode: Mode,
    quad_vertex_buffer: VertexBuffer<Vertex>,
    quad_indices: index::NoIndices,
    circle_vertex_buffer: VertexBuffer<Vertex>,
    circle_indices: index::NoIndices,
    program: Program,
    luminosity: f32,

    text_system: TextSystem,
    font_texture: Rc<FontTexture>,

    draw_parameters: DrawParameters<'static>,
}

impl Graphics {
    pub fn new<F: Facade>(facade: &F, setting: GraphicsSetting) -> Result<Graphics, GraphicsCreationError> {
        let quad_vertex = vec![
            Vertex { position: [-1., -1.] },
            Vertex { position: [ 1., -1.] },
            Vertex { position: [-1.,  1.] },
            Vertex { position: [ 1.,  1.] }
        ];
        let quad_vertex_buffer = VertexBuffer::new(facade, &quad_vertex)?;

        let quad_indices = index::NoIndices(index::PrimitiveType::TriangleStrip);

        let mut circle_vertex = vec!(Vertex { position: [0., 0.] });
        {
            let delta_angle = std::f32::consts::PI * 2. / setting.circle_precision as f32;
            let mut angle = 0f32;
            circle_vertex.push(Vertex { position: [angle.cos(), angle.sin()]});
            for _ in 0..setting.circle_precision {
                angle += delta_angle;
                circle_vertex.push(Vertex { position: [angle.cos(), angle.sin()]});
            }
        }

        let circle_vertex_buffer = VertexBuffer::new(facade, &circle_vertex)?;

        let circle_indices = index::NoIndices(index::PrimitiveType::TriangleFan);

        let vertex_shader_src = r#"
            #version 100
            attribute vec2 position;
            uniform mat4 trans;
            uniform mat4 camera;
            void main() {
                mat4 matrix = camera * trans;
                gl_Position = matrix * vec4(position, 0.0, 1.0);
            }
        "#;
        let fragment_shader_src = r#"
            #version 100
            precision mediump float;
            uniform vec4 color;
            void main() {
                gl_FragColor = color;
            }
        "#;
        let program = Program::from_source(facade, vertex_shader_src, fragment_shader_src, None)?;

        let mut colors = setting.colors.clone();
        colors.apply(&mut |color: &mut [f32;4]| {
            color[0] *= setting.luminosity;
            color[1] *= setting.luminosity;
            color[2] *= setting.luminosity;
        });

        let draw_parameters = DrawParameters {
            smooth: Some(Smooth::DontCare),
            blend: Blend::alpha_blending(),
            depth: Depth {
                test: DepthTest::IfMoreOrEqual,
                write: true,
                .. Default::default()
            },
            .. Default::default()
        };

        let font_file = File::open(&Path::new(&setting.font))
            .map_err(|ioe| GraphicsCreationError::FontFileOpen(ioe))?;

        Ok(Graphics {
            context: facade.get_context().clone(),
            colors: colors,
            colors_setting: setting.colors,
            mode: setting.mode,
            quad_vertex_buffer: quad_vertex_buffer,
            quad_indices: quad_indices,
            circle_vertex_buffer: circle_vertex_buffer,
            circle_indices: circle_indices,
            program: program,
            luminosity: setting.luminosity,

            text_system: TextSystem::new(facade),
            font_texture: Rc::new(FontTexture::new(facade, font_file, setting.font_size, FontTexture::ascii_character_list())?),

            draw_parameters: draw_parameters,
        })
    }

    pub fn new_text_display(&self, text: &str) -> TextDisplay<Rc<FontTexture>> {
        TextDisplay::new(&self.text_system, self.font_texture.clone(), text)
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

pub struct Frame<'a> {
    frame: glium::Frame,
    graphics: &'a mut  Graphics,
    camera_matrix: [[f32;4];4],
    billboard_camera_matrix: [[f32;4];4],
}

#[derive(Clone, Debug)]
pub struct Camera {
    pub x: f32,
    pub y: f32,
    pub zoom: f32,
}

impl Camera {
    pub fn new(x: f32, y: f32, zoom: f32) -> Self {
        Camera {
            x: x,
            y: y,
            zoom: zoom,
        }
    }
}

impl<'a> Frame<'a> {
    pub fn new(graphics: &'a mut Graphics, mut frame: glium::Frame, camera: &'a Camera) -> Frame<'a> {
        let (width, height) = graphics.context.get_framebuffer_dimensions();
        let ratio = width as f32/ height as f32;

        let camera_matrix = {
            let (kx, ky) = if ratio > 1. {
                (camera.zoom / ratio,
                 camera.zoom)
            } else {
                (camera.zoom,
                 camera.zoom * ratio)
            };
            let dx = -camera.x;
            let dy = -camera.y;
            [
                [   kx,    0., 0., 0.],
                [   0.,    ky, 0., 0.],
                [   0.,    0., 1., 0.],
                [kx*dx, ky*dy, 0., 1.]
            ]
        };
        let billboard_camera_matrix = {
            let (kx, ky) = if ratio > 1. {
                (1. / ratio, 1.)
            } else {
                (1., ratio)
            };
            [
                [   kx,    0., 0., 0.],
                [   0.,    ky, 0., 0.],
                [   0.,    0., 1., 0.],
                [   0.,    0., 0., 1.]
            ]
        };

        frame.clear_depth(0f32);;

        let uniform = uniform!{
            trans: {
                let mut trans = vecmath::mat4_id::<f32>();
                trans[3][2] = 0.1;
                trans
            },
            camera: vecmath::mat4_id::<f32>(),
            color: Color::Base1.into_vec4(graphics.mode, &graphics.colors),
        };

        frame.draw(
            &graphics.quad_vertex_buffer,
            &graphics.quad_indices,
            &graphics.program,
            &uniform,
            &graphics.draw_parameters).unwrap();

        Frame {
            billboard_camera_matrix: billboard_camera_matrix,
            camera_matrix: camera_matrix,
            frame: frame,
            graphics: graphics,
        }
    }

    pub fn draw_square(&mut self, x: f32, y: f32, radius: f32, layer: Layer, color: Color) {
        self.draw_rectangle(x, y, radius*2., radius*2., layer, color);
    }

    pub fn draw_rectangle(&mut self, x: f32, y: f32, width: f32, height: f32, layer: Layer, color: Color) {
        let trans = {
            [
                [ width/2.,        0.,           0., 0.],
                [       0., height/2.,           0., 0.],
                [       0.,        0.,           1., 0.],
                [        x,         y, layer.into(), 1.]
            ]
        };

        let uniform = uniform!{
            trans: trans,
            camera: if layer == Layer::BillBoard { self.billboard_camera_matrix } else { self.camera_matrix },
            color: color.into_vec4(self.graphics.mode, &self.graphics.colors),
        };

        self.frame.draw(
            &self.graphics.quad_vertex_buffer,
            &self.graphics.quad_indices,
            &self.graphics.program,
            &uniform,
            &self.graphics.draw_parameters).unwrap();
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
            camera: if layer == Layer::BillBoard { self.billboard_camera_matrix } else { self.camera_matrix },
            color: color.into_vec4(self.graphics.mode, &self.graphics.colors),
        };

        self.frame.draw(
            &self.graphics.circle_vertex_buffer,
            &self.graphics.circle_indices,
            &self.graphics.program,
            &uniform,
            &self.graphics.draw_parameters).unwrap();
    }

    pub fn draw_quad(&mut self, trans: Transformation, layer: Layer, color: Color) {
        let trans = [
            [ trans[0][0], trans[1][0],           0., 0.],
            [ trans[0][1], trans[1][1],           0., 0.],
            [          0.,          0.,           1., 0.],
            [ trans[0][2], trans[1][2], layer.into(), 1.]
        ];
        let uniform = uniform!{
            trans: trans,
            camera: if layer == Layer::BillBoard { self.billboard_camera_matrix } else { self.camera_matrix },
            color: color.into_vec4(self.graphics.mode, &self.graphics.colors),
        };

        self.frame.draw(
            &self.graphics.quad_vertex_buffer,
            &self.graphics.quad_indices,
            &self.graphics.program,
            &uniform,
            &self.graphics.draw_parameters).unwrap();
    }

    pub fn draw_text(&mut self, text: &TextDisplay<Rc<FontTexture>>, x: f32, y: f32, size: f32, layer: Layer, color: Color) {
        let trans = [
            [                    1.,                     0.,           0., 0.],
            [                    0.,                   size,           0., 0.],
            [                    0.,                     0.,           1., 0.],
            [ x-text.get_width()/2., y-text.get_height()/3., layer.into(), 1.]
        ];

        let camera = if layer == Layer::BillBoard { self.billboard_camera_matrix } else { self.camera_matrix };
        let matrix = vecmath::col_mat4_mul(camera, trans);

        let color = color.into_vec4(self.graphics.mode, &self.graphics.colors);
        let color = (color[0], color[1], color[2] , color[3]);

        let behavior = glium::uniforms::SamplerBehavior {
            magnify_filter: glium::uniforms::MagnifySamplerFilter::Linear,
            minify_filter: glium::uniforms::MinifySamplerFilter::Linear,
            .. Default::default()
        };
        glium_text::draw_with_params(
            text,
            &self.graphics.text_system,
            &mut self.frame,
            matrix,
            color,
            behavior,
            self.graphics.draw_parameters.clone(),
            ).unwrap();
    }

    pub fn draw_line(&mut self, x: f32, y: f32, angle: f32, length: f32, width: f32, layer: Layer, color: Color) {
        let l2 = length/2.;
        let w2 = width/2.;
        let cosa = angle.cos();
        let sina = angle.sin();
        let cx = x +l2*cosa;
        let cy = y +l2*sina;

        let trans = [
            [l2*cosa, -w2*sina, cx],
            [l2*sina,  w2*cosa, cy]
        ];

        self.draw_quad(trans, layer, color);
    }

    #[inline]
    pub fn finish(self) -> Result<(), SwapBuffersError> {
        self.frame.finish()
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
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

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum Layer {
    Floor,
    Middle,
    Ceil,
    BillBoard,
}

impl Into<f32> for Layer {
    fn into(self) -> f32 {
        match self {
            Layer::Floor => 0.1,
            Layer::Middle => 0.2,
            Layer::Ceil => 0.3,
            Layer::BillBoard => 1.0,
        }
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum Mode {
    Light,
    Dark,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
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
