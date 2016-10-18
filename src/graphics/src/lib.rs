extern crate vecmath;
extern crate toml;
// extern crate libc;
extern crate rusttype;
extern crate unicode_normalization;
extern crate itertools;
extern crate arrayvec;
#[macro_use] extern crate glium;
#[macro_use] extern crate configuration;

// mod glium_text;

use itertools::Itertools;
use glium::{
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
use glium::texture::Texture2d;
use rusttype::{
    FontCollection,
    Font,
    Scale,
    point,
    vector,
    Rect,
};
use rusttype::gpu_cache::Cache;

use std::rc::Rc;
use std::borrow::Cow;
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
        [[1.,0.,0.],
        [0.,1.,0.]]
    }
}

#[derive(Debug,Clone)]
pub struct GraphicsSetting {
    pub colors: ColorsValue,
    pub mode: Mode,
    pub luminosity: f32,
    pub circle_precision: usize,
    pub billboard_font_scale: f32,
    pub font: String,
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

    colors: ColorsValue,
    colors_setting: ColorsValue,
    mode: Mode,
    quad_vertex_buffer: VertexBuffer<Vertex>,
    quad_indices: index::NoIndices,
    circle_vertex_buffer: VertexBuffer<Vertex>,
    circle_indices: index::NoIndices,
    program: Program,
    luminosity: f32,

    billboard_font_scale: Scale,
    font: Font<'static>,
    font_cache: Cache,
    font_cache_tex: Texture2d,
    font_program: Program,

    draw_parameters: DrawParameters<'static>,
}

#[derive(Debug)]
pub enum GraphicsCreationError {
    ProgramCreationError(ProgramCreationError),
    BufferCreationError(BufferCreationError),
    FontFileOpenError(std::io::Error),
    FontFileReadError(std::io::Error),
    FontTextureCreationError,
    InvalidFont,
}

impl Error for GraphicsCreationError {
    fn description(&self) -> &str {
        use self::GraphicsCreationError::*;
        match *self {
            ProgramCreationError(_) => "program creation failed",
            BufferCreationError(_) => "buffer creation failed",
            FontFileOpenError(_) => "open font file failed",
            FontFileReadError(_) => "an error occured while reading the font file",
            InvalidFont => "font not supported",
            FontTextureCreationError => "font texture creation failed",
        }
    }
    fn cause(&self) -> Option<&Error> {
        use self::GraphicsCreationError::*;
        match *self {
            ProgramCreationError(ref e) => e.cause(),
            BufferCreationError(ref e) => e.cause(),
            FontFileOpenError(ref e) => e.cause(),
            FontFileReadError(ref e) => e.cause(),
            InvalidFont => None,
            FontTextureCreationError => None,
        }
    }
}

impl fmt::Display for GraphicsCreationError {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        use self::GraphicsCreationError::*;
        match *self {
            ProgramCreationError(ref e) => write!(fmt,"{}: {}",self.description(),e),
            BufferCreationError(ref e) => write!(fmt,"{}: {}",self.description(),e),
            FontFileOpenError(ref e) => write!(fmt,"{}: {}",self.description(),e),
            FontFileReadError(ref e) => write!(fmt,"{}: {}",self.description(),e),
            InvalidFont => write!(fmt,"{}",self.description()),
            FontTextureCreationError => write!(fmt,"{}",self.description()),
        }
    }
}
impl Graphics {
    pub fn new<F: Facade>(facade: &F, setting: GraphicsSetting) -> Result<Graphics,GraphicsCreationError> {
        use std::io::Read;

        let quad_vertex = vec![
            Vertex { position: [-1., -1.] },
            Vertex { position: [ 1., -1.] },
            Vertex { position: [-1.,  1.] },
            Vertex { position: [ 1.,  1.] }
        ];
        let quad_vertex_buffer = try!(VertexBuffer::new(facade, &quad_vertex)
            .map_err(|bce| GraphicsCreationError::BufferCreationError(bce)));

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
            .map_err(|bce| GraphicsCreationError::BufferCreationError(bce)));

        let circle_indices = index::NoIndices(index::PrimitiveType::TriangleFan);

        let vertex_shader_src = r#"
            #version 130
            in vec2 position;
            uniform mat4 trans;
            uniform mat4 camera;
            void main() {
                mat4 matrix = camera * trans;
                gl_Position = matrix * vec4(position, 0.0, 1.0);
            }
        "#;
        let fragment_shader_src = r#"
            #version 130
            out vec4 out_color;
            uniform vec4 color;
            void main() {
                out_color = color;
            }
        "#;
        let program = try!(Program::from_source(facade, vertex_shader_src, fragment_shader_src, None)
            .map_err(|pce| GraphicsCreationError::ProgramCreationError(pce)));

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

