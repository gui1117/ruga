use glium::{
    Frame,
    Surface,
    VertexBuffer,
    index,
    Program,
    uniforms,
};
use glium::backend::glutin_backend::GlutinFacade;

pub mod color {
    pub const BLACK: [f64;4] = [0.,0.,0.,1.];
    pub const RED:   [f64;4] = [1.,0.,0.,1.];
    pub const BLUE:  [f64;4] = [0.,0.,1.,1.];
    pub const GREEN: [f64;4] = [0.,1.,0.,1.];
}

#[derive(Clone,Copy)]
struct Vertex {
    position: [f64;2],
}

implement_vertex!(Vertex, position);

pub struct Assets {
    square_vertex_buffer: VertexBuffer<Vertex>,
    square_indices: index::NoIndices,
    program: Program,
}

impl Assets {
    pub fn new(facade: &GlutinFacade) -> Assets {
        let square_vertex = vec![
            Vertex { position: [-0.5, -0.5] },
            Vertex { position: [ 0.5, -0.5] },
            Vertex { position: [-0.5,  0.5] },
            Vertex { position: [ 0.5,  0.5] }];

        let square_vertex_buffer = VertexBuffer::new(facade, &square_vertex).unwrap();
        let square_indices = index::NoIndices(index::PrimitiveType::TriangleStrip);

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
            program: program,
        }
    }
}

pub struct FrameManager<'l> {
    frame: Frame,
    ext_dt: f64,
    x: f64,
    y: f64,
    zoom: f64,
    assets: &'l Assets,
    camera: [[f32;4];4],
}

impl<'l> FrameManager<'l> {
    pub fn new(assets: &'l Assets, frame: Frame, ext_dt: f64, x: f64, y: f64, zoom: f64) -> FrameManager<'l> {
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
            ext_dt: ext_dt,
            x: x,
            y: y,
            zoom: zoom,
            assets: assets,
            camera: camera,
        }
    }

    pub fn draw_square(&mut self, color: [f64;4], x: f64, y: f64, width: f64, height: f64) {
        let trans = {
            let kx = width as f32;
            let ky = height as f32;
            let dx = x as f32;
            let dy = y as f32;
            [
                [   kx,    0., 0., 0.],
                [   0.,    ky, 0., 0.],
                [   0.,    0., 1., 0.],
                [dx, dy, 0., 1.]
            ]
        };
        let uniform = uniform!{
            trans: trans,
            camera: self.camera,
            color: [1.,0.,0.,1.0f32],
        };
        self.frame.draw(
            &self.assets.square_vertex_buffer, 
            &self.assets.square_indices, 
            &self.assets.program, 
            &uniform,
            &Default::default()).unwrap();
    }

    pub fn draw_line(&mut self, color: [f64;4], x: f64, y: f64, angle: f64, length: f64) {
    }

    pub fn clear(&mut self) {
        self.frame.clear_color_and_depth((0.0, 0.0, 0.0, 1.0), 1.0);
    }

    pub fn finish(self) {
        self.frame.finish().unwrap();
    }
}
