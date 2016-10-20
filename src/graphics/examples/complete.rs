extern crate glium;
extern crate graphics;

fn main() {
    use glium::DisplayBuild;

    let window = glium::glutin::WindowBuilder::new()
            .with_dimensions(640,480)
            .build_glium()
            .unwrap();

    let mut graphics = graphics::Graphics::new(&window, graphics::GraphicsSetting {
        colors: graphics::ColorsValue {
            base03: [1.0, 1.0, 1.0, 1.0],
            base02: [1.0, 1.0, 1.0, 1.0],
            base01: [1.0, 1.0, 1.0, 1.0],
            base00: [1.0, 1.0, 1.0, 1.0],
            base0: [1.0, 1.0, 1.0, 1.0],
            base1: [1.0, 1.0, 1.0, 1.0],
            base2: [1.0, 1.0, 1.0, 1.0],
            base3: [1.0, 1.0, 1.0, 1.0],
            yellow: [1.0, 1.0, 1.0, 1.0],
            orange: [1.0, 1.0, 1.0, 1.0],
            red: [1.0, 0.0, 0.0, 0.5],
            magenta: [1.0, 1.0, 1.0, 1.0],
            violet: [1.0, 1.0, 1.0, 1.0],
            blue: [0.0, 0.0, 1.0, 0.5],
            cyan: [1.0, 1.0, 1.0, 1.0],
            green: [0.0, 1.0, 0.0, 0.5],
        },
        mode: graphics::Mode::Dark,
        circle_precision: 32,
        luminosity: 1.0,
        billboard_font_scale: 0.1,
        font: "assets/DejaVuSansMono-Bold.ttf".into(),
    }).unwrap();

    let mut camera = graphics::Camera::new(0.0,0.0, 0.001);

    let mut _t = 1f32;

    loop {
        _t += 0.05;
        camera.x = 100.0;
        camera.y = 10.0;
        camera.zoom = 0.01;

        let mut frame = graphics::Frame::new(&mut graphics, window.draw(), &camera);
        frame.draw_billboard_centered_text("Aôttttt\np",graphics::Color::Blue);
        // frame.draw_billboard_centered_text("p",graphics::Color::Green);
        // frame.draw_rectangle(110.0, 20.0, 1.0, 1.0, graphics::Layer::Floor, graphics::Color::Red);
        // frame.draw_text(110.0, 20.0, 1.0, "O", graphics::Layer::Floor, graphics::Color::Blue);
        frame.finish().unwrap();

        for ev in window.poll_events() {
            match ev {
                glium::glutin::Event::Closed => return,
                _ => ()
            }
        }
    }
}
