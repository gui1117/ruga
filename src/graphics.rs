use arrayvec;
use vecmath;
use glium::{
    self,
    Blend,
    SwapBuffersError,
    Surface,
    VertexBuffer,
    index,
    Program,
    DrawParameters,
    Depth,
    DepthTest,
};
use glium::backend::{Facade, Context};
use glium::program::ProgramCreationError;
use glium::vertex::BufferCreationError;
use glium::draw_parameters::Smooth;
use glium::texture::{Texture2d, TextureCreationError};
use rusttype::{
    SharedBytes,
    FontCollection,
    Font,
    Scale,
    point,
    vector,
    Vector,
    Rect,
};
use rusttype::gpu_cache::Cache;

use std::error::Error;
use std::fmt;
use std::io::{self, Read};
use std::rc::Rc;
use std::borrow::Cow;
use std::f32::consts::PI;

// GLSL version is 150 even if 130 would be OK
// but macos seem to have issue with 130 so ...

pub type Transformation = vecmath::Matrix2x3<f32>;

const CIRCLE_PRECISION: usize = 64;
const BEZIER_PRECISION: usize = 20;

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

#[derive(Clone,Copy)]
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

pub struct Graphics {
    context: Rc<Context>,

    quad_vertex_buffer: VertexBuffer<Vertex>,
    quad_indices: index::NoIndices,
    circle_vertex_buffer: VertexBuffer<Vertex>,
    circle_indices: index::NoIndices,
    program: Program,

    line_indices: index::NoIndices,
    line_program: Program,

    font: Font<'static>,
    font_cache: Cache,
    font_cache_tex: Texture2d,
    font_program: Program,

    draw_parameters: DrawParameters<'static>,
}

#[derive(Debug)]
pub enum GraphicsError {
    ProgramCreation(ProgramCreationError),
    BufferCreation(BufferCreationError),
    Io(io::Error),
    TextureCreation(TextureCreationError),
    InvalidFont,
}

impl Error for GraphicsError {
    fn description(&self) -> &str {
        use self::GraphicsError::*;
        match *self {
            ProgramCreation(ref err) => err.description(),
            BufferCreation(ref err) => err.description(),
            Io(ref err) => err.description(),
            TextureCreation(ref err) => err.description(),
            InvalidFont => "font not supported",
        }
    }
    fn cause(&self) -> Option<&Error> {
        use self::GraphicsError::*;
        match *self {
            ProgramCreation(ref e) => e.cause(),
            BufferCreation(ref e) => e.cause(),
            Io(ref e) => e.cause(),
            TextureCreation(ref e) => e.cause(),
            InvalidFont => None,
        }
    }
}
impl fmt::Display for GraphicsError {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        use self::GraphicsError::*;
        match *self {
            ProgramCreation(ref e) => write!(fmt,"Glium program creation error: {}", e),
            BufferCreation(ref e) => write!(fmt,"Glium buffer creation error: {}", e),
            Io(ref e) => write!(fmt,"Io error: {}", e),
            TextureCreation(ref e) => write!(fmt,"Glium texture creation error: {}", e),
            InvalidFont => write!(fmt,"Font not supported"),
        }
    }
}
impl From<ProgramCreationError> for GraphicsError {
    fn from(err: ProgramCreationError) -> GraphicsError {
        GraphicsError::ProgramCreation(err)
    }
}
impl From<BufferCreationError> for GraphicsError {
    fn from(err: BufferCreationError) -> GraphicsError {
        GraphicsError::BufferCreation(err)
    }
}
impl From<io::Error> for GraphicsError {
    fn from(err: io::Error) -> GraphicsError {
        GraphicsError::Io(err)
    }
}
impl From<TextureCreationError> for GraphicsError {
    fn from(err: TextureCreationError) -> GraphicsError {
        GraphicsError::TextureCreation(err)
    }
}