        let mut font_file = try!(std::fs::File::open(&std::path::Path::new(&setting.font)).map_err(|ioe| GraphicsCreationError::FontFileOpenError(ioe)));
        let mut font_data = vec!();
        try!(font_file.read_to_end(&mut font_data).map_err(|e| GraphicsCreationError::FontFileReadError(e)));
        let font = try!(FontCollection::from_bytes(font_data).into_font().ok_or(GraphicsCreationError::InvalidFont));

        let dpi_factor = 1; // FIXME: different from one in retina display
        let (screen_width, screen_height) = facade.get_context().get_framebuffer_dimensions();
        let (cache_width, cache_height) = (screen_width * dpi_factor, screen_height * dpi_factor);

        let font_cache = Cache::new(cache_width, cache_height, 0.1, 0.1);

        //TODO maybe 130 ?
        let font_program = program!(
            facade,
            140 => {
            vertex: "
                #version 140
                uniform float z;
                in vec2 position;
                in vec2 tex_coords;
                out vec2 v_tex_coords;

                void main() {
                    gl_Position = vec4(position, z, 1.0);
                    v_tex_coords = tex_coords;
                }
            ",

            fragment: "
                #version 140
                uniform sampler2D tex;
                uniform vec4 color;
                in vec2 v_tex_coords;
                out vec4 f_colour;

                void main() {
                    f_colour = color * vec4(1.0, 1.0, 1.0, texture(tex, v_tex_coords).r);
                }
            "
            }).unwrap();

        let font_cache_tex = glium::texture::Texture2d::with_format(
            facade,
            glium::texture::RawImage2d {
                data: Cow::Owned(vec![128u8; cache_width as usize * cache_height as usize]),
                width: cache_width,
                height: cache_height,
                format: glium::texture::ClientFormat::U8
            },
            glium::texture::UncompressedFloatFormat::U8,
            glium::texture::MipmapsOption::NoMipmap).unwrap();

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

            billboard_font_scale: Scale::uniform(setting.billboard_font_scale * screen_height as f32),
            font_cache: font_cache,
            font: font,
            font_cache_tex: font_cache_tex,
            font_program: font_program,

            draw_parameters: draw_parameters,
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

pub struct Frame<'a> {
    frame: glium::Frame,
    graphics: &'a mut  Graphics,
    camera: [[f32;4];4],
    billboard_camera: [[f32;4];4],
}

#[derive(Clone,Debug)]
pub struct Camera {
    pub x: f32,
    pub y: f32,
    pub zoom: f32,
    pub ratio: f32,
}

impl Camera {
    pub fn new<F: Facade>(facade: &F, zoom: f32) -> Self {
        let (width,height) = facade.get_context().get_framebuffer_dimensions();

        Camera {
            x: 0.,
            y: 0.,
            zoom: zoom,
            ratio: width as f32/ height as f32,
        }
    }
}

impl<'a> Frame<'a> {
    pub fn new(graphics: &'a mut Graphics, mut frame: glium::Frame, camera: &Camera) -> Frame<'a> {
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
        let billboard_camera = {
            let kx = 1.0;
            let ky = camera.ratio;
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
                trans[3][2] = 0.001;
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
            billboard_camera: billboard_camera,
            camera: camera_matrix,
            frame: frame,
            graphics: graphics,
        }
    }

    pub fn draw_square(&mut self, x: f32, y: f32, radius: f32, layer: Layer, color: Color) {
        self.draw_rectangle(x,y,radius*2.,radius*2.,layer,color);
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
            camera: if layer == Layer::BillBoard { self.billboard_camera } else { self.camera },
            color: color.into_vec4(self.graphics.mode,&self.graphics.colors),
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
            camera: if layer == Layer::BillBoard { self.billboard_camera } else { self.camera },
            color: color.into_vec4(self.graphics.mode,&self.graphics.colors),
        };

        self.frame.draw(
            &self.graphics.circle_vertex_buffer,
            &self.graphics.circle_indices,
            &self.graphics.program,
            &uniform,
            &self.graphics.draw_parameters).unwrap();
    }

