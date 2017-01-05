pub mod resources {
    impl_resource! {
        Notifications,
    }

    pub struct Notifications(pub Vec<(String, usize)>);
    impl Notifications {
        pub fn new() -> Self {
            Notifications(Vec::new())
        }
    }
}

pub mod draw_systems {
    use graphics::{self, Layer};
    use specs;
    use colors;

    use super::resources::*;

    const NOTIFICATION_DL: f32 = 0.03;
    const NOTIFICATION_SCALE: f32 = 0.02;
    const NOTIFICATION_SMALL_MARGIN: f32 = 0.005;
    const NOTIFICATION_BIG_MARGIN: f32 = 0.01;

    pub fn draw_notifications(world: &mut specs::World, frame: &mut graphics::Frame) {
        let mut notifications = world.write_resource::<Notifications>();

        let (mut x, mut y) = frame.get_down_left_billboard_anchor();
        x += NOTIFICATION_DL;
        y += NOTIFICATION_DL;

        for &mut (ref notification, ref mut count) in notifications.0.iter_mut().rev() {
            *count -= 1;
            let (width, height) = frame.get_size(NOTIFICATION_SCALE, &*notification);
            {
                let w = width + NOTIFICATION_BIG_MARGIN * 2.0;
                let h = height + NOTIFICATION_BIG_MARGIN * 2.0;
                let x = x + w / 2.0;
                let y = y + h / 2.0;
                frame.draw_rectangle(x, y, w, h, Layer::UnderBillboard, colors::BLACK);
            }
            {
                let w = width + NOTIFICATION_SMALL_MARGIN * 2.0;
                let h = height + NOTIFICATION_SMALL_MARGIN * 2.0;
                let x = x + w / 2.0 + NOTIFICATION_BIG_MARGIN / 2.0;
                let y = y + h / 2.0 + NOTIFICATION_BIG_MARGIN / 2.0;
                frame.draw_rectangle(x, y, w, h, Layer::UnderBillboard, colors::WHITE);
            }
            frame.draw_text(x + NOTIFICATION_BIG_MARGIN,
                            y + NOTIFICATION_BIG_MARGIN / 2.,
                            NOTIFICATION_SCALE,
                            notification,
                            Layer::UnderBillboard,
                            colors::BLACK);
            y += height + NOTIFICATION_BIG_MARGIN * 2.0;
        }

        notifications.0.retain(|&(_, count)| count > 0)
    }

}
