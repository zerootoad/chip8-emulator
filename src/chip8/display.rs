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

fn debug(chip8: &Chip8, canvas: &mut Canvas<Window>, ttf_context: &sdl2::ttf::Sdl2TtfContext) {
    let font = ttf_context
        .load_font("src/assets/consolas.ttf", 16)
        .unwrap();

    canvas.set_draw_color(Color::RGB(50, 50, 50));
    canvas.clear();

    let opcode_text = format!("Opcode: {:04X}", chip8.opcode);
    draw_text(
        canvas,
        &font,
        &opcode_text,
        10,
        10,
        Color::RGB(255, 255, 255),
    );

    draw_text(
        canvas,
        &font,
        "Registers:",
        10,
        40,
        Color::RGB(255, 255, 255),
    );
    draw_text(canvas, &font, "Stack:", 180, 40, Color::RGB(255, 255, 255));

    let mut y = 60;
    for (i, &value) in chip8.reg.iter().enumerate() {
        let reg_text = format!("V{:X}: {:02X}", i, value);
        draw_text(canvas, &font, &reg_text, 10, y, Color::RGB(200, 200, 200));

        if i < chip8.stack.len() {
            let stack_text = format!("SP[{}]: {:04X}", i, chip8.stack[i]);
            draw_text(
                canvas,
                &font,
                &stack_text,
                180,
                y,
                Color::RGB(200, 200, 200),
            );
        }

        y += 20;
    }

    let sp_text = format!("Stack Pointer: {}", chip8.sp);
    draw_text(canvas, &font, &sp_text, 180, y, Color::RGB(255, 255, 255));

    let ireg_text = format!("I Register: {:04X}", chip8.ireg);
    draw_text(canvas, &font, &ireg_text, 10, y, Color::RGB(255, 255, 255));

    canvas.present();
}

fn draw_text(
    canvas: &mut Canvas<Window>,
    font: &sdl2::ttf::Font,
    text: &str,
    x: i32,
    y: i32,
    color: Color,
) {
    let surface = font.render(text).blended(color).unwrap();
    let texture_creator = canvas.texture_creator();
    let texture = texture_creator
        .create_texture_from_surface(&surface)
        .unwrap();
    let target = Rect::new(x, y, surface.width(), surface.height());
    canvas.copy(&texture, None, Some(target)).unwrap();
}

pub fn emulate(title: &str, delay: u64, chip8: &mut Chip8) {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let window = video_subsystem
        .window(title, WIDTH, HEIGHT)
        .position_centered()
        .opengl()
        .build()
        .unwrap();

    let dwindow = video_subsystem
        .window("Debugger", 400, 400)
        .position_centered()
        .opengl()
        .build()
        .unwrap();

    let ttf_context = sdl2::ttf::init().unwrap();

    let mut dcanvas = dwindow.into_canvas().present_vsync().build().unwrap();
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
                    Keycode::X => chip8.keys[0] = true,
                    Keycode::Num1 => chip8.keys[1] = true,
                    Keycode::Num2 => chip8.keys[2] = true,
                    Keycode::Num3 => chip8.keys[3] = true,
                    Keycode::Q => chip8.keys[4] = true,
                    Keycode::W => chip8.keys[5] = true,
                    Keycode::E => chip8.keys[6] = true,
                    Keycode::A => chip8.keys[7] = true,
                    Keycode::S => chip8.keys[8] = true,
                    Keycode::D => chip8.keys[9] = true,
                    Keycode::Z => chip8.keys[0xA] = true,
                    Keycode::C => chip8.keys[0xB] = true,
                    Keycode::Num4 => chip8.keys[0xC] = true,
                    Keycode::R => chip8.keys[0xD] = true,
                    Keycode::F => chip8.keys[0xE] = true,
                    Keycode::V => chip8.keys[0xF] = true,
                    _ => {}
                },
                Event::KeyUp {
                    keycode: Some(key), ..
                } => match key {
                    Keycode::X => chip8.keys[0] = false,
                    Keycode::Num1 => chip8.keys[1] = false,
                    Keycode::Num2 => chip8.keys[2] = false,
                    Keycode::Num3 => chip8.keys[3] = false,
                    Keycode::Q => chip8.keys[4] = false,
                    Keycode::W => chip8.keys[5] = false,
                    Keycode::E => chip8.keys[6] = false,
                    Keycode::A => chip8.keys[7] = false,
                    Keycode::S => chip8.keys[8] = false,
                    Keycode::D => chip8.keys[9] = false,
                    Keycode::Z => chip8.keys[0xA] = false,
                    Keycode::C => chip8.keys[0xB] = false,
                    Keycode::Num4 => chip8.keys[0xC] = false,
                    Keycode::R => chip8.keys[0xD] = false,
                    Keycode::F => chip8.keys[0xE] = false,
                    Keycode::V => chip8.keys[0xF] = false,
                    _ => {}
                },
                _ => {}
            }
        }

        for _ in 0..10 {
            chip8.cycle();
        }

        draw(&chip8.display, &mut canvas);
        debug(chip8, &mut dcanvas, &ttf_context);

        std::thread::sleep(std::time::Duration::from_millis(delay));
    }
}

fn draw(display: &[bool; 64 * 32], canvas: &mut Canvas<Window>) {
    canvas.set_draw_color(Color::RGB(0, 0, 0));
    canvas.clear();

    canvas.set_draw_color(Color::RGB(255, 255, 255));
    for (i, &px) in display.iter().enumerate() {
        if px {
            let x = (i % 64) as i32;
            let y = (i / 64) as i32;

            let rect = Rect::new(x * SCALE as i32, y * SCALE as i32, SCALE, SCALE);
            canvas.fill_rect(rect).unwrap();
        }
    }
    canvas.present();
}
