use std::time::Duration;

mod screen;
use screen::Screen;

mod pump;
use pump::Pump;

const WIDTH: u32 = 500;
const HEIGHT: u32 = 500;

fn draw_pixels(screen: &mut Screen, y_off: i32) {
    for y in y_off..y_off + 100 {
        for x in 100..200 {
            screen.set_pixel(x, y, 124, 2, 4);
        }
    }
}

pub fn main() {
    let sdl_context = sdl2::init().unwrap();

    let mut screen = Screen::init(&sdl_context, "rusty rays", WIDTH, HEIGHT);
    let mut pump = Pump::init(&sdl_context);

    let mut y_off = 0;
    while !pump.terminated() {
        screen.clear();
        y_off = (y_off + 2) % HEIGHT as i32;
        draw_pixels(&mut screen, y_off);

        screen.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}