impl Graphics {
    pub fn new<F: Facade>(facade: &F) -> Result<Graphics,GraphicsError> {
        let quad_vertex = vec![
            Vertex { position: [-1., -1.] },
            Vertex { position: [ 1., -1.] },
            Vertex { position: [-1.,  1.] },
            Vertex { position: [ 1.,  1.] }
        ];
        let quad_vertex_buffer = try!(VertexBuffer::new(facade, &quad_vertex));

        let quad_indices = index::NoIndices(index::PrimitiveType::TriangleStrip);

        let mut circle_vertex = vec!(Vertex { position: [0., 0.] });
        {
            let delta_angle = PI * 2. / CIRCLE_PRECISION as f32;
            let mut angle = 0f32;
            circle_vertex.push(Vertex { position: [angle.cos(),angle.sin()]});
            for _ in 0..CIRCLE_PRECISION {
                angle += delta_angle;
                circle_vertex.push(Vertex { position: [angle.cos(),angle.sin()]});
            }
        }

        let circle_vertex_buffer = try!(VertexBuffer::new(facade, &circle_vertex));

        let circle_indices = index::NoIndices(index::PrimitiveType::TriangleFan);

        let vertex_shader_src = r#"
            #version 150
            in vec2 position;
            uniform mat4 trans;
            uniform mat4 camera;
            void main() {
                mat4 matrix = camera * trans;
                gl_Position = matrix * vec4(position, 0.0, 1.0);
            }
        "#;
        let fragment_shader_src = r#"
            #version 150
            out vec4 out_color;
            uniform vec4 color;
            void main() {
                out_color = color;
            }
        "#;
        let program = try!(Program::from_source(facade, vertex_shader_src, fragment_shader_src, None));

        let line_indices = index::NoIndices(index::PrimitiveType::TriangleStrip);
        let line_vertex_shader_src = r#"
            #version 150
            in vec2 position;
            uniform float z;
            uniform mat4 camera;
            void main() {
                gl_Position = camera * vec4(position, z, 1.0);
            }
        "#;
        let line_fragment_shader_src = r#"
            #version 150
            out vec4 out_color;
            uniform vec4 color;
            void main() {
                out_color = color;
            }
        "#;
        let line_program = try!(Program::from_source(facade, line_vertex_shader_src, line_fragment_shader_src, None));

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

        let font_data = include_bytes!("DejaVuSansMono-Bold.ttf");
        let font = try!(FontCollection::from_bytes(SharedBytes::ByRef(font_data)).into_font()
                        .ok_or(GraphicsError::InvalidFont));

        let dpi_factor = 1; // FIXME: different from one in retina display
        let (screen_width, screen_height) = facade.get_context().get_framebuffer_dimensions();
        let (cache_width, cache_height) = (screen_width * dpi_factor, screen_height * dpi_factor);

        let font_cache = Cache::new(cache_width, cache_height, 0.1, 0.1);

        let font_vertex_shader_src = r#"
                #version 150
                uniform float z;
                in vec2 position;
                in vec2 tex_coords;
                out vec2 v_tex_coords;

                void main() {
                    gl_Position = vec4(position, z, 1.0);
                    v_tex_coords = tex_coords;
                }
        "#;
        let font_fragment_shader_src = r#"
                #version 150
                uniform sampler2D tex;
                uniform vec4 color;
                in vec2 v_tex_coords;
                out vec4 f_colour;

                void main() {
                    f_colour = color * vec4(1.0, 1.0, 1.0, texture(tex, v_tex_coords).r);
                }
        "#;
        let font_program = try!(Program::from_source(facade, font_vertex_shader_src, font_fragment_shader_src, None));

        let font_cache_tex = try!(glium::texture::Texture2d::with_format(
            facade,
            glium::texture::RawImage2d {
                data: Cow::Owned(vec![128u8; cache_width as usize * cache_height as usize]),
                width: cache_width,
                height: cache_height,
                format: glium::texture::ClientFormat::U8
            },
            glium::texture::UncompressedFloatFormat::U8,
            glium::texture::MipmapsOption::NoMipmap));

        Ok(Graphics {
            context: facade.get_context().clone(),

            quad_vertex_buffer: quad_vertex_buffer,
            quad_indices: quad_indices,
            circle_vertex_buffer: circle_vertex_buffer,
            circle_indices: circle_indices,
            program: program,

            line_indices: line_indices,
            line_program: line_program,

            font_cache: font_cache,
            font: font,
            font_cache_tex: font_cache_tex,
            font_program: font_program,

            draw_parameters: draw_parameters,
        })
    }
    pub fn set_font<R: Read>(&mut self, font: &mut R) -> Result<(), GraphicsError> {
        let mut font_data = vec!();
        try!(font.read_to_end(&mut font_data));
        self.font = try!(FontCollection::from_bytes(font_data).into_font()
                        .ok_or(GraphicsError::InvalidFont));
        Ok(())
    }
    pub fn resize(&mut self) -> Result<(), GraphicsError> {
        let dpi_factor = 1; // FIXME: different from one in retina display
        let (screen_width, screen_height) = self.context.get_framebuffer_dimensions();
        let (cache_width, cache_height) = (screen_width * dpi_factor, screen_height * dpi_factor);

        self.font_cache = Cache::new(cache_width, cache_height, 0.1, 0.1);
        self.font_cache_tex = try!(glium::texture::Texture2d::with_format(
            &self.context,
            glium::texture::RawImage2d {
                data: Cow::Owned(vec![128u8; cache_width as usize * cache_height as usize]),
                width: cache_width,
                height: cache_height,
                format: glium::texture::ClientFormat::U8
            },
            glium::texture::UncompressedFloatFormat::U8,
            glium::texture::MipmapsOption::NoMipmap));

        Ok(())
    }
}

