extern crate sdl2;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::Canvas;
use sdl2::video::Window;

use super::cpu::Chip8;

const SCALE: u32 = 10;
const HEIGHT: u32 = 32 * SCALE;
const WIDTH: u32 = 64 * SCALE;

pub fn emulate(title: &str, chip8: &mut Chip8) {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let window = video_subsystem
        .window(title, WIDTH, HEIGHT)
        .position_centered()
        .opengl()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().present_vsync().build().unwrap();
    canvas.clear();
    canvas.present();

    let mut event_pump = sdl_context.event_pump().unwrap();
    let mut r = false;

    while !r {
        for evt in event_pump.poll_iter() {
            match evt {
                Event::Quit { .. } => r = true,
                Event::KeyDown {
                    keycode: Some(key), ..
                } => match key {
                    Keycode::Escape => r = true,
                    Keycode::X => chip8.keys[0] = 0xFF,
                    Keycode::Num1 => chip8.keys[1] = 0xFF,
                    Keycode::Num2 => chip8.keys[2] = 0xFF,
                    Keycode::Num3 => chip8.keys[3] = 0xFF,
                    Keycode::Q => chip8.keys[4] = 0xFF,
                    Keycode::W => chip8.keys[5] = 0xFF,
                    Keycode::E => chip8.keys[6] = 0xFF,
                    Keycode::A => chip8.keys[7] = 0xFF,
                    Keycode::S => chip8.keys[8] = 0xFF,
                    Keycode::D => chip8.keys[9] = 0xFF,
                    Keycode::Z => chip8.keys[0xA] = 0xFF,
                    Keycode::C => chip8.keys[0xB] = 0xFF,
                    Keycode::Num4 => chip8.keys[0xC] = 0xFF,
                    Keycode::R => chip8.keys[0xD] = 0xFF,
                    Keycode::F => chip8.keys[0xE] = 0xFF,
                    Keycode::V => chip8.keys[0xF] = 0xFF,
                    _ => {}
                },
                Event::KeyUp {
                    keycode: Some(key), ..
                } => match key {
                    Keycode::X => chip8.keys[0] = 0,
                    Keycode::Num1 => chip8.keys[1] = 0,
                    Keycode::Num2 => chip8.keys[2] = 0,
                    Keycode::Num3 => chip8.keys[3] = 0,
                    Keycode::Q => chip8.keys[4] = 0,
                    Keycode::W => chip8.keys[5] = 0,
                    Keycode::E => chip8.keys[6] = 0,
                    Keycode::A => chip8.keys[7] = 0,
                    Keycode::S => chip8.keys[8] = 0,
                    Keycode::D => chip8.keys[9] = 0,
                    Keycode::Z => chip8.keys[0xA] = 0,
                    Keycode::C => chip8.keys[0xB] = 0,
                    Keycode::Num4 => chip8.keys[0xC] = 0,
                    Keycode::R => chip8.keys[0xD] = 0,
                    Keycode::F => chip8.keys[0xE] = 0,
                    Keycode::V => chip8.keys[0xF] = 0,
                    _ => {}
                },
                _ => {}
            }
        }

        for _ in 0..10 {
            chip8.cycle();
        }
        draw(&chip8.display, &mut canvas);

        std::thread::sleep(std::time::Duration::from_millis(32));
    }
}

fn draw(display: &[u32; 64 * 32], canvas: &mut Canvas<Window>) {
    canvas.set_draw_color(Color::RGB(0, 0, 0));
    canvas.clear();

    canvas.set_draw_color(Color::RGB(255, 255, 255));
    for (i, &px) in display.iter().enumerate() {
        if px != 0 {
            let x = (i % 64) as i32;
            let y = (i / 64) as i32;

            let rect = Rect::new(x * SCALE as i32, y * SCALE as i32, SCALE, SCALE);
            canvas.fill_rect(rect).unwrap();
        }
    }
    canvas.present();
}
