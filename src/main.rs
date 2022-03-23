use std::collections::HashMap;
use std::env;

use bus::Bus;
use cart::Cart;
use cpu::CPU;
use joypad::Inputs;
use rendering::{Frame, render};
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::PixelFormatEnum;

pub mod cpu;
pub mod registers;
pub mod lib;
pub mod instructions;
pub mod bus;
pub mod cart;
pub mod debug;
pub mod ppu;
pub mod joypad;
pub mod rendering;


const WINDOW_WIDTH: usize = 256;
const WINDOW_HEIGHT: usize = 240;

fn main() {
    // Get rom path from cmd line arg
    let args: Vec<String> = env::args().collect();
    let cart_path = args.get(1).expect("No ROM path provided");
    let cart = Cart::new(cart_path);
    
    // Init SLD2
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let window = video_subsystem
        .window("NES", (WINDOW_WIDTH * 2) as u32, (WINDOW_HEIGHT * 2) as u32)
        .position_centered().build().unwrap();

    let mut canvas = window.into_canvas().present_vsync().build().unwrap();
    let mut event_pump = sdl_context.event_pump().unwrap();
    canvas.set_scale(2.0, 2.0).unwrap();

    let creator = canvas.texture_creator();
    let mut texture = creator.create_texture_target(
        PixelFormatEnum::RGB24, 
        (WINDOW_WIDTH) as u32, 
        (WINDOW_HEIGHT) as u32
    ).unwrap();

    let key_map = HashMap::from([
        (Keycode::A, Inputs::A),
        (Keycode::S, Inputs::B),
        (Keycode::Space, Inputs::Select),
        (Keycode::Return, Inputs::Start),
        (Keycode::Up, Inputs::Up),
        (Keycode::Down, Inputs::Down),
        (Keycode::Left, Inputs::Left),
        (Keycode::Right, Inputs::Right),
    ]);

    let mut frame = Frame::new();
    
    let bus = Bus::new(cart, |ppu, joypad| {
        render(ppu, &mut frame);
        texture.update(None, &frame.data, WINDOW_WIDTH * 2 * 3).unwrap();

        canvas.copy(&texture, None, None).unwrap();

        canvas.present();
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => std::process::exit(0),
 
                Event::KeyDown { keycode, .. } => {
                    if let Some(key) = key_map.get(&keycode.unwrap_or(Keycode::Ampersand)) {
                        joypad.set_button_pressed_status(*key, true);
                    }
                }

                Event::KeyUp { keycode, .. } => {
                    if let Some(key) = key_map.get(&keycode.unwrap_or(Keycode::Ampersand)) {
                        joypad.set_button_pressed_status(*key, false);
                    }
                }
                _ => { /* do nothing */ }
            }
        }
    });

    let mut cpu = CPU::new(bus);
    cpu.reset();
    cpu.run(|_cpu| {
        // println!("{}", cpu.trace());
    });
}