extern crate glium;
extern crate graphics;

fn main() {
    use glium::DisplayBuild;

    let window = glium::glutin::WindowBuilder::new()
            .with_vsync()
            .with_multisampling(2)
            .with_dimensions(640,480)
            .build_glium()
            .unwrap();

    let mut graphics = graphics::Graphics::new(&window, graphics::GraphicsSetting {
        colors: graphics::ColorsValue {
            base03: [0.0, 0.0, 0.0, 1.0],
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
        font_size: 40,
        font: "assets/DejaVuSansMono-Bold.ttf".into(),
    }).unwrap();

    let camera = graphics::Camera::new(0.0,0.0,0.08);
    let text = graphics.new_text_display("toto");

    loop {
        let mut frame = graphics::Frame::new(&mut graphics, window.draw(), &camera);
        frame.draw_text(&text, 0., 0., 1., graphics::Layer::BillBoard, graphics::Color::Red);
        // frame.draw_rectangle(-11.0, 2.0, 0.4, 0.4, graphics::Layer::Floor, graphics::Color::Red);
        frame.draw_rectangle(0., 0., 0.4, 0.4, graphics::Layer::BillBoard, graphics::Color::Red);
        // frame.draw_text(-11.0, 2.0, 0.4, "Un pur esprit s'accroît sous l'écorce des pierres !", graphics::Layer::Floor, graphics::Color::Base5);
        frame.finish().unwrap();

        for ev in window.poll_events() {
            match ev {
                glium::glutin::Event::Closed => return,
                _ => ()
            }
        }
    }
}
