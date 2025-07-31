use raylib::prelude::*;
use raylib::color::Color;
mod framebuffer;
use crate::framebuffer::Framebuffer;
mod line;
use line::line;
use std::thread;
use std::time::Duration;

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

    let mut translate_x = 0.0;
    let mut translate_y = 0.0;

    let mut direction_x = 1.0;
    let mut direction_y = 1.0;
    let speed = 2.0;

    while !window.window_should_close() {
        if translate_x + 50.0 > framebuffer_width as f32 || translate_x < 0.0 {
            direction_x = - direction_x;
        }

        if translate_y + 50.0 > framebuffer_height as f32 || translate_y < 0.0 {
            direction_y = - direction_y;
        }

        translate_x += direction_x * speed;
        translate_y += direction_y * speed;
      

        framebuffer.clear();

        render(&mut framebuffer, translate_x, translate_y);

        // Si se presiona F12, hacer un screenshot
        if window.is_key_pressed(KeyboardKey::KEY_F12) {
            framebuffer.render_to_file("screenshot.png");
            println!("Screenshot guardado como screenshot.png");
        }



        framebuffer.swap_buffers(&mut window, &raylib_thread);

        thread::sleep(Duration::from_millis(16));

    }
}

fn render(
    framebuffer:&mut Framebuffer,
    translate_x: f32,
    translate_y: f32,
) {
    framebuffer.set_background_color(Color::GREEN);
    line(
        framebuffer,
        Vector2::new(50.0 + translate_x, 50.0 +translate_y),
        Vector2::new(350.0 + translate_x, 350.0 +translate_y),
    );

    framebuffer.set_background_color(Color::RED);
    line(
        framebuffer,
        Vector2::new(350.0 + translate_x, 50.0 +translate_y),
        Vector2::new(50.0 + translate_x, 350.0 +translate_y),
    );
}
