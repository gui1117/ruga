use graphics::{self, Layer};
use specs;
use specs::Join;
use app;
use resources::*;
use components::*;
use colors;

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
            let width = width + NOTIFICATION_BIG_MARGIN*2.0;
            let height = height + NOTIFICATION_BIG_MARGIN*2.0;
            let x = x + width/2.0;
            let y = y + height/2.0;
            frame.draw_rectangle(x, y, width, height, graphics::Layer::UnderBillboard, colors::BASE01);
        }
        {
            let width = width + NOTIFICATION_SMALL_MARGIN*2.0;
            let height = height + NOTIFICATION_SMALL_MARGIN*2.0;
            let x = x + width/2.0 + NOTIFICATION_BIG_MARGIN/2.0;
            let y = y + height/2.0 + NOTIFICATION_BIG_MARGIN/2.0;
            frame.draw_rectangle(x, y, width, height, graphics::Layer::Billboard, colors::BASE2);
        }
        frame.draw_text(x+NOTIFICATION_BIG_MARGIN, y+NOTIFICATION_BIG_MARGIN, NOTIFICATION_SCALE, notification, graphics::Layer::AboveBillboard, colors::BASE03);
        y += height+NOTIFICATION_BIG_MARGIN*2.0;
    }

    notifications.0.retain(|&(_, count)| count > 0)
}