    pub fn draw_billbaord_centered_text(&mut self, text: &str, color: Color) {
        let glyphs = {
            use unicode_normalization::UnicodeNormalization;

            let mut lines = vec!();
            let mut current_line = vec!();
            for chr in text.nfc() {
                if let '\n' = chr {
                    lines.push(current_line.drain(..).collect());
                } else if let Some(glyph) = self.graphics.font.glyph(chr) {
                    current_line.push(glyph.scaled(self.graphics.billboard_font_scale));
                }
            };
            lines.push(current_line);

            let v_metrics = self.graphics.font.v_metrics(self.graphics.billboard_font_scale);
            let advance_height = v_metrics.ascent - v_metrics.descent + v_metrics.line_gap;

            let nbr_of_lines = lines.len();
            let glyphs = lines.drain(..).enumerate().flat_map(|(id, mut line)| {
                if line.len() == 0 { return vec!() };

                let height = if nbr_of_lines % 2 == 0 {
                    (((nbr_of_lines/2) as i32 - id as i32) as f32 - 0.5) * advance_height
                } else {
                    ((nbr_of_lines/2) as i32 - id as i32) as f32 * advance_height
                };

                let first_width = line.first().unwrap().h_metrics().advance_width;
                let total_width = line.iter().tuple_windows().fold(first_width, |mut sum, (a, b)| {
                    sum += self.graphics.font.pair_kerning(self.graphics.billboard_font_scale, a.id(), b.id());
                    sum += b.h_metrics().advance_width;

                    sum
                });

                let mut caret = point(-total_width / 2., height);
                let mut last_glyph_id = None;

                line.drain(..).map(|glyph| {
                    if let Some(id) = last_glyph_id.take() {
                        caret.x += self.graphics.font.pair_kerning(self.graphics.billboard_font_scale, id, glyph.id());
                    }
                    last_glyph_id = Some(glyph.id());
                    let glyph = glyph.positioned(caret);
                    caret.x += glyph.unpositioned().h_metrics().advance_width;
                    glyph
                }).collect::<Vec<_>>()
            }).collect::<Vec<_>>();

            glyphs
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

        let z: f32 = Layer::BillBoard.into();
        let uniforms = uniform! {
            tex: self.graphics.font_cache_tex.sampled().magnify_filter(glium::uniforms::MagnifySamplerFilter::Nearest),
            color: color.into_vec4(self.graphics.mode,&self.graphics.colors), //TODO luminosity ?
            z: z,
        };

        let vertex_buffer = {
            let origin = point(1.0, -1.0);
            let (screen_width, screen_height) = {
                let (w,h) = self.graphics.context.get_framebuffer_dimensions();
                (w as f32, h as f32)
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

    /// (x,y) correspond to the down-left anchor
    pub fn draw_text(&mut self, x: f32, y: f32, text: &str, layer: Layer, color: Color) {
        // TODO
        // use std::io::Write;

        // let color = {
        //     let c = color.into_vec4(self.graphics.mode,&self.graphics.colors);
        //     (c[0],c[2],c[2],c[3])
        // };

        // let mut index = 0;
        // let mut remain = text;
        // while remain.len() > 0 {
        //     if index == lines.len() {
        //         writeln!(&mut std::io::stderr(), "draw_text: text doesn't fit in lines: \t\"{}\"",text).unwrap();
        //         break;
        //     }

        //     let split = if lines[index].length*2 > remain.len() {
        //         remain.len()
        //     } else {
        //         lines[index].length*2
        //     };
        //     let (burn,not_burn) = remain.split_at(split);
        //     remain = not_burn;

        //     let text_display = glium_text::TextDisplay::new(&self.graphics.text_system, &self.graphics.font, burn);

        //     let dx = lines[index].x as f32;
        //     let dy = lines[index].y as f32 + 0.2;
        //     let dz = layer.into();
        //     let ratio = self.graphics.font_ratio;
        //     let trans = [
        //         [ 0.5,    0., 0., 0.],
        //         [ 0., ratio/2., 0., 0.],
        //         [ 0.,    0., 1., 0.],
        //         [ dx,    dy, dz, 1.]
        //     ];

        //     let matrix = vecmath::row_mat4_mul(trans,self.camera);
        //     glium_text::draw(&text_display, &self.graphics.text_system, &mut self.frame, matrix, color);
        //     index += 1;
        // }
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
            camera: if layer == Layer::BillBoard { self.billboard_camera } else { self.camera },
            color: color.into_vec4(self.graphics.mode,&self.graphics.colors),
        };

        self.frame.draw(
            &self.graphics.quad_vertex_buffer,
            &self.graphics.quad_indices,
            &self.graphics.program,
            &uniform,
            &self.graphics.draw_parameters).unwrap();
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

        self.draw_quad(trans,layer,color);
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

impl_from_into_toml_for_enum!{
    Color {
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
}

impl Color {
    pub fn from_str(s: &str) -> Self {
        match s {
            "base5" => Color::Base5,
            "base4" => Color::Base4,
            "base3" => Color::Base3,
            "base2" => Color::Base2,
            "base1" => Color::Base1,
            "yellow" => Color::Yellow,
            "orange" => Color::Orange,
            "red" => Color::Red,
            "magenta" => Color::Magenta,
            "violet" => Color::Violet,
            "blue" => Color::Blue,
            "cyan" => Color::Cyan,
            "green" => Color::Green,
            _ => unreachable!(),
        }
    }
    pub fn from_string(s: &String) -> Self {
        match &**s {
            "base5" => Color::Base5,
            "base4" => Color::Base4,
            "base3" => Color::Base3,
            "base2" => Color::Base2,
            "base1" => Color::Base1,
            "yellow" => Color::Yellow,
            "orange" => Color::Orange,
            "red" => Color::Red,
            "magenta" => Color::Magenta,
            "violet" => Color::Violet,
            "blue" => Color::Blue,
            "cyan" => Color::Cyan,
            "green" => Color::Green,
            _ => unreachable!(),
        }
    }
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

#[derive(Debug,Clone,Copy,PartialEq)]
pub enum Layer {
    Floor,
    Middle,
    Ceil,
    BillBoard,
}

impl_from_into_toml_for_enum!{
    Layer {
        Floor,
        Middle,
        Ceil,
        BillBoard,
    }
}

impl Layer {
    pub fn from_str(s: &str) -> Self {
        match s {
            "floor" => Layer::Floor,
            "middle" => Layer::Middle,
            "ceil" => Layer::Ceil,
            "billboard" => Layer::BillBoard,
            _ => unreachable!(),
        }
    }
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

    let display = glium::glutin::WindowBuilder::new()
        .with_dimensions(640,480)
        .build_glium()
        .unwrap();

    let colors_value = ColorsValue {
        base03: [ 0., 0.16862746, 0.21176471, 1. ],
        base02: [ 0.02745098, 0.21176471, 0.25882354, 1. ],
        base01: [ 0.34509805, 0.43137255, 0.45882353, 1. ],
        base00: [ 0.39607844, 0.48235294, 0.5137255, 1. ],
        base0: [ 0.5137255, 0.5803922, 0.5882353, 1. ],
        base1: [ 0.5764706, 0.6313726, 0.6313726, 1. ],
        base2: [ 0.93333334, 0.9098039, 0.8352941, 1. ],
        base3: [ 0.99215686, 0.9647059, 0.8901961, 1. ],
        yellow: [ 0.70980394, 0.5372549, 0., 1. ],
        orange: [ 0.79607844, 0.29411766, 0.08627451, 1. ],
        red: [ 0.8627451, 0.19607843, 0.18431373, 1. ],
        magenta: [ 0.827451, 0.21176471, 0.50980395, 1. ],
        violet: [ 0.42352942, 0.44313726, 0.76862746, 1. ],
        blue: [ 0.14901961, 0.54509807, 0.8235294, 1. ],
        cyan: [ 0.16470589, 0.6313726, 0.59607846, 1. ],
        green: [ 0.52156866, 0.6, 0., 1. ],
    };

    let setting = GraphicsSetting {
        colors: colors_value,
        mode: Mode::Dark,
        luminosity: 0.5,
        circle_precision: 32,
        font_precision: 24,
        font_file: "assets/DejaVuSansMono-Bold.ttf".into(),
        font_ratio: 1.5,
        billboard_font_length: 0.1,
        billboard_font_interline: 1.4,
    };

    let graphics = Graphics::new(&display,setting).unwrap();

    let camera = Camera {
        x: 0.,
        y: 0.,
        zoom: 0.05,
        ratio: 1.33,
    };

    let trans = Transformation::identity().scale(0.1,0.5).translate(0.2,0.9);

    for _ in 0..10 {
        let mut target = Frame::new(&graphics,display.draw(),&camera);
        target.draw_rectangle(0.,0.,1.,1.,Layer::Floor,Color::Base2);
        target.draw_circle(0.,0.,10.,Layer::Middle,Color::Base3);
        target.draw_rectangle(0.,0.,1.1,0.4,Layer::BillBoard,Color::Yellow);
        target.draw_quad(trans,Layer::Ceil,Color::Base4);
        target.draw_line(0.,0.,1.,10.,0.1,Layer::Ceil,Color::Base5);
        target.draw_line(1.,1.,1.,10.,0.1,Layer::Ceil,Color::Base5);
        target.draw_billboard_centered_text("_\n_\ntoto\nest\na\nla\nplage",Color::Green);
        target.finish().unwrap();
        for ev in display.poll_events() {
            match ev {
                glium::glutin::Event::Closed => return,
                _ => ()
            }
        }
        std::thread::sleep(std::time::Duration::from_millis(200));
    }
}