pub struct Frame<'a> {
    frame: glium::Frame,
    graphics: &'a mut  Graphics,
    camera: &'a Camera,
    camera_matrix: [[f32;4];4],
    billboard_camera_matrix: [[f32;4];4],
}

#[derive(Clone,Debug)]
pub struct Camera {
    pub x: f32,
    pub y: f32,
    pub zoom: f32,
}

impl Camera {
    pub fn new(x: f32, y: f32, zoom: f32) -> Self {
        Camera { x: x, y: y, zoom: zoom, }
    }
}

impl<'a> Frame<'a> {
    pub fn new(graphics: &'a mut Graphics, mut frame: glium::Frame, camera: &'a Camera) -> Frame<'a> {
        let (width,height) = graphics.context.get_framebuffer_dimensions();
        let ratio = width as f32/ height as f32;

        let camera_matrix = {
            let kx = camera.zoom;
            let ky = camera.zoom*ratio;
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
            let kx = 1.0;
            let ky = ratio;
            [
                [   kx,    0., 0., 0.],
                [   0.,    ky, 0., 0.],
                [   0.,    0., 1., 0.],
                [   0.,    0., 0., 1.]
            ]
        };

        frame.clear_color_and_depth((1.0, 1.0, 1.0, 1.0), 0f32);;

