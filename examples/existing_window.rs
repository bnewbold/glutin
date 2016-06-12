#![cfg(any(target_os = "linux", target_os = "dragonfly", target_os = "freebsd", target_os = "openbsd"))]

extern crate glutin;

use std::env;
use std::u64;
use std::process::exit;
mod support;

use glutin::os::unix::WindowBuilderExt;

fn resize_callback(width: u32, height: u32) {
    println!("Window resized to {}x{}", width, height);
}

fn usage() {
    println!("This example requires a single argument (a hex/decimal X Window ID");
    println!("You can find this with, eg, 'xwininfo -tree'");
}

fn main() {

    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        usage();
        exit(-1);
    }

    let window_id_parse = {
        if (&args[1]).starts_with("0x") {
            u64::from_str_radix(args[1].trim_left_matches('0').trim_left_matches('x'), 16)
        } else {
            args[1].parse::<u64>()
        }
    };
    let window_id = window_id_parse.expect("Failed to parse numerical arg");

    let mut window = glutin::WindowBuilder::new()
                                           .from_existing_window(window_id)
                                           .build()
                                           .unwrap();
    window.set_window_resize_callback(Some(resize_callback as fn(u32, u32)));
    let _ = unsafe { window.make_current() };

    println!("Pixel format of the window: {:?}", window.get_pixel_format());

    let context = support::load(&window);

    println!("Mouse over target window to draw; events will be printed here");

    for event in window.wait_events() {
        context.draw_frame((0.0, 1.0, 0.0, 1.0));
        let _ = window.swap_buffers();

        println!("{:?}", event);

        match event {
            glutin::Event::Closed => break,
            _ => ()
        }
    }
}
