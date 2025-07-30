use raylib::prelude::*;
use raylib::color::Color;
mod framebuffer;
use crate::framebuffer::Framebuffer;
mod line;
use line::line;

fn main() {
    let window_width = 800;
    let window_height = 600;

    let framebuffer_width = 800;
    let framebuffer_height = 600;

    let (mut window, raylib_thread) = raylib::init()
        .size(window_width, window_height)
        .title("Window Example")
        .log_level(TraceLogLevel::LOG_WARNING)
        .build();

    let mut framebuffer = Framebuffer::new(framebuffer_width, framebuffer_height, Color::new(50,50,100,255));

    framebuffer.set_background_color(Color::new(50,50,100,255));
    framebuffer.clear();

    framebuffer.set_curret_color(Color::GREEN);
    line (
        &mut framebuffer,
        Vector2::new(50.0, 50.0),
        Vector2::new(350.0, 350.0),
    );

    framebuffer.set_curret_color(Color::RED);
    line (
        &mut framebuffer,
        Vector2::new(350.0, 50.0),
        Vector2::new(50.0, 350.0),
    );

    while !window.window_should_close() {
        framebuffer.swap_buffers(&mut window, &raylib_thread)
    }
}
