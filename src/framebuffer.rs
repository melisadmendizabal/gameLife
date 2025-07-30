use raylib::prelude::*;
use raylib::color::Color;

pub struct Framebuffer {
    pub width: u32,
    pub height: u32,
    pub color_buffer: Image, //guarda los pixeles
    background_color: Color, //color de fondo para limpiar
    curret_color: Color, //color actual
}

impl Framebuffer {
    pub fn new(width: u32, height: u32, background_color: Color) -> Self {
        let color_buffer = Image::gen_image_color(width as i32, height as i32, background_color);
        Framebuffer {
            width,
            height,
            color_buffer,
            background_color,
            curret_color: Color::WHITE,
        }
    }

    pub fn clear(&mut self) { //limpia su buffer de colores
        self.color_buffer = Image::gen_image_color(self.width as i32, self.height as i32, self.background_color); 
    }

    pub fn set_pixel(&mut self, x: u32, y: u32) {//poner un pixel que no se salga de la pantalla
        if x < self.width && y < self.height {
            Image::draw_pixel(&mut self.color_buffer, x as i32, y as i32, self.curret_color);
        }
    }

    pub fn set_background_color(&mut self, color: Color){ //settear el color de fondo
        self.background_color= color;
    }

    pub fn set_curret_color(&mut self, color: Color) {//setear el color
        self.curret_color = color;
    }

    pub fn render_to_file(&self, file_path: &str){
        Image::export_image(&self.color_buffer, file_path);
    }

    //nueva función para pintar ventanas 
    pub fn swap_buffers(
        &self,
        window: &mut RaylibHandle,
        raylib_thread: &RaylibThread,
    ) {
        if let Ok(texture) = window.load_texture_from_image(raylib_thread, &self.color_buffer) {
            //modificación para que escale 
            //Primero hay que obtner el tamaño de la pantalla actual
            let screen_width = window.get_screen_width();
            let screen_height = window.get_screen_height();

            //hay que calcular la escala sin dañar el aspecto
            let scale_x = screen_width as f32 / self.width as f32;
            let scale_y = screen_height as f32 / self.height as f32;
            let scale = scale_x.min(scale_y);

            //este será el tamaño final de framebuffer
            let scaled_width = (self.width as f32 *scale) as i32;
            let scaled_height = (self.height as f32 *scale) as i32;

            //luego se debe de buscar el centro de la ventana 
            let offset_x = (screen_width - scaled_width) /2;
            let offset_y = (screen_height - scaled_height) /2;

            //lo que ya tenía
            let mut renderer = window.begin_drawing(raylib_thread);
            renderer.clear_background(Color::BLACK);

            renderer.draw_texture_ex(
                &texture, 
                Vector2::new(offset_x as f32, offset_y as f32),
                0.0, //rotación?
                scale, //la escala
                Color::WHITE
            );
        }
    }
    
}