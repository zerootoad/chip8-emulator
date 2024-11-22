use chip8::cpu::Chip8;

pub mod chip8;

fn main() {
    /*
     _______ _ __ ___   ___
    |_  / _ \ '__/ _ \ / _ \
     / /  __/ | | (_) | (_) |
    /___\___|_|  \___/ \___/
    */

    let mut chip8 = Chip8::new();
    chip8.load_fontset();
    chip8.load_rom("src/roms/tetris.ch8");

    chip8.emulate("chip8 emulator", 32);
}