        Frame {
            billboard_camera_matrix: billboard_camera_matrix,
            camera_matrix: camera_matrix,
            camera: camera,
            frame: frame,
            graphics: graphics,
        }
    }

    pub fn draw_square(&mut self, x: f32, y: f32, radius: f32, layer: Layer, color: [f32;4]) {
        self.draw_rectangle(x,y,radius*2.,radius*2.,layer,color);
    }

    pub fn draw_rectangle(&mut self, x: f32, y: f32, width: f32, height: f32, layer: Layer, color: [f32;4]) {
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
            camera: if layer.billboard() { self.billboard_camera_matrix } else { self.camera_matrix },
            color: color,
        };

        self.frame.draw(
            &self.graphics.quad_vertex_buffer,
            &self.graphics.quad_indices,
            &self.graphics.program,
            &uniform,
            &self.graphics.draw_parameters).unwrap();
    }

    pub fn draw_circle(&mut self, x: f32, y: f32, radius: f32, layer: Layer, color: [f32;4]) {
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
            camera: if layer.billboard() { self.billboard_camera_matrix } else { self.camera_matrix },
            color: color,
        };

        self.frame.draw(
            &self.graphics.circle_vertex_buffer,
            &self.graphics.circle_indices,
            &self.graphics.program,
            &uniform,
            &self.graphics.draw_parameters).unwrap();
    }

    /// (x,y) correspond to the down-left anchor
    pub fn draw_text(&mut self, x: f32, y: f32, scale: f32, text: &str, layer: Layer, color: [f32;4]) {
        let glyphs = {
            use unicode_normalization::UnicodeNormalization;

            let (screen_width,_) = {
                let (w,h) = self.graphics.context.get_framebuffer_dimensions();
                (w as f32, h as f32)
            };

            let scale = if layer.billboard() {
                Scale::uniform(scale * screen_width * 0.5)
            } else {
                Scale::uniform(scale * self.camera.zoom * screen_width * 0.5)
            };

            let metrics = self.graphics.font.v_metrics(scale);
            let mut caret = point(0.0, metrics.descent - metrics.line_gap/2.0);
            let mut last_glyph_id = None;
            let mut res = vec!();

            for chr in text.nfc() {
                if let Some(glyph) = self.graphics.font.glyph(chr) {
                    let glyph = glyph.scaled(scale);
                    if let Some(id) = last_glyph_id.take() {
                        caret.x += self.graphics.font.pair_kerning(scale, id, glyph.id());
                    }
                    last_glyph_id = Some(glyph.id());
                    let glyph = glyph.positioned(caret);
                    caret.x += glyph.unpositioned().h_metrics().advance_width;
                    res.push(glyph);
                }
            };

            res
        };

        for glyph in &glyphs {
            self.graphics.font_cache.queue_glyph(0, glyph.clone());
        }

        {
            let ref mut font_cache_tex = self.graphics.font_cache_tex;
            self.graphics.font_cache.cache_queued(|rect, data| {
                font_cache_tex.main_level().write(glium::Rect {
                    left: rect.min.x,
                    bottom: rect.min.y,
                    width: rect.width(),
                    height: rect.height()
                }, glium::texture::RawImage2d {
                    data: Cow::Borrowed(data),
                    width: rect.width(),
                    height: rect.height(),
                    format: glium::texture::ClientFormat::U8
                });
            }).unwrap();
        }

        let z: f32 = Layer::Billboard.into();
        let uniforms = uniform! {
            tex: self.graphics.font_cache_tex.sampled().magnify_filter(glium::uniforms::MagnifySamplerFilter::Nearest),
            color: color,
            z: z,
        };

        let vertex_buffer = {
            let (screen_width, screen_height) = {
                let (w,h) = self.graphics.context.get_framebuffer_dimensions();
                (w as f32, h as f32)
            };

            let origin = if layer.billboard() {
                let px = 1.0 + x;
                let py = -1.0 + y*screen_width/screen_height;

                let (ppx,ppy) = pixel_perfect((px,py), screen_width, screen_height);
                point(ppx,ppy)
            } else {
                let px = 1.0 + (x - self.camera.x)*self.camera.zoom;
                let py = -1.0 + (y - self.camera.y)*self.camera.zoom*screen_width/screen_height;

                let (ppx,ppy) = pixel_perfect((px,py), screen_width, screen_height);
                point(ppx,ppy)
            };

            let vertices: Vec<FontVertex> = glyphs.iter().flat_map(|g| {
                if let Ok(Some((uv_rect, screen_rect))) = self.graphics.font_cache.rect_for(0, g) {
                    let gl_rect = Rect {
                        min: origin
                            + (vector(screen_rect.min.x as f32 / screen_width - 0.5,
                                      1.0 - screen_rect.min.y as f32 / screen_height - 0.5)) * 2.0,
                        max: origin
                            + (vector(screen_rect.max.x as f32 / screen_width - 0.5,
                                      1.0 - screen_rect.max.y as f32 / screen_height - 0.5)) * 2.0
                    };
                    arrayvec::ArrayVec::<[FontVertex; 6]>::from([
                        FontVertex {
                            position: [gl_rect.min.x, gl_rect.max.y],
                            tex_coords: [uv_rect.min.x, uv_rect.max.y],
                        },
                        FontVertex {
                            position: [gl_rect.min.x,  gl_rect.min.y],
                            tex_coords: [uv_rect.min.x, uv_rect.min.y],
                        },
                        FontVertex {
                            position: [gl_rect.max.x,  gl_rect.min.y],
                            tex_coords: [uv_rect.max.x, uv_rect.min.y],
                        },
                        FontVertex {
                            position: [gl_rect.max.x,  gl_rect.min.y],
                            tex_coords: [uv_rect.max.x, uv_rect.min.y],
                        },
                        FontVertex {
                            position: [gl_rect.max.x, gl_rect.max.y],
                            tex_coords: [uv_rect.max.x, uv_rect.max.y],
                        },
                        FontVertex {
                            position: [gl_rect.min.x, gl_rect.max.y],
                            tex_coords: [uv_rect.min.x, uv_rect.max.y],
                        }])
                } else {
                    arrayvec::ArrayVec::new()
                }
            }).collect();
            glium::VertexBuffer::new(&self.graphics.context, &vertices).unwrap()
        };

        self.frame.draw(&vertex_buffer,
                    &glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList),
                    &self.graphics.font_program,
                    &uniforms,
                    &self.graphics.draw_parameters).unwrap();
    }

    pub fn draw_quad(&mut self, trans: Transformation, layer: Layer, color: [f32;4]) {
        let trans = [
            [ trans[0][0], trans[1][0],           0., 0.],
            [ trans[0][1], trans[1][1],           0., 0.],
            [          0.,          0.,           1., 0.],
            [ trans[0][2], trans[1][2], layer.into(), 1.]
        ];
        let uniform = uniform!{
            trans: trans,
            camera: if layer.billboard() { self.billboard_camera_matrix } else { self.camera_matrix },
            color: color,
        };

        self.frame.draw(
            &self.graphics.quad_vertex_buffer,
            &self.graphics.quad_indices,
            &self.graphics.program,
            &uniform,
            &self.graphics.draw_parameters).unwrap();
    }

    pub fn draw_line(&mut self, p0: (f32,f32), p1: (f32,f32), p2: (f32,f32), p3: (f32,f32), width: f32, layer: Layer, color: [f32;4]) {
        let p0 = Vector { x: p0.0, y: p0.1 };
        let p1 = Vector { x: p1.0, y: p1.1 };
        let p2 = Vector { x: p2.0, y: p2.1 };
        let p3 = Vector { x: p3.0, y: p3.1 };

        let mut vertices: Vec<Vertex> = vec!();

        let dt = 1.0/BEZIER_PRECISION as f32;
        let mut t = 0f32;

        for _ in 0..BEZIER_PRECISION+1 {
            let p = (1.0-t).powi(3)*p0 + 3.0*t*(1.0-t).powi(2)*p1 + 3.0*t.powi(2)*(1.0-t)*p2 + t.powi(3)*p3;
            let n = 3.0*(1.0-t).powi(2)*(p1-p0) + 6.0*t*(1.0-t)*(p2-p1) + 3.0*t.powi(2)*(p3-p2);
            let o = Vector { x: -n.y, y: n.x };
            let o = o/(o.x.powi(2)+o.y.powi(2)).sqrt();

            let a = p + o*width/2.0;
            let b = p - o*width/2.0;

            vertices.push(Vertex { position: [a.x, a.y] });
            vertices.push(Vertex { position: [b.x, b.y] });
            t += dt;
        }

        let z: f32 = Layer::Billboard.into();
        let uniform = uniform!{
            z: z,
            camera: if layer.billboard() { self.billboard_camera_matrix } else { self.camera_matrix },
            color: color,
        };
        let vertex_buffer = glium::VertexBuffer::new(&self.graphics.context, &vertices).unwrap();

        self.frame.draw(
            &vertex_buffer,
            &self.graphics.line_indices,
            &self.graphics.line_program,
            &uniform,
            &self.graphics.draw_parameters).unwrap();
    }

    pub fn get_down_left_billboard_anchor(&self) -> (f32,f32) {
        let (width,height) = self.graphics.context.get_framebuffer_dimensions();
        (-1.0, -(height as f32/ width as f32))
    }

    pub fn get_size(&self, scale: f32, text: &str) -> (f32, f32) {
        use unicode_normalization::UnicodeNormalization;

        let (screen_width, screen_height) = {
            let (w,h) = self.graphics.context.get_framebuffer_dimensions();
            (w as f32, h as f32)
        };

        let scale = Scale::uniform(scale * screen_width * 0.5);

        let mut width = 0.0;

        for chr in text.nfc() {
            if let Some(glyph) = self.graphics.font.glyph(chr) {
                width += glyph.scaled(scale).h_metrics().advance_width;
            }
        };

        let metrics = self.graphics.font.v_metrics(scale);
        let height = metrics.descent + metrics.line_gap + metrics.ascent;

        width *= 2.0;

        (width / screen_width, height / screen_height)
    }

    #[inline]
    pub fn finish(self) -> Result<(), SwapBuffersError> {
        self.frame.finish()
    }
}

