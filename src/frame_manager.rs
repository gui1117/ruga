//! inspired by m6502 Xscreensaver

// palette :
// Black ($0) "#000000",
// White ($1) "#ffffff",
// Red ($2) "#880000",
// Cyan ($3) "#aaffee",
// Purple ($4) "#cc44cc",
// Green ($5) "#00cc55",
// Blue ($6) "#0000aa",
// Yellow ($7) "#eeee77",
// Orange ($8) "#dd8855",
// Brown ($9) "#664400",
// Light red ($a) "#ff7777",
// Dark gray ($b) "#333333",
// Gray ($c) "#777777",
// Light green ($d) "#aaff66",
// Light blue ($e) "#0088ff",
// Light gray ($f) "#bbbbbb"

use glium::{
    Frame,
    Surface,
    VertexBuffer,
    index,
    Program,
};
use glium::backend::glutin_backend::GlutinFacade;
use image;
use std::io::Cursor;

#[derive(Clone,Copy,Eq,PartialEq)]
pub enum Animation {
    Boid,
    CharacterRifle,
    CharacterSniper,
    CharacterShotgun,
    Spider,
    BurningWall,
    Wall,
}

pub mod color {
    pub const BLACK:      [f32;4] = [0.00,0.00,0.00,0.50];
    pub const WHITE:      [f32;4] = [1.00,1.00,1.00,0.50];
    pub const RED:        [f32;4] = [0.50,0.00,0.00,0.50];
    pub const CYAN:       [f32;4] = [0.66,1.00,0.93,0.50];
    pub const PURPLE:     [f32;4] = [0.80,0.27,0.80,0.50];
    pub const GREEN:      [f32;4] = [0.00,0.80,0.33,0.50];
    pub const BLUE:       [f32;4] = [0.00,0.80,0.66,0.50];
    pub const YELLOW:     [f32;4] = [0.93,0.93,0.46,0.50];
    pub const ORANGE:     [f32;4] = [0.86,0.53,0.33,0.50];
    pub const BROWN :     [f32;4] = [0.40,0.27,0.00,0.50];
    pub const LIGHT_RED:  [f32;4] = [1.00,0.46,0.46,0.50];
    pub const DARK_GRAY:  [f32;4] = [0.20,0.20,0.20,0.50];
    pub const GRAY:       [f32;4] = [0.46,0.46,0.46,0.50];
    pub const LIGHT_GREEN:[f32;4] = [0.66,1.00,0.40,0.50];
    pub const LIGHT_BLUE: [f32;4] = [0.00,0.53,1.00,0.50];
    pub const LIGHT_GRAY: [f32;4] = [0.73,0.73,0.73,0.50];
}

#[derive(Clone,Copy)]
struct Vertex {
    position: [f64;2],
}

implement_vertex!(Vertex, position);

pub struct Assets {
    square_vertex_buffer: VertexBuffer<Vertex>,
    square_indices: index::NoIndices,
    line_vertex_buffer: VertexBuffer<Vertex>,
    line_indices: index::NoIndices,
    tileset: glium::texture::RawImage2d,
    program: Program,
}

impl Assets {
    pub fn new(facade: &GlutinFacade) -> Assets {
        let tileset = image::load(Cursor::new(&include_bytes!("assets/graphics/tileset.png")[..]),
                                image::PNG).unwrap().to_rgba();
        let tileset_dimensions = tileset.dimensions();
        let tileset = glium::texture::RawImage2d::from_raw_rgba_reversed(tileset.into_raw(), tileset_dimensions);

        let square_vertex = vec![
            Vertex { position: [-0.5, -0.5] },
            Vertex { position: [ 0.5, -0.5] },
            Vertex { position: [-0.5,  0.5] },
            Vertex { position: [ 0.5,  0.5] }
        ];

        let square_vertex_buffer = VertexBuffer::new(facade, &square_vertex).unwrap();
        let square_indices = index::NoIndices(index::PrimitiveType::TriangleStrip);

        let line_vertex = vec![
            Vertex { position: [0., 0.] },
            Vertex { position: [ 1., 1.] }
        ];

        let line_vertex_buffer = VertexBuffer::new(facade, &line_vertex).unwrap();
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


        let program = Program::from_source(facade, vertex_shader_src, fragment_shader_src, None).unwrap();

        Assets {
            square_vertex_buffer: square_vertex_buffer,
            square_indices: square_indices,
            line_vertex_buffer: line_vertex_buffer,
            line_indices: line_indices,
            program: program,
        }
    }
}

pub struct FrameManager<'l> {
    frame: Frame,
    // ext_dt: f64,
    // x: f64,
    // y: f64,
    // zoom: f64,
    assets: &'l Assets,
    camera: [[f32;4];4],
}

impl<'l> FrameManager<'l> {
    pub fn new(assets: &'l Assets, frame: Frame, _ext_dt: f64, x: f64, y: f64, zoom: f64) -> FrameManager<'l> {
        let camera = {
            let k = zoom as f32;
            let dx = -x as f32;
            let dy = -y as f32;
            [
                [   k,   0., 0., 0.],
                [  0.,    k, 0., 0.],
                [  0.,   0.,  k, 0.],
                [k*dx, k*dy, 0., 1.]
            ]
        };

        FrameManager {
            frame: frame,
            // ext_dt: ext_dt,
            // x: x,
            // y: y,
            // zoom: zoom,
            assets: assets,
            camera: camera,
        }
    }

    pub fn draw_animation(&mut self, animation: Animation) {
    }

    pub fn draw_square(&mut self, color: [f32;4], x: f64, y: f64, width: f64, height: f64) {
        let trans = {
            let kx = width as f32;
            let ky = height as f32;
            let dx = x as f32;
            let dy = y as f32;
            [
                [ kx, 0., 0., 0.],
                [ 0., ky, 0., 0.],
                [ 0., 0., 1., 0.],
                [ dx, dy, 0., 1.]
            ]
        };
        let uniform = uniform!{
            trans: trans,
            camera: self.camera,
            color: color,
        };
        self.frame.draw(
            &self.assets.square_vertex_buffer,
            &self.assets.square_indices,
            &self.assets.program,
            &uniform,
            &Default::default()).unwrap();
    }

    pub fn draw_line(&mut self, color: [f32;4], x: f64, y: f64, angle: f64, length: f64) {
        let trans = {
            let kx = (length*angle.cos()) as f32;
            let ky = (length*angle.sin()) as f32;
            let dx = x as f32;
            let dy = y as f32;
            [
                [ kx, 0., 0., 0.],
                [ 0., ky, 0., 0.],
                [ 0., 0., 1., 0.],
                [ dx, dy, 0., 1.]
            ]
        };
        let uniform = uniform!{
            trans: trans,
            camera: self.camera,
            color: color,
        };
        self.frame.draw(
            &self.assets.line_vertex_buffer,
            &self.assets.line_indices,
            &self.assets.program,
            &uniform,
            &Default::default()).unwrap();
    }

    pub fn clear(&mut self) {
        self.frame.clear_color_and_depth((0.0, 0.0, 0.0, 1.0), 1.0);
    }

    pub fn finish(self) {
        self.frame.finish().unwrap();
    }
}