fn pixel_perfect(p: (f32,f32), screen_width: f32, screen_height: f32) -> (f32,f32) {
    (
        (p.0*screen_width/2.0).round()/screen_width*2.0,
        (p.1*screen_height/2.0).round()/screen_height*2.0
    )
}

#[derive(Debug,Clone,Copy,PartialEq)]
pub enum Layer {
    #[allow(dead_code)] UnderFloor,
    #[allow(dead_code)] Floor,
    #[allow(dead_code)] AboveFloor,
    #[allow(dead_code)] UnderMiddle,
    #[allow(dead_code)] Middle,
    #[allow(dead_code)] AboveMiddle,
    #[allow(dead_code)] UnderCeil,
    #[allow(dead_code)] Ceil,
    #[allow(dead_code)] AboveCeil,
    #[allow(dead_code)] UnderBillboard,
    #[allow(dead_code)] Billboard,
    #[allow(dead_code)] AboveBillboard,
}

impl Into<f32> for Layer {
    fn into(self) -> f32 {
        match self {
            Layer::UnderFloor => 0.01,
            Layer::Floor => 0.02,
            Layer::AboveFloor => 0.03,
            Layer::UnderMiddle => 0.04,
            Layer::Middle => 0.05,
            Layer::AboveMiddle => 0.06,
            Layer::UnderCeil => 0.07,
            Layer::Ceil => 0.08,
            Layer::AboveCeil => 0.09,
            Layer::UnderBillboard => 0.10,
            Layer::Billboard => 0.11,
            Layer::AboveBillboard => 0.12,
        }
    }
}

impl Layer {
    fn billboard(self) -> bool {
        use self::Layer::*;
        match self {
            Billboard | AboveBillboard | UnderBillboard => true,
            _ => false,
        }
    }
}
